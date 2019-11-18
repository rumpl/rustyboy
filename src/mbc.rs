use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub struct MBC {
    rom: Vec<u8>,
}

impl MBC {
    pub fn new(data: Vec<u8>) -> MBC {
        MBC { rom: data }
    }

    pub fn read(&self, idx: u16) -> u8 {
        println!("{}", idx);
        self.rom[idx as usize]
    }
}

pub fn get_rom(path: &str) -> io::Result<MBC> {
    let mut data = vec![];
    let mut file = File::open(path)?;
    file.read_to_end(&mut data)?;
    Ok(MBC::new(data))
}
