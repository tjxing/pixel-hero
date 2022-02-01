pub struct Controller {

}

impl Controller {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read_joy(&self, _index: u8) -> u8 {
        0
    }

    pub fn write_joy_strode(&mut self, _v: u8) {

    }
}