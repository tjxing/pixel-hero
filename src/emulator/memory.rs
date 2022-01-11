use crate::log::console_log;

pub struct Memory {
    values: [u8; 2048]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            values: [0; 2048]
        }
    }

    pub fn read_zero_page(&self, addr: u8) -> u8 {
        self.values[addr as usize]
    }

    pub fn write_zero_page(&mut self, addr: u8, v: u8) {
        self.values[addr as usize] = v;
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.values[(addr & 0x07FF) as usize]
    }

    pub fn write(&mut self, addr: u16, v: u8) {
        self.values[(addr & 0x07FF) as usize] = v;
    }
}