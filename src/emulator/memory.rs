const MEMORY_SIZE: usize = 2048;

pub struct Memory {
    values: [u8; MEMORY_SIZE]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            values: [0; MEMORY_SIZE]
        }
    }

    pub fn read_zero_page(&self, addr: u8) -> u8 {
        self.values[addr as usize]
    }

    pub fn write_zero_page(&mut self, addr: u8, v: u8) {
        self.values[addr as usize] = v;
    }

    pub fn read_stack(&self, addr: u8) -> u8 {
        self.values[(0x0100 | addr as u16) as usize]
    }

    pub fn write_stack(&mut self, addr: u8, v: u8) {
        self.values[(0x0100 | addr as u16) as usize] = v;
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.values[(addr & 0x07FF) as usize]
    }

    pub fn write(&mut self, addr: u16, v: u8) {
        self.values[(addr & 0x07FF) as usize] = v;
    }
}