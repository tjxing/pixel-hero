const SIZE: usize = 32;

pub struct Palette {
    data: [u8; SIZE]
}

impl Palette {
    pub fn new() -> Self {
        Self {
            data: [0; SIZE]
        }
    }

    pub fn read(&self, addr: u16, grey: bool) -> u8 {
        let result = self.data[(addr & 0x1F) as usize];
        if grey {
            result & 0x30
        } else {
            result
        }
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