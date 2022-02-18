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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_mirror() {
        let mut palette = Palette::new();

        palette.write(0x3F00, 10);
        assert_eq!(palette.read(0x3F00, false), 10);
        assert_eq!(palette.read(0x3F10, false), 10);

        palette.write(0x3F10, 20);
        assert_eq!(palette.read(0x3F00, false), 20);
        assert_eq!(palette.read(0x3F10, false), 20);

        palette.write(0x3F04, 30);
        assert_eq!(palette.read(0x3F04, false), 30);
        assert_eq!(palette.read(0x3F14, false), 30);

        palette.write(0x3F14, 40);
        assert_eq!(palette.read(0x3F04, false), 40);
        assert_eq!(palette.read(0x3F14, false), 40);

        palette.write(0x3F08, 50);
        assert_eq!(palette.read(0x3F08, false), 50);
        assert_eq!(palette.read(0x3F18, false), 50);

        palette.write(0x3F18, 60);
        assert_eq!(palette.read(0x3F08, false), 60);
        assert_eq!(palette.read(0x3F18, false), 60);

        palette.write(0x3F0C, 70);
        assert_eq!(palette.read(0x3F0C, false), 70);
        assert_eq!(palette.read(0x3F1C, false), 70);

        palette.write(0x3F1C, 80);
        assert_eq!(palette.read(0x3F0C, false), 80);
        assert_eq!(palette.read(0x3F1C, false), 80);
    }

    #[test]
    fn test_write_read() {
        let mut palette = Palette::new();
        for i in 0..8 as u16 {
            for j in 1..4 as u16 {
                let addr = 0x3F00 + i * 4 + j;
                let color = i as u8 * 0x10 + j as u8;
                palette.write(addr, color);
                assert_eq!(palette.read(addr, false), color);
            }
        }
    }

    #[test]
    fn test_read_grey() {
        let mut palette = Palette::new();
        palette.write(0x3F01, 0xF1);
        assert_eq!(palette.read(0x3F01, true), 0x30);
    }
}