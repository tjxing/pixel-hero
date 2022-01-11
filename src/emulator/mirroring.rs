use crate::rom::Rom;
use crate::log::console_log;

pub struct Mirroring {
    read: fn(u16, memory: &[u8], rom: &Rom) -> u8,
    write: fn(u16, u8, memory: &mut [u8], rom: &mut Rom)
}

impl Mirroring {
    pub fn read(&self) -> fn(u16, memory: &[u8], rom: &Rom) -> u8 {
        self.read
    }

    pub fn write(&self) -> fn(u16, u8, memory: &mut [u8], rom: &mut Rom) {
        self.write
    }
}

pub fn select_mirroring(rom: &Rom) -> Mirroring {
    if rom.mirroring().unwrap() == 0 {
        Mirroring {
            read: horizontal_mirroring_read,
            write: horizontal_mirroring_write
        }
    } else {
        Mirroring {
            read: vertical_mirroring_read,
            write: vertical_mirroring_write
        }
    }
}

fn horizontal_mirroring_read(addr: u16, memory: &[u8], rom: &Rom) -> u8 {
    let index = addr & 0x0BFF;
    if index >= 0x0800 {
        memory[(index - 0x0400) as usize]
    } else {
        memory[index as usize]
    }
}

fn horizontal_mirroring_write(addr: u16, v: u8, memory: &mut [u8], rom: &mut Rom) {
    let index = addr & 0x0BFF;
    if index >= 0x0800 {
        memory[(index - 0x0400) as usize] = v;
    } else {
        memory[index as usize] = v;
    }
}

fn vertical_mirroring_read(addr: u16, memory: &[u8], rom: &Rom) -> u8 {
    memory[(addr & 0x07FF) as usize]
}

fn vertical_mirroring_write(addr: u16, v: u8, memory: &mut [u8], rom: &mut Rom) {
    memory[(addr & 0x07FF) as usize] = v;
}