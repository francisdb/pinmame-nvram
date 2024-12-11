use std::io;
use std::io::{Read, Seek, SeekFrom, Write};

/// Number of bytes appended to the NVRAM for dip switches
///
/// PinMAME has a maximum of 10 banks with 8 switches each
/// Somehow only 6 bytes are written to the nvram file
/// https://github.com/vpinball/pinmame/blob/f14bbc89c48d0ecb0d44d4be7a694730cfbf24e1/src/wpc/core.c#L2303-L2309
const DIP_SWITCH_BYTES: i64 = 6;

pub fn get_dip_switch<T: Read + Seek>(
    nvram_file: &mut T,
    switch_count: usize,
    number: usize,
) -> io::Result<bool> {
    validate_dip_switch_range(switch_count, number)?;
    let index = number - 1;
    let register = index / 8;
    let bit = index % 8;
    let mut buff = [0; 1];
    nvram_file.seek(SeekFrom::End(-DIP_SWITCH_BYTES + register as i64))?;
    nvram_file.read_exact(&mut buff)?;
    Ok((buff[0] & (1 << bit)) != 0)
}

pub fn set_dip_switch<T: Read + Write + Seek>(
    nvram_file: &mut T,
    switch_count: usize,
    number: usize,
    on: bool,
) -> io::Result<()> {
    validate_dip_switch_range(switch_count, number)?;
    let index = number - 1;
    let register = index / 8;
    let bit = index % 8;
    // write single byte with value
    let mut buff = [0; 1];
    nvram_file.seek(SeekFrom::End(-DIP_SWITCH_BYTES + register as i64))?;
    nvram_file.read_exact(&mut buff)?;
    if on {
        buff[0] |= 1 << bit;
    } else {
        buff[0] &= !(1 << bit);
    }
    nvram_file.seek(SeekFrom::End(-DIP_SWITCH_BYTES + register as i64))?;
    nvram_file.write_all(&buff)
}

fn validate_dip_switch_range(switch_count: usize, number: usize) -> io::Result<()> {
    const MAX_SWITCH_COUNT: i64 = DIP_SWITCH_BYTES * 8;
    if switch_count > (MAX_SWITCH_COUNT) as usize {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Switch count {} out of range, expected 0-{}",
                switch_count, MAX_SWITCH_COUNT
            ),
        ));
    }
    if number < 1 || number > switch_count {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Dip switch #{} out of range, expected 1-{}",
                number, switch_count
            ),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::io;
    use std::io::SeekFrom;

    #[test]
    fn test_dip_switches() -> io::Result<()> {
        // cursor with 8 bytes, we always have 6 dip switch bytes
        let switch_count = 32;
        let mut cursor = io::Cursor::new(vec![0; 8]);
        let switch1 = get_dip_switch(&mut cursor, switch_count, 1)?;
        let switch2 = get_dip_switch(&mut cursor, switch_count, 32)?;

        // switches are OFF
        assert_eq!(false, switch1);
        assert_eq!(false, switch2);
        set_dip_switch(&mut cursor, switch_count, 1, true)?;
        set_dip_switch(&mut cursor, switch_count, 32, true)?;

        // switches are both ON
        let switch1 = get_dip_switch(&mut cursor, switch_count, 1)?;
        let switch2 = get_dip_switch(&mut cursor, switch_count, 32)?;
        assert_eq!(true, switch1);
        assert_eq!(true, switch2);

        // the switch data should be written to the cursor
        let mut buff = [0; 6];
        cursor.seek(SeekFrom::End(-6))?;
        cursor.read_exact(&mut buff)?;
        assert_eq!([1, 0, 0, 128, 0, 0], buff);

        // other data in the cursor should not be changed
        let mut buff = [0; 2];
        cursor.seek(SeekFrom::Start(0))?;
        cursor.read_exact(&mut buff)?;
        assert_eq!([0, 0], buff);
        Ok(())
    }

    #[test]
    fn test_dip_switch_out_of_range() -> io::Result<()> {
        let mut cursor = io::Cursor::new(vec![0; 8]);
        let result = get_dip_switch(&mut cursor, 16, 17);
        assert!(matches!(
            result,
            Err(ref e) if e.kind() == io::ErrorKind::InvalidInput && e.to_string() == "Dip switch #17 out of range, expected 1-16"
        ));
        let result = set_dip_switch(&mut cursor, 16, 0, true);
        assert!(matches!(
            result,
            Err(ref e) if e.kind() == io::ErrorKind::InvalidInput && e.to_string() == "Dip switch #0 out of range, expected 1-16"
        ));
        let result = set_dip_switch(&mut cursor, 8 * 6 + 1, 1, true);
        println!("{:?}", result);
        assert!(matches!(
            result,
            Err(ref e) if e.kind() == io::ErrorKind::InvalidInput && e.to_string() == "Switch count 49 out of range, expected 0-48"
        ));
        Ok(())
    }
}
