mod model;

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub fn read() {
    // open ~/vpinball/tables/Medieval Madness (Williams 1997)/pinmame/roms/mm_109c.zip
    let mut nvram_file = File::open(
        "/home/francisdb/vpinball/tables/Medieval Madness (Williams 1997)/pinmame/nvram/mm_109c.nv",
    )
    .unwrap();
    println!("nvram_file: {:?}", nvram_file);

    // seek to first high score name
    // read high score name

    let first_initials = read_initials(&mut nvram_file, 0x1D29);
    println!("First place initials: {:?}", first_initials);
    let second_initials = read_initials(&mut nvram_file, 0x1D31);
    println!("Second place initials: {:?}", second_initials);
    let third_initials = read_initials(&mut nvram_file, 0x1D39);
    println!("Third place initials: {:?}", third_initials);
    let fourth_initials = read_initials(&mut nvram_file, 0x1D41);
    println!("Fourth place initials: {:?}", fourth_initials);
}

fn read_initials(nvram_file: &mut File, location: u64) -> String {
    nvram_file.seek(SeekFrom::Start(location)).unwrap();
    let mut buff = [0; 3];
    nvram_file.read_exact(&mut buff).unwrap();
    // TODO is utf8 the right encoding? I would expect ASCII which is a subset of UTF8
    std::str::from_utf8(&buff).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        read();
        assert_eq!(5, 5);
    }
}
