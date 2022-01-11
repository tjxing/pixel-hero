pub struct Palette {
    data: [u8; 32]
}

impl Palette {
    pub fn new() -> Self {
        Palette {
            data: [0; 32]
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.data[(addr & 0x1F) as usize]
    }

    pub fn write(&mut self, addr: u16, v: u8) {
        let index = addr & 0x1F;
        if index % 4 == 0 {
            let i = index / 4 % 4;
            self.data[(4 * i) as usize] = v;
            self.data[(4 * i) as usize + 16] = v;
        } else {
            self.data[index as usize] = v;
        }
    }
}