pub struct Registers {
    values: [u8; 32]
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            values: [0; 32]
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.values[(addr & 0xFF) as usize]
    }

    pub fn write(&mut self, addr:u16, v: u8) {
        self.values[(addr & 0xFF) as usize] = v;
    }
}