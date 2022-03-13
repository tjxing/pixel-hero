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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_read_zero_page() {
        let mut memory = Memory::new();
        memory.values[0] = 0xF1;
        memory.values[100] = 0xF2;
        memory.values[255] = 0xF3;
        assert_eq!(memory.read_zero_page(0), 0xF1);
        assert_eq!(memory.read_zero_page(100), 0xF2);
        assert_eq!(memory.read_zero_page(255), 0xF3);
    }

    #[test]
    pub fn test_write_zero_page() {
        let mut memory = Memory::new();
        memory.write_zero_page(0, 0xF1);
        memory.write_zero_page(100, 0xF2);
        memory.write_zero_page(255, 0xF3);
        assert_eq!(memory.values[0], 0xF1);
        assert_eq!(memory.values[100], 0xF2);
        assert_eq!(memory.values[255], 0xF3);
    }

    #[test]
    pub fn test_read_stack() {
        let mut memory = Memory::new();
        memory.values[256] = 0xF1;
        memory.values[256 + 100] = 0xF2;
        memory.values[256 + 255] = 0xF3;
        assert_eq!(memory.read_stack(0), 0xF1);
        assert_eq!(memory.read_stack(100), 0xF2);
        assert_eq!(memory.read_stack(255), 0xF3);
    }

    #[test]
    pub fn test_write_stack() {
        let mut memory = Memory::new();
        memory.write_stack(0, 0xF1);
        memory.write_stack(100, 0xF2);
        memory.write_stack(255, 0xF3);
        assert_eq!(memory.values[256], 0xF1);
        assert_eq!(memory.values[256 + 100], 0xF2);
        assert_eq!(memory.values[256 + 255], 0xF3);
    }

    #[test]
    pub fn test_read() {
        let mut memory = Memory::new();
        memory.values[0] = 0xF1;
        memory.values[1024] = 0xF2;
        memory.values[MEMORY_SIZE - 1] = 0xF3;
        assert_eq!(memory.read(0), 0xF1);
        assert_eq!(memory.read(1024), 0xF2);
        assert_eq!(memory.read(MEMORY_SIZE as u16 - 1), 0xF3);
    }

    #[test]
    pub fn test_write() {
        let mut memory = Memory::new();
        memory.write(0, 0xF1);
        memory.write(1024, 0xF2);
        memory.write(MEMORY_SIZE as u16 - 1, 0xF3);
        assert_eq!(memory.values[0], 0xF1);
        assert_eq!(memory.values[1024], 0xF2);
        assert_eq!(memory.values[MEMORY_SIZE - 1], 0xF3);
    }
}