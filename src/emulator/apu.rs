pub struct APU {

}

impl APU {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn read_status(&self) -> u8 {
        0
    }

    pub fn write_register(&mut self, _addr: u16, _v: u8) {

    }
}