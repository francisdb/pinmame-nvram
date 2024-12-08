use crate::model::{Endian, Nibble};
use serde_json::Number;
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};

pub(crate) enum Location {
    Continuous { start: u64, length: usize },
    Scattered { offsets: Vec<u64> },
}

pub(crate) fn de_nibble(length: usize, buff: &[u8], nibble: Nibble) -> io::Result<Vec<u8>> {
    // TODO make this more efficient
    let resulting_length = (length + 1) / 2;
    let mut result = vec![0; resulting_length];
    let mut buffer = buff.to_owned();
    if length % 2 != 0 {
        // prepend 0
        buffer = vec![0].into_iter().chain(buffer).collect();
    };
    let mut iter = buffer.into_iter();
    for b in result.iter_mut() {
        // if uneven the high byte should be 0
        let high = iter.next().unwrap();
        let low = iter.next().unwrap();
        *b = match nibble {
            Nibble::High => (high & 0xF0) | ((low & 0xF0) >> 4),
            Nibble::Low => ((high & 0x0F) << 4) | (low & 0x0F),
        };
    }
    Ok(result)
}

pub(crate) fn do_nibble(length: usize, buff: &[u8], nibble: Nibble) -> io::Result<Vec<u8>> {
    if nibble == Nibble::High && length % 2 != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Length should be even when writing the high nibble",
        ));
    }
    if nibble == Nibble::Low && length % 2 != 0 && buff[0] > 0x0F {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "When writing the low nibble for an uneven length the first nibble should be 0",
        ));
    }
    if length < buff.len() * 2 - 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Length should be at least twice the length of the buffer minus 1",
        ));
    }
    if length > buff.len() * 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Length should be at most twice the length of the buffer",
        ));
    }
    let mut result = Vec::with_capacity(length);
    for b in buff.iter() {
        match nibble {
            Nibble::High => {
                result.push(b & 0xF0);
                result.push((b & 0x0F) << 4);
            }
            Nibble::Low => {
                result.push((b & 0xF0) >> 4);
                result.push(b & 0x0F);
            }
        }
    }
    // remove the first byte if the length is uneven
    if length % 2 != 0 {
        result.remove(0);
    }
    Ok(result)
}

pub(crate) fn read_ch<A: Read + Seek>(
    stream: &mut A,
    location: u64,
    length: usize,
    mask: Option<u64>,
    char_map: &Option<String>,
    nibble: &Option<Nibble>,
) -> io::Result<String> {
    let mut buff = vec![0; length];
    read_exact_at(stream, location, &mut buff)?;

    if let Some(nibble) = nibble {
        let result = de_nibble(length, &buff, *nibble)?;
        // filter out zero bytes
        let result = result.into_iter().filter(|&b| b != 0).collect::<Vec<u8>>();
        return String::from_utf8(result.to_vec())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e));
    }

    // apply mask if set
    if let Some(mask) = mask {
        for b in buff.iter_mut() {
            *b &= mask as u8;
        }
    }
    // if char_map is set, convert the buffer to a string
    if let Some(char_map) = char_map {
        let mut result = String::new();
        for b in buff.iter() {
            result.push(char_map.chars().nth(*b as usize).unwrap_or('?'));
        }
        return Ok(result);
    }

    //String::from_utf8(buff.to_vec()).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    // TODO we probably need a better solution for this
    // See High Speed where "Custom Msg Line 3" contains invalid characters by default
    // A letter with a period is encoded as 0x80 (period) + letter
    // can we replace any non-utf8 characters with \u00C4 in the string?
    let mut result = String::new();
    for b in buff.iter() {
        if *b > 0x80 {
            // use \u escape for non-ASCII characters;
            // this is not perfect as it will cause double encoding for json
            // TODO clean this up
            result.push_str(&format!("\\u{:04X}", *b));
        } else {
            result.push(*b as char);
        }
    }
    Ok(result)
}

pub(crate) fn write_ch<A: Write + Seek>(
    stream: &mut A,
    location: u64,
    length: usize,
    value: String,
    char_map: &Option<String>,
    nibble: &Option<Nibble>,
) -> io::Result<()> {
    stream.seek(SeekFrom::Start(location))?;
    // if buffer contains non-ASCII characters, fail
    if !value.is_ascii() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("String is not ASCII: {}", value),
        ));
    }
    let mut buff = value.as_bytes().to_vec();
    if let Some(char_map) = char_map {
        for b in buff.iter_mut() {
            let idx = char_map.find(*b as char).unwrap_or(0);
            *b = idx as u8;
        }
    }
    if buff.len() > length {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("String is too long: {} > {}", buff.len(), length),
        ));
    }

    if let Some(nibble) = nibble {
        buff = do_nibble(length, &buff, *nibble)?;
    }

    stream.write_all(&buff)?;
    Ok(())
}

