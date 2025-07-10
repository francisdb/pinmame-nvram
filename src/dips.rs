/// This module provides functions to read and write dip switches on a NVRAM file.
///
/// Note: These function will return the state of all dip switches supported by PinMAME,
/// even if the rom only uses a subset. For more specific switch access including looking up
/// metadata, you should use the NVRAM struct.
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};

/// Number of bytes appended to the NVRAM for dip switches
///
/// PinMAME has a maximum of 10 banks with 8 switches each
/// Only 6 bytes are written to the nvram file
/// https://github.com/vpinball/pinmame/blob/f14bbc89c48d0ecb0d44d4be7a694730cfbf24e1/src/wpc/core.c#L2303-L2309
const DIP_SWITCH_BYTES: usize = 6;

/// Maximum number of dip switches that we can handle
const MAX_SWITCH_COUNT: usize = DIP_SWITCH_BYTES * 8;

#[derive(Debug, PartialEq)]
pub struct DipSwitchState {
    pub nr: usize,
    pub on: bool,
}

/// Get the state of a dip switch
///
/// # Arguments
/// * `nvram_file` - A mutable reference to a Read + Seek implementor
/// * `number` - The number of the dip switch to get, 1-based
///
/// # Returns
/// The state of the dip switch
///
/// # Errors
/// An io::Error if the dip switch number is out of range
///
pub fn get_dip_switch<T: Read + Seek>(nvram_file: &mut T, number: usize) -> io::Result<bool> {
    validate_generic_dip_switch_range(number)?;
    let index = number - 1;
    let register = index / 8;
    let bit = index % 8;
    let mut buff = [0; 1];
    let dip_location_from_end = -(DIP_SWITCH_BYTES as i64) + register as i64;
    nvram_file.seek(SeekFrom::End(dip_location_from_end))?;
    nvram_file.read_exact(&mut buff)?;
    Ok((buff[0] & (1 << bit)) != 0)
}

/// Set the state of a dip switch
///
/// # Arguments
/// * `nvram_file` - A mutable reference to a Read + Write + Seek implementor
/// * `number` - The number of the dip switch to set, 1-based
/// * `on` - The state to set the dip switch to
///
/// # Returns
/// An io::Result indicating success or failure
///
/// # Errors
/// An io::Error if the dip switch number is out of range
pub fn set_dip_switch<T: Read + Write + Seek>(
    nvram_file: &mut T,
    number: usize,
    on: bool,
) -> io::Result<()> {
    validate_generic_dip_switch_range(number)?;
    let index = number - 1;
    let register = index / 8;
    let bit = index % 8;
    // write single byte with value
    let mut buff = [0; 1];
    let dip_location_from_end = -(DIP_SWITCH_BYTES as i64) + register as i64;
    nvram_file.seek(SeekFrom::End(dip_location_from_end))?;
    nvram_file.read_exact(&mut buff)?;
    if on {
        buff[0] |= 1 << bit;
    } else {
        buff[0] &= !(1 << bit);
    }
    nvram_file.seek(SeekFrom::End(dip_location_from_end))?;
    nvram_file.write_all(&buff)
}

/// Get the state of all dip switches
///
/// Note: This function will return the state of all dip switches supported by PinMame,
/// even if the rom only uses a subset.
///
/// # Arguments
/// * `nvram_file` - A mutable reference to a Read + Seek implementor
///
/// # Returns
/// A vector of DipSwitchState structs
pub fn get_all_dip_switches<T: Read + Seek>(nvram_file: &mut T) -> io::Result<Vec<DipSwitchState>> {
    let mut dip_switches = Vec::new();
    for number in 1..=MAX_SWITCH_COUNT {
        let on = get_dip_switch(nvram_file, number)?;
        dip_switches.push(DipSwitchState { nr: number, on });
    }
    Ok(dip_switches)
}

/// Set the state of all provided dip switches
///
/// # Arguments
/// * `nvram_file` - A mutable reference to a Read + Write + Seek implementor
/// * `dip_switches` - A slice of DipSwitchState structs
///
/// # Returns
/// An io::Result indicating success or failure
pub fn set_dip_switches<T: Read + Write + Seek>(
    nvram_file: &mut T,
    dip_switches: &[DipSwitchState],
) -> io::Result<()> {
    for dip_switch in dip_switches {
        set_dip_switch(nvram_file, dip_switch.nr, dip_switch.on)?;
    }
    Ok(())
}

pub(crate) fn validate_generic_dip_switch_range(number: usize) -> io::Result<()> {
    if !(1..=MAX_SWITCH_COUNT).contains(&number) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Dip switch #{number} out of range, expected 1-{MAX_SWITCH_COUNT}"
            ),
        ));
    }
    Ok(())
}

pub(crate) fn validate_dip_switch_range(switch_count: usize, number: usize) -> io::Result<()> {
    if switch_count > MAX_SWITCH_COUNT {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Switch count {switch_count} out of range, expected 0-{MAX_SWITCH_COUNT}"
            ),
        ));
    }
    if number < 1 || number > switch_count {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Dip switch #{number} out of range, expected 1-{switch_count}"
            ),
        ));
    }
    validate_generic_dip_switch_range(number)?;
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
        let mut cursor = io::Cursor::new(vec![0; switch_count]);
        let switch1 = get_dip_switch(&mut cursor, 1)?;
        let switch2 = get_dip_switch(&mut cursor, 32)?;

        assert_eq!(false, switch1);
        assert_eq!(false, switch2);

        set_dip_switch(&mut cursor, 1, true)?;
        set_dip_switch(&mut cursor, 32, true)?;
        let switch1 = get_dip_switch(&mut cursor, 1)?;
        let switch2 = get_dip_switch(&mut cursor, 32)?;

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
        let result = validate_dip_switch_range(16, 17);
        assert!(matches!(
            result,
            Err(ref e) if e.kind() == io::ErrorKind::InvalidInput && e.to_string() == "Dip switch #17 out of range, expected 1-16"
        ));
        let result = validate_dip_switch_range(16, 0);
        assert!(matches!(
            result,
            Err(ref e) if e.kind() == io::ErrorKind::InvalidInput && e.to_string() == "Dip switch #0 out of range, expected 1-16"
        ));
        let result = validate_dip_switch_range(8 * 6 + 1, 1);
        println!("{result:?}");
        assert!(matches!(
            result,
            Err(ref e) if e.kind() == io::ErrorKind::InvalidInput && e.to_string() == "Switch count 49 out of range, expected 0-48"
        ));
        Ok(())
    }

    #[test]
    fn test_dip_switch_all() -> io::Result<()> {
        let mut cursor = io::Cursor::new(vec![0; 6]);
        let dip_switches = get_all_dip_switches(&mut cursor)?;
        assert_eq!(MAX_SWITCH_COUNT, dip_switches.len());
        for dip_switch in dip_switches {
            assert_eq!(false, dip_switch.on);
        }
        let mut written_switches = Vec::new();
        for i in (1..=MAX_SWITCH_COUNT).step_by(2) {
            written_switches.push(DipSwitchState { nr: i, on: true });
        }
        set_dip_switches(&mut cursor, &written_switches)?;
        let read_switches = get_all_dip_switches(&mut cursor)?;

        for (i, dip_switch) in read_switches.iter().enumerate() {
            if i % 2 == 0 {
                assert_eq!(true, dip_switch.on);
            } else {
                assert_eq!(false, dip_switch.on);
            }
        }
        Ok(())
    }
}
