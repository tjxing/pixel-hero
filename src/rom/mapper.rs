use super::slice::Slice;

pub trait Mapper {
    fn read_prg(&self, addr: u16) -> u8;
    fn write_prg(&mut self, addr: u16, value: u8);
    fn read_chr(&self, addr: u16) -> u8;
    fn write_chr(&mut self, addr: u16, value: u8);
}

pub fn find_mapper(mapper: u16, sub_mapper: Option<u8>,
                   prg_rom: Slice, chr_rom: Option<Slice>) -> Option<Box<dyn Mapper>> {
    if mapper == 3 {
        Some(Box::new(super::mappers::Mapper003::new(prg_rom, chr_rom.unwrap())))
    } else {
        None
    }
}