/// Read a binary coded decimal number from the nvram file
/// https://en.wikipedia.org/wiki/Binary-coded_decimal
///
/// # Arguments
///
/// * `nvram_file` - The file to read from
/// * `location` - The location in the file to start reading from
/// * `length` - The number of bytes to read
pub(crate) fn read_bcd<A: Read + Seek>(
    stream: &mut A,
    location: Location,
    nibble: &Option<Nibble>,
    scale: &Number,
    endian: Endian,
) -> io::Result<u64> {
    let mut buff = match location {
        Location::Continuous { start, length } => {
            let mut buff = vec![0; length];
            read_exact_at(stream, start, &mut buff)?;
            buff
        }
        Location::Scattered { offsets } => {
            let mut buff = vec![0; offsets.len()];
            for offset in offsets.iter() {
                let mut byte = [0; 1];
                read_exact_at(stream, *offset, &mut byte)?;
                buff.push(byte[0]);
            }
            buff
        }
    };

    if endian == Endian::Little {
        buff.reverse();
    }

    if let Some(nibble) = nibble {
        buff = de_nibble(buff.len(), &buff, *nibble)?;
    }

    let mut score = 0;
    for item in buff.iter() {
        score *= 100;
        score += cap_bcd(item & 0x0F) as u64;
        score += cap_bcd((item & 0xF0) >> 4) as u64 * 10;
    }
    if scale.is_u64() {
        Ok((score as f64 * scale.as_u64().unwrap() as f64) as u64)
    } else {
        Ok((score as f64 * scale.as_f64().unwrap()) as u64)
    }
}

/// Ignore nibbles 0xA to 0xF (0xF = blank on Dracula/Wild Fyre) (prefix)
pub(crate) fn cap_bcd(value: u8) -> u8 {
    if value > 9 {
        0
    } else {
        value
    }
}

pub(crate) fn write_bcd<A: Write + Seek>(
    stream: &mut A,
    location: u64,
    length: usize,
    nibble: &Option<Nibble>,
    value: u64,
) -> io::Result<()> {
    stream.seek(SeekFrom::Start(location))?;
    // the nibble function will validate the length
    let buff_len = if nibble.is_some() {
        (length + 1) / 2
    } else {
        length
    };
    let mut buff = vec![0; buff_len];
    let mut score = value;
    for b in buff.iter_mut() {
        *b = ((score % 10) + ((score / 10) << 4)) as u8;
        score /= 100;
    }

    if let Some(nibble) = nibble {
        buff = do_nibble(length, &buff, *nibble)?;
    }

    stream.write_all(&buff)?;
    Ok(())
}

pub(crate) fn read_int<T: Read + Seek>(
    nvram_file: &mut T,
    endian: Endian,
    start: u64,
    length: usize,
) -> io::Result<u64> {
    nvram_file.seek(SeekFrom::Start(start))?;
    let mut buff = vec![0; length];
    read_exact_at(nvram_file, start, &mut buff)?;
    let score = match endian {
        Endian::Big => buff
            .iter()
            .fold(0u64, |acc, &x| acc.wrapping_shl(8).wrapping_add(x as u64)),
        Endian::Little => buff
            .iter()
            .rev()
            .fold(0u64, |acc, &x| acc.wrapping_shl(8).wrapping_add(x as u64)),
    };
    Ok(score)
}

fn read_exact_at<A: Seek + Read>(stream: &mut A, offset: u64, buff: &mut [u8]) -> io::Result<()> {
    stream.seek(SeekFrom::Start(offset))?;
    match stream.read_exact(buff) {
        Ok(()) => Ok(()),
        Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            format!(
                "Unexpected EOF while reading at position {} with length {}",
                offset,
                buff.len()
            ),
        )),
        Err(e) => Err(e),
    }
}

/// A special type for a real-time clock value from a WPC game,
/// stored as a sequence of 7 bytes:
/// * two-byte year (2015 is 0x07 0xDF),
/// * month (1-12),
/// * day of month (1-31),
/// * day of the week (0-6, 0=Sunday),
/// * hour (0-23)
/// * minute (0-59).
///
pub(crate) fn read_wpc_rtc<T: Read + Seek>(
    nvram_file: &mut T,
    start: u64,
    length: usize,
) -> io::Result<String> {
    let mut buff = vec![0; length];
    read_exact_at(nvram_file, start, &mut buff)?;
    let year = (buff[0] as u16) << 8 | buff[1] as u16;
    let month = buff[2];
    let day = buff[3];
    let _dow = buff[4];
    let hour = buff[5];
    let minute = buff[6];
    // output as "YYYY-MM-DD HH:MM"
    Ok(format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        year, month, day, hour, minute
    ))
}

#[cfg(test)]
mod tests {
    use crate::encoding::*;
    use crate::model::{Endian, Nibble};
    use std::io;

    #[test]
    fn test_read_bcd() -> io::Result<()> {
        let mut cursor = io::Cursor::new(vec![0x12, 0x34, 0x56, 0x78, 0x90]);
        let location = Location::Continuous {
            start: 0,
            length: 5,
        };
        let score = read_bcd(&mut cursor, location, &None, &Number::from(1), Endian::Big)?;
        pretty_assertions::assert_eq!(score, 1_234_567_890);
        Ok(())
    }

