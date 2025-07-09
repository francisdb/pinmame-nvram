use crate::encoding::read_exact_at;
use crate::model::{Checksum8, Checksum16, Endian, MemoryLayoutType, NvramMap, Platform};
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct ChecksumMismatch<T> {
    pub label: Option<String>,
    pub expected: T,
    pub calculated: T,
}

/// checksum8: An array of memory regions protected by an 8-bit checksum.
/// The last byte of the range is set so that the low byte from the sum of all bytes in the range is 0xFF.
///
/// They introduce an optional groupings key used to treat a single descriptor as a list of equally-sized groupings.
///
/// (On WPC games, the audits are a series of 6-byte entries, each with an 8-bit checksum as the last byte.)
pub(crate) fn verify_checksum8<T: Read + Seek>(
    nvram_file: &mut T,
    checksum8: &Checksum8,
) -> io::Result<Option<ChecksumMismatch<u8>>> {
    let start: u64 = (&checksum8.start).into();
    let end: u64 = (&checksum8.end).into();

    let group_ranges: Vec<[u64; 2]> = groupings_to_ranges(start, end, &checksum8.groupings)?;

    let checksum_failures_result: io::Result<Vec<ChecksumMismatch<u8>>> = group_ranges
        .iter()
        .map(|range| verify_checksum8_range(nvram_file, checksum8, range[0], range[1]))
        .filter_map(|r| r.transpose())
        .collect();
    let checksum_failures = checksum_failures_result?;
    if checksum_failures.is_empty() {
        Ok(None)
    } else {
        // TODO we might want to report all checksum failures
        Ok(Some(checksum_failures[0].clone()))
    }
}

/// Convert the groupings into a list of ranges
/// The end is inclusive
/// THe returned groupings are also inclusive
fn groupings_to_ranges(start: u64, end: u64, groupings: &Option<u64>) -> io::Result<Vec<[u64; 2]>> {
    let ranges = match groupings {
        Some(group_size) => {
            // validate that the range is divisible by the groupings
            let elements = end - start + 1;
            if elements % group_size != 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Inclusive range [{} - {}] ({} elements) is not divisible by groupings {}",
                        start, end, elements, group_size
                    ),
                ));
            }
            let group_count = (end - start + 1) / group_size;
            (0..group_count)
                .map(|i| {
                    let group_start = start + i * group_size;
                    let group_end = group_start + group_size - 1;
                    [group_start, group_end]
                })
                .collect()
        }
        None => {
            // single range
            vec![[start, end]]
        }
    };
    Ok(ranges)
}

/// Range of bytes to verify the checksum8
/// the end is inclusive
fn verify_checksum8_range<T: Read + Seek>(
    nvram_file: &mut T,
    checksum8: &Checksum8,
    start: u64,
    end: u64,
) -> io::Result<Option<ChecksumMismatch<u8>>> {
    let length = (end - start + 1) as usize;
    let mut buff = vec![0; length];
    read_exact_at(nvram_file, start, &mut buff)?;
    let stored_sum = buff.pop().unwrap();
    let calc_sum: u8 = 0xFFu8 - buff.iter().fold(0u8, |acc, &x| acc.wrapping_add(x));
    if calc_sum != stored_sum {
        return Ok(Some(ChecksumMismatch {
            label: Some(checksum8.label.clone()),
            expected: stored_sum,
            calculated: calc_sum,
        }));
    }
    Ok(None)
}

/// checksum16: An array of memory regions protected by a 16-bit checksum. The last two bytes of
/// the range are set so that adding all other bytes in the range results in a value of 0xFFFF.
pub(crate) fn verify_checksum16<T: Read + Seek>(
    nvram_file: &mut T,
    checksum16: &Checksum16,
    endian: Endian,
    offset: u64,
) -> io::Result<Option<ChecksumMismatch<u16>>> {
    let physical_start: u64 = (&checksum16.start).into();
    let start = physical_start - offset;
    let end = end(checksum16)? - offset;
    let length = (1 + end - start) as usize;
    let mut buff = vec![0; length];
    read_exact_at(nvram_file, start, &mut buff)?;

    let stored_sum = match endian {
        Endian::Big => (buff.pop().unwrap() as u16) + ((buff.pop().unwrap() as u16) << 8),
        Endian::Little => ((buff.pop().unwrap() as u16) << 8) + buff.pop().unwrap() as u16,
    };

    // adding sum + all other bytes should result in 0xFFFF
    let calc_sum: u16 = 0xFFFFu16 - buff.iter().fold(0u16, |acc, &x| acc.wrapping_add(x as u16));
    if calc_sum != stored_sum {
        return Ok(Some(ChecksumMismatch {
            label: checksum16.label.clone(),
            expected: stored_sum,
            calculated: calc_sum,
        }));
    }
    Ok(None)
}

fn end(checksum16: &Checksum16) -> io::Result<u64> {
    let start: u64 = (&checksum16.start).into();
    let end: u64 = if let Some(end) = &checksum16.end {
        end.into()
    } else if let Some(length) = &checksum16.length {
        start + length - 1
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Checksum16 must have either end or length",
        ));
    };
    Ok(end)
}

pub(crate) fn verify_all_checksum16<T: Read + Seek>(
    mut nvram_file: &mut T,
    map: &NvramMap,
    platform: &Platform,
) -> io::Result<Vec<ChecksumMismatch<u16>>> {
    let endian = platform.endian;
    let offset = platform.offset(MemoryLayoutType::NVRam);
    map.checksum16
        .iter()
        .flatten()
        .map(|cs| verify_checksum16(&mut nvram_file, cs, endian, offset))
        .filter_map(|r| r.transpose())
        .collect()
}

