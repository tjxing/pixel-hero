use crate::rom::mapper::Mapper;
use crate::rom::slice::Slice;

const K: u16 = 1024;
const BANK_SIZE: u16 = 8 * K;

pub struct Mapper003 {
    prg_rom: Slice,
    chr_rom: Slice,
    bank: u8,
    read_prg: fn(s: &Slice, addr: u16) -> u8
}

impl Mapper003 {
    pub fn new(prg_rom: Slice, chr_rom: Slice) -> Mapper003 {
        let read_prg_fn = if prg_rom.length() == 32 * K as u32 {
            read_prg_32k
        } else {
            read_prg_16k
        };
        Mapper003 {
            prg_rom,
            chr_rom,
            bank: 0,
            read_prg: read_prg_fn
        }
    }
}

impl Mapper for Mapper003 {
    fn read_prg(&self, addr: u16) -> u8 {
        (self.read_prg)(&self.prg_rom, addr)
    }

    fn write_prg(&mut self, _addr: u16, value: u8) {
        self.bank = value & 0x03;
    }

    fn read_chr(&self, addr: u16) -> u8 {
        self.chr_rom.at(self.bank as u32 * BANK_SIZE as u32 + addr as u32)
    }

    fn write_chr(&mut self, _addr: u16, _value: u8) {}
}

fn read_prg_32k(s: &Slice, addr: u16) -> u8 {
    s.at(addr as u32 - 0x8000)
}

fn read_prg_16k(s: &Slice, addr: u16) -> u8 {
    s.at((addr as u32 - 0x8000) & 0x3FFF)
}