    #[test]
    fn test_read_ch() -> io::Result<()> {
        let mut cursor = io::Cursor::new(vec![0x41, 0x42, 0x43, 0x44, 0x45]);
        let score = read_ch(&mut cursor, 0x0000, 5, None, &None, &None)?;
        pretty_assertions::assert_eq!(score, "ABCDE");
        Ok(())
    }

    #[test]
    fn test_read_ch_with_charmap() -> io::Result<()> {
        let char_map = Some("???????????ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string());
        let mut cursor = io::Cursor::new(vec![0x0B, 0x0C, 0x0D, 0x0E, 0x0F]);
        let score = read_ch(&mut cursor, 0x0000, 5, None, &char_map, &None)?;
        pretty_assertions::assert_eq!(score, "ABCDE");
        Ok(())
    }

    #[test]
    fn test_write_ch() -> io::Result<()> {
        let mut cursor = io::Cursor::new(vec![0x00, 0x00, 0x00, 0x00, 0x00]);
        write_ch(&mut cursor, 0x0000, 5, "ABCDE".to_string(), &None, &None)?;
        pretty_assertions::assert_eq!(cursor.into_inner(), vec![0x41, 0x42, 0x43, 0x44, 0x45]);
        Ok(())
    }

    #[test]
    fn test_write_ch_with_charmap() -> io::Result<()> {
        let char_map = Some("???????????ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string());
        let mut cursor = io::Cursor::new(vec![0x00, 0x00, 0x00, 0x00, 0x00]);
        write_ch(
            &mut cursor,
            0x0000,
            5,
            "ABCDE".to_string(),
            &char_map,
            &None,
        )?;
        pretty_assertions::assert_eq!(cursor.into_inner(), vec![0x0B, 0x0C, 0x0D, 0x0E, 0x0F]);
        Ok(())
    }

    #[test]
    fn test_read_ch_with_nibble() -> io::Result<()> {
        // Nibble: where the sequence 0x04 0x01 0x04 0x02 0x04 0x03
        // translates to 0x41 0x42 0x43 which is the string "ABC"
        let mut cursor = io::Cursor::new(vec![0x04, 0x01, 0x04, 0x02, 0x04, 0x03]);
        let score = read_ch(&mut cursor, 0x0000, 6, None, &None, &Some(Nibble::Low))?;
        pretty_assertions::assert_eq!(score, "ABC");
        Ok(())
    }

    #[test]
    fn test_do_nibble_even() {
        let buff = vec![0x41, 0x42, 0x43];
        let result = do_nibble(6, &buff, Nibble::Low).unwrap();
        pretty_assertions::assert_eq!(result, vec![0x04, 0x01, 0x04, 0x02, 0x04, 0x03]);
    }

    #[test]
    fn test_do_nibble_uneven() {
        let buff = vec![0x01, 0x42, 0x43];
        let result = do_nibble(5, &buff, Nibble::Low).unwrap();
        pretty_assertions::assert_eq!(result, vec![0x01, 0x04, 0x02, 0x04, 0x03]);
    }

    #[test]
    #[should_panic(
        expected = "When writing the low nibble for an uneven length the first nibble should be 0"
    )]
    fn test_do_nibble_uneven_fail_drop() {
        let buff = vec![0x11, 0x42, 0x43];
        do_nibble(5, &buff, Nibble::Low).unwrap();
    }

    #[test]
    #[should_panic(expected = "Length should be at most twice the length of the buffer")]
    fn test_do_nibble_uneven_fail_length() {
        let buff = vec![0x01, 0x42];
        do_nibble(6, &buff, Nibble::Low).unwrap();
    }

    #[test]
    #[should_panic(expected = "Length should be at least twice the length of the buffer minus 1")]
    fn test_do_nibble_uneven_fail_length2() {
        let buff = vec![0x01, 0x42, 0x43];
        do_nibble(2, &buff, Nibble::Low).unwrap();
    }

    #[test]
    fn test_de_nibble_high_even() {
        let buff = vec![0x40, 0x10, 0x40, 0x20, 0x40, 0x30];
        let result = de_nibble(6, &buff, Nibble::High).unwrap();
        pretty_assertions::assert_eq!(result, vec![0x41, 0x42, 0x43]);
    }

    #[test]
    fn test_de_nibble_high_uneven() {
        let buff = vec![0x10, 0x40, 0x20, 0x40, 0x30];
        let result = de_nibble(5, &buff, Nibble::High).unwrap();
        pretty_assertions::assert_eq!(result, vec![0x01, 0x42, 0x43]);
    }

    #[test]
    fn test_de_nibble_low_uneven() {
        let buff = vec![0x04, 0x01, 0x04, 0x02, 0x04];
        let result = de_nibble(5, &buff, Nibble::Low).unwrap();
        pretty_assertions::assert_eq!(result, vec![0x04, 0x14, 0x24]);
    }
}