fn update_checksum16<T: Read + Seek + Write>(
    nvram_file: &mut T,
    checksum16: &Checksum16,
    endian: Endian,
) -> io::Result<()> {
    let start: u64 = (&checksum16.start).into();
    let end: u64 = end(checksum16)?;
    let length = (1 + end - start) as usize;

    let mut buff = vec![0; length - 2];
    read_exact_at(nvram_file, start, &mut buff)?;

    // adding sum + all other bytes should result in 0xFFFF
    let calc_sum: u16 = 0xFFFFu16 - buff.iter().fold(0u16, |acc, &x| acc.wrapping_add(x as u16));

    // push the calculated sum to the end of the buffer
    match endian {
        Endian::Big => {
            buff.push((calc_sum >> 8) as u8);
            buff.push((calc_sum & 0xFF) as u8);
        }
        Endian::Little => {
            buff.push((calc_sum & 0xFF) as u8);
            buff.push((calc_sum >> 8) as u8);
        }
    }

    nvram_file.seek(SeekFrom::Start(start))?;
    nvram_file.write_all(&buff)?;

    Ok(())
}

pub(crate) fn update_all_checksum16<T: Read + Seek + Write>(
    mut nvram_file: &mut T,
    map: &NvramMap,
    platform: &Platform,
) -> io::Result<()> {
    let endian = platform.endian;
    map.checksum16
        .iter()
        .flatten()
        .try_for_each(|cs| update_checksum16(&mut nvram_file, cs, endian))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Nvram;
    use crate::checksum::ChecksumMismatch;
    use crate::model::{Checksum8, HexOrInteger};
    use pretty_assertions::assert_eq;
    use std::fs::OpenOptions;
    use std::io;
    use std::path::Path;

    #[test]
    fn test_verify_checksum8_range() -> io::Result<()> {
        let mut cursor = io::Cursor::new([0xAA, 0x11, 0x44]);
        let checksum8 = Checksum8 {
            label: "test".to_string(),
            start: HexOrInteger::Integer(0),
            end: HexOrInteger::Integer(2),
            groupings: None,
            _notes: None,
        };
        let result = verify_checksum8_range(&mut cursor, &checksum8, 0, 2);
        assert_eq!(None, result?);
        Ok(())
    }

    #[test]
    fn test_verify_checksum8() -> io::Result<()> {
        #[rustfmt::skip]
        let mut cursor = io::Cursor::new([
            0xAA, 0x11, 0x11, 0x11, 0x22
        ]);
        let checksum8 = Checksum8 {
            label: "test".to_string(),
            start: HexOrInteger::Integer(0),
            end: HexOrInteger::Integer(4),
            groupings: None,
            _notes: None,
        };
        let result = verify_checksum8(&mut cursor, &checksum8);
        assert_eq!(None, result?);
        Ok(())
    }

    #[test]
    fn test_verify_checksum8_grouped() -> io::Result<()> {
        #[rustfmt::skip]
        let mut cursor = io::Cursor::new([
            0xAA, 0x11, 0x44,
            0xFF, 0x00, 0x00
        ]);
        let checksum8 = Checksum8 {
            label: "test".to_string(),
            start: HexOrInteger::Integer(0),
            end: HexOrInteger::Integer(5),
            groupings: Some(3),
            _notes: None,
        };
        let result = verify_checksum8(&mut cursor, &checksum8);
        assert_eq!(None, result?);
        Ok(())
    }

    #[test]
    fn test_verify_checksum8_range_mismatch() -> io::Result<()> {
        let mut cursor = io::Cursor::new([0xAA, 0x11, 0xFF]);
        let checksum8 = Checksum8 {
            label: "test".to_string(),
            start: HexOrInteger::Integer(0),
            end: HexOrInteger::Integer(2),
            groupings: None,
            _notes: None,
        };
        let result = verify_checksum8_range(&mut cursor, &checksum8, 0, 2);
        assert_eq!(
            Some(ChecksumMismatch {
                label: Some("test".to_string()),
                expected: 0xFF,
                calculated: 0x44
            }),
            result?
        );
        Ok(())
    }

    #[test]
    fn test_groupings_to_ranges() -> io::Result<()> {
        let groupings = Some(3);
        let ranges = groupings_to_ranges(0, 5, &groupings)?;
        assert_eq!(vec![[0, 2], [3, 5]], ranges);
        Ok(())
    }

    #[test]
    #[should_panic = "Inclusive range [0 - 4] (5 elements) is not divisible by groupings 3"]
    fn test_groupings_to_ranges_invalid_range() {
        let groupings = Some(3);
        groupings_to_ranges(0, 4, &groupings).unwrap();
    }

    #[test]
    fn test_verify_all_checksum16() -> io::Result<()> {
        let mut file = OpenOptions::new().read(true).open("testdata/dm_lx4.nv")?;
        let nvram = Nvram::open(Path::new("testdata/dm_lx4.nv"))?.unwrap();
        let checksum_failures = verify_all_checksum16(&mut file, &nvram.map, &nvram.platform)?;
        assert_eq!(Vec::<ChecksumMismatch<u16>>::new(), checksum_failures);
        Ok(())
    }
}
