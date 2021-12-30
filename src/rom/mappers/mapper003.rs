use crate::rom::mapper::Mapper;
use crate::rom::slice::Slice;
use crate::log::console_log;

const K: u16 = 1024;
const BANK_SIZE: u16 = 8 * K;

pub struct Mapper003 {
    prg_rom: Slice,
    chr_rom: Slice,
    bank: u8
}

impl Mapper003 {
    pub fn new(prg_rom: Slice, chr_rom: Slice) -> Mapper003 {
        Mapper003 {
            prg_rom,
            chr_rom,
            bank: 0
        }
    }
}

impl Mapper for Mapper003 {
    fn read_prg(&self, addr: u16) -> u8 {
        if self.prg_rom.length() == 32 * K as u32 || addr < 0xC000 {
            self.prg_rom.at(addr as u32 - 0x8000)
        } else {
            self.prg_rom.at(addr as u32 - 0xC000)
        }
    }

    fn write_prg(&mut self, addr: u16, value: u8) {
        self.bank = value & 0x03;
    }

    fn read_chr(&self, addr: u16) -> u8 {
        self.chr_rom.at(self.bank as u32 * BANK_SIZE as u32 + addr as u32)
    }

    fn write_chr(&mut self, _addr: u16, _value: u8) {}
}