use crate::encoding::read_exact_at;
use crate::model::{
    Checksum, Checksum8, Checksum16, DEFAULT_LENGTH, Endian, MemoryLayoutType, Mirror, Nibble,
    NvramMap, Platform,
};
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
    offset: u64,
) -> io::Result<Option<ChecksumMismatch<u8>>> {
    let start: u64 = u64::from(&checksum8.start) - offset;

    // Non-adjacent checksum (file format v0.8): `start`..=`end` describe only the
    // data bytes and the checksum is stored separately at `checksum`. Used for
    // e.g. the single-byte Credits field on Williams System 11, whose checksum
    // lives at an unrelated address.
    if let Some(checksum_address) = checksum8.checksum {
        let data_end: u64 = match &checksum8.end {
            Some(e) => u64::from(e) - offset,
            None => start + checksum8.length.unwrap_or(DEFAULT_LENGTH as u64) - 1,
        };
        let mut buff = vec![0; (data_end - start + 1) as usize];
        read_exact_at(nvram_file, start, &mut buff)?;
        let calc_sum: u8 = 0xFFu8 - buff.iter().fold(0u8, |acc, &x| acc.wrapping_add(x));

        let mut checksum_byte = [0u8; 1];
        read_exact_at(nvram_file, checksum_address - offset, &mut checksum_byte)?;
        let stored_sum = checksum_byte[0];

        return if calc_sum != stored_sum {
            Ok(Some(ChecksumMismatch {
                label: Some(checksum8.label.clone()),
                expected: stored_sum,
                calculated: calc_sum,
            }))
        } else {
            Ok(None)
        };
    }

    let end: u64 = match &checksum8.end {
        Some(e) => u64::from(e) - offset,
        None => {
            let length: u64 = checksum8.length.unwrap_or(DEFAULT_LENGTH as u64);
            start + length
        }
    };

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
            if !elements.is_multiple_of(*group_size) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Inclusive range [{start} - {end}] ({elements} elements) is not divisible by groupings {group_size}"
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
    let start = u64::from(&checksum16.start) - offset;
    let end = end(checksum16, offset)?;
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

fn end(checksum16: &Checksum16, offset: u64) -> io::Result<u64> {
    let start: u64 = (&checksum16.start).into();
    let end: u64 = if let Some(end) = &checksum16.end {
        u64::from(end) - offset
    } else if let Some(length) = &checksum16.length {
        start + length - 1 - offset
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
    if map._fileformat >= VALIDATION_FILEFORMAT {
        return validation_checksums(map)
            .filter(|cs| cs.bits == Some(16))
            .map(|cs| verify_checksum(&mut nvram_file, cs, platform))
            .filter_map(|r| r.transpose())
            .collect();
    }
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
    offset: u64,
) -> io::Result<()> {
    let start: u64 = u64::from(&checksum16.start) - offset;
    let end: u64 = end(checksum16, offset)?;
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
    if map._fileformat >= VALIDATION_FILEFORMAT {
        return validation_checksums(map)
            .filter(|cs| cs.bits == Some(16))
            .try_for_each(|cs| update_checksum(&mut nvram_file, cs, platform));
    }
    let endian = platform.endian;
    let offset = platform.offset(MemoryLayoutType::NVRam);
    map.checksum16
        .iter()
        .flatten()
        .try_for_each(|cs| update_checksum16(&mut nvram_file, cs, endian, offset))
}

/// Update all checksums, whatever their size.
pub(crate) fn update_all_checksums<T: Read + Seek + Write>(
    mut nvram_file: &mut T,
    map: &NvramMap,
    platform: &Platform,
) -> io::Result<()> {
    if map._fileformat < VALIDATION_FILEFORMAT {
        // TODO also update the checksum8 records of older maps
        return update_all_checksum16(nvram_file, map, platform);
    }
    validation_checksums(map).try_for_each(|cs| update_checksum(&mut nvram_file, cs, platform))
}

/// File format from which checksums are described by `_metadata.validation`;
/// `checksum8` and `checksum16` are then ignored.
pub(crate) const VALIDATION_FILEFORMAT: f64 = 0.9;

/// A `_metadata.validation.checksum` record resolved to file positions.
struct ChecksumLocation {
    data_start: u64,
    data_length: u64,
    checksum_position: u64,
    bits: u8,
}

fn checksum_location(checksum: &Checksum, offset: u64) -> io::Result<ChecksumLocation> {
    let invalid = |msg: String| io::Error::new(io::ErrorKind::InvalidData, msg);
    let label = checksum.label.as_deref().unwrap_or("(no label)");
    let bits = checksum.bits.unwrap_or(8);
    let width = match bits {
        4 | 8 => 1,
        16 => 2,
        _ => {
            return Err(invalid(format!(
                "Checksum {label}: unsupported bits {bits}"
            )));
        }
    };
    let start = u64::from(&checksum.start);
    let length = match (&checksum.end, &checksum.length) {
        (Some(end), _) => u64::from(end) + 1 - start,
        (None, Some(length)) => u64::from(length),
        (None, None) => DEFAULT_LENGTH as u64,
    };
    let data_start = start - offset;
    let location = match &checksum.checksum {
        Some(address) => ChecksumLocation {
            data_start,
            data_length: length,
            checksum_position: u64::from(address) - offset,
            bits,
        },
        None if length > width => ChecksumLocation {
            data_start,
            data_length: length - width,
            checksum_position: data_start + length - width,
            bits,
        },
        None => {
            return Err(invalid(format!(
                "Checksum {label}: length {length} leaves no room for the checksum"
            )));
        }
    };
    Ok(location)
}

fn nibble_value(byte: u8, nibble: Nibble) -> u8 {
    match nibble {
        Nibble::High => byte >> 4,
        Nibble::Low | Nibble::Both => byte & 0x0F,
    }
}

/// Returns the (stored, calculated) checksum
fn read_checksum<T: Read + Seek>(
    nvram_file: &mut T,
    checksum: &Checksum,
    platform: &Platform,
) -> io::Result<(u16, u16)> {
    let location = checksum_location(checksum, platform.offset(MemoryLayoutType::NVRam))?;
    let nibble = platform.nibble(MemoryLayoutType::NVRam);
    let mut data = vec![0; location.data_length as usize];
    read_exact_at(nvram_file, location.data_start, &mut data)?;
    let sum = data.iter().fold(0u16, |acc, &b| {
        let value = if location.bits == 4 {
            nibble_value(b, nibble)
        } else {
            b
        };
        acc.wrapping_add(value as u16)
    });
    let mask: u16 = match location.bits {
        4 => 0x000F,
        8 => 0x00FF,
        _ => 0xFFFF,
    };
    let calculated = if checksum.complement { !sum } else { sum } & mask;

    let stored = match location.bits {
        16 => {
            let mut bytes = [0u8; 2];
            read_exact_at(nvram_file, location.checksum_position, &mut bytes)?;
            match platform.endian {
                Endian::Big => u16::from_be_bytes(bytes),
                Endian::Little => u16::from_le_bytes(bytes),
            }
        }
        bits => {
            let mut byte = [0u8; 1];
            read_exact_at(nvram_file, location.checksum_position, &mut byte)?;
            if bits == 4 {
                nibble_value(byte[0], nibble) as u16
            } else {
                byte[0] as u16
            }
        }
    };
    Ok((stored, calculated))
}

pub(crate) fn verify_checksum<T: Read + Seek>(
    nvram_file: &mut T,
    checksum: &Checksum,
    platform: &Platform,
) -> io::Result<Option<ChecksumMismatch<u16>>> {
    let (stored, calculated) = read_checksum(nvram_file, checksum, platform)?;
    if stored != calculated {
        return Ok(Some(ChecksumMismatch {
            label: checksum.label.clone(),
            expected: stored,
            calculated,
        }));
    }
    Ok(None)
}

fn update_checksum<T: Read + Seek + Write>(
    nvram_file: &mut T,
    checksum: &Checksum,
    platform: &Platform,
) -> io::Result<()> {
    let (_, calculated) = read_checksum(nvram_file, checksum, platform)?;
    let location = checksum_location(checksum, platform.offset(MemoryLayoutType::NVRam))?;
    let bytes = match location.bits {
        16 => match platform.endian {
            Endian::Big => calculated.to_be_bytes().to_vec(),
            Endian::Little => calculated.to_le_bytes().to_vec(),
        },
        4 => {
            // keep the other nibble of the byte
            let mut byte = [0u8; 1];
            read_exact_at(nvram_file, location.checksum_position, &mut byte)?;
            let value = calculated as u8;
            let byte = match platform.nibble(MemoryLayoutType::NVRam) {
                Nibble::High => (byte[0] & 0x0F) | (value << 4),
                Nibble::Low | Nibble::Both => (byte[0] & 0xF0) | value,
            };
            vec![byte]
        }
        _ => vec![calculated as u8],
    };
    nvram_file.seek(SeekFrom::Start(location.checksum_position))?;
    nvram_file.write_all(&bytes)
}

/// Whether all regions of the mirror hold the same data
pub(crate) fn verify_mirror<T: Read + Seek>(
    nvram_file: &mut T,
    mirror: &Mirror,
    platform: &Platform,
) -> io::Result<bool> {
    let offset = platform.offset(MemoryLayoutType::NVRam);
    let nibble = platform.nibble(MemoryLayoutType::NVRam);
    let start = mirror.start.as_ref().map(u64::from).unwrap_or(0);
    let length = u64::from(&mirror.length) as usize;
    let mut first: Option<Vec<u8>> = None;
    for region_offset in &mirror.offsets {
        let mut region = vec![0; length];
        read_exact_at(
            nvram_file,
            start + u64::from(region_offset) - offset,
            &mut region,
        )?;
        if nibble != Nibble::Both {
            region
                .iter_mut()
                .for_each(|b| *b = nibble_value(*b, nibble));
        }
        match &first {
            None => first = Some(region),
            Some(first) if *first != region => return Ok(false),
            Some(_) => {}
        }
    }
    Ok(true)
}

fn validation_checksums(map: &NvramMap) -> impl Iterator<Item = &Checksum> {
    map._metadata
        .validation
        .iter()
        .flat_map(|v| v.checksum.iter().flatten())
}

/// All checksum mismatches, whatever the size of the checksum.
pub(crate) fn verify_all_checksums<T: Read + Seek>(
    nvram_file: &mut T,
    map: &NvramMap,
    platform: &Platform,
) -> io::Result<Vec<ChecksumMismatch<u16>>> {
    if map._fileformat < VALIDATION_FILEFORMAT {
        let offset = platform.offset(MemoryLayoutType::NVRam);
        let mut mismatches: Vec<ChecksumMismatch<u16>> = Vec::new();
        for checksum8 in map.checksum8.iter().flatten() {
            if let Some(m) = verify_checksum8(nvram_file, checksum8, offset)? {
                mismatches.push(ChecksumMismatch {
                    label: m.label,
                    expected: m.expected as u16,
                    calculated: m.calculated as u16,
                });
            }
        }
        mismatches.extend(verify_all_checksum16(nvram_file, map, platform)?);
        return Ok(mismatches);
    }
    validation_checksums(map)
        .map(|cs| verify_checksum(nvram_file, cs, platform))
        .filter_map(|r| r.transpose())
        .collect()
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
            end: Some(HexOrInteger::Integer(2)),
            length: None,
            checksum: None,
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
            end: Some(HexOrInteger::Integer(4)),
            length: None,
            checksum: None,
            groupings: None,
            _notes: None,
        };
        let result = verify_checksum8(&mut cursor, &checksum8, 0);
        assert_eq!(None, result?);
        Ok(())
    }

    #[test]
    fn test_verify_checksum8_using_length() -> io::Result<()> {
        #[rustfmt::skip]
        let mut cursor = io::Cursor::new([
            0xAA, 0x11, 0x11, 0x11, 0x22
        ]);
        let checksum8 = Checksum8 {
            label: "test".to_string(),
            start: HexOrInteger::Integer(0),
            end: None,
            length: Some(4),
            checksum: None,
            groupings: None,
            _notes: None,
        };
        let result = verify_checksum8(&mut cursor, &checksum8, 0);
        assert_eq!(None, result?);
        Ok(())
    }

    #[test]
    fn test_verify_checksum8_non_adjacent_checksum() -> io::Result<()> {
        // data byte at 0 (0x10), unrelated bytes, checksum byte at 4 (0xEF).
        // 0x10 + 0xEF == 0xFF -> valid (mirrors diner_l4 Credits).
        #[rustfmt::skip]
        let mut cursor = io::Cursor::new([
            0x10, 0x00, 0x00, 0x00, 0xEF
        ]);
        let checksum8 = Checksum8 {
            label: "Credits".to_string(),
            start: HexOrInteger::Integer(0),
            end: None,
            length: None,
            checksum: Some(4),
            groupings: None,
            _notes: None,
        };
        assert_eq!(None, verify_checksum8(&mut cursor, &checksum8, 0)?);
        Ok(())
    }

    #[test]
    fn test_verify_checksum8_non_adjacent_checksum_mismatch() -> io::Result<()> {
        // data byte 0xFF with checksum byte 0xFF: 0xFF + 0xFF != 0xFF (blank slot)
        #[rustfmt::skip]
        let mut cursor = io::Cursor::new([
            0xFF, 0x00, 0x00, 0x00, 0xFF
        ]);
        let checksum8 = Checksum8 {
            label: "Credits".to_string(),
            start: HexOrInteger::Integer(0),
            end: None,
            length: None,
            checksum: Some(4),
            groupings: None,
            _notes: None,
        };
        assert_eq!(
            Some(ChecksumMismatch {
                label: Some("Credits".to_string()),
                expected: 0xFF,
                calculated: 0x00,
            }),
            verify_checksum8(&mut cursor, &checksum8, 0)?
        );
        Ok(())
    }

    #[test]
    fn test_verify_checksum8_nvram_base() -> io::Result<()> {
        // NVRAM mapped at a non-zero CPU address (as on Stern SAM): map
        // addresses are CPU addresses, the file starts at the NVRAM base.
        #[rustfmt::skip]
        let mut cursor = io::Cursor::new([
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x00, 0x00, 0xFE, 0xFF, 0xFF, 0xFF
        ]);
        let checksum8 = Checksum8 {
            label: "test".to_string(),
            start: HexOrInteger::Integer(0x02102008),
            end: Some(HexOrInteger::Integer(0x0210200C)),
            length: None,
            checksum: None,
            groupings: None,
            _notes: None,
        };
        assert_eq!(None, verify_checksum8(&mut cursor, &checksum8, 0x02102000)?);
        Ok(())
    }

    #[test]
    fn test_verify_checksum8_non_adjacent_checksum_nvram_base() -> io::Result<()> {
        #[rustfmt::skip]
        let mut cursor = io::Cursor::new([
            0x10, 0x00, 0x00, 0x00, 0xEF
        ]);
        let checksum8 = Checksum8 {
            label: "test".to_string(),
            start: HexOrInteger::Integer(0x1000),
            end: None,
            length: None,
            checksum: Some(0x1004),
            groupings: None,
            _notes: None,
        };
        assert_eq!(None, verify_checksum8(&mut cursor, &checksum8, 0x1000)?);
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
            end: Some(HexOrInteger::Integer(5)),
            length: None,
            checksum: None,
            groupings: Some(3),
            _notes: None,
        };
        let result = verify_checksum8(&mut cursor, &checksum8, 0);
        assert_eq!(None, result?);
        Ok(())
    }

    #[test]
    fn test_verify_checksum8_range_mismatch() -> io::Result<()> {
        let mut cursor = io::Cursor::new([0xAA, 0x11, 0xFF]);
        let checksum8 = Checksum8 {
            label: "test".to_string(),
            start: HexOrInteger::Integer(0),
            end: Some(HexOrInteger::Integer(2)),
            length: None,
            checksum: None,
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

    #[test]
    fn test_verify_all_checksums_validation() -> io::Result<()> {
        // SAM 8-bit, WPC 16-bit and Gottlieb System 80B 4-bit checksums
        for rom in ["acd_170h", "dm_lx4", "arena"] {
            let mut nvram = Nvram::open(Path::new(&format!("testdata/{rom}.nv")))?.unwrap();
            assert_eq!(
                Vec::<ChecksumMismatch<u16>>::new(),
                nvram.verify_all_checksums()?,
                "{rom}"
            );
        }
        Ok(())
    }

    #[test]
    fn test_update_all_checksums_4bit() -> io::Result<()> {
        let dir = testdir::testdir!();
        let nv_path = dir.join("arena.nv");
        std::fs::copy("testdata/arena.nv", &nv_path)?;
        let mut data = std::fs::read(&nv_path)?;
        // first nibble of High Score #1 (0x18B7), its checksum is at 0x18F8
        data[0xB7] = (data[0xB7] & 0xF0) | ((data[0xB7] + 1) & 0x0F);
        data[0xF8] |= 0xF0;
        std::fs::write(&nv_path, &data)?;

        let mut nvram = Nvram::open(&nv_path)?.unwrap();
        let mismatches = nvram.verify_all_checksums()?;
        assert_eq!(1, mismatches.len());
        assert_eq!(Some("High Score #1".to_string()), mismatches[0].label);

        let mut file = OpenOptions::new().read(true).write(true).open(&nv_path)?;
        update_all_checksums(&mut file, &nvram.map, &nvram.platform)?;
        assert_eq!(
            Vec::<ChecksumMismatch<u16>>::new(),
            nvram.verify_all_checksums()?
        );
        // the high nibble is not part of the 4-bit checksum and is kept
        assert_eq!(0xF0, std::fs::read(&nv_path)?[0xF8] & 0xF0);
        Ok(())
    }
}
