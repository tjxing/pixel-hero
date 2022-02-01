use crate::emulator::bus::Bus;

#[derive(Copy, Clone)]
pub struct Instruction {
    action: fn(&mut Bus) -> u8
}

impl Instruction {
    pub fn new(action: fn(&mut Bus) -> u8) -> Self {
        Self {
            action
        }
    }

    pub fn apply(&self, bus: &mut Bus) -> u8 {
        (self.action)(bus)
    }
}