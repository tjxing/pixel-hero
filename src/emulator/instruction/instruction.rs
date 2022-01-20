use crate::emulator::bus::Bus;

#[derive(Copy, Clone)]
pub struct Instruction {
    cycles: u8,
    action: fn(&mut Bus) -> u8
}

impl Instruction {
    pub fn new(cycles: u8, action: fn(&mut Bus) -> u8) -> Self {
        Self {
            cycles,
            action
        }
    }

    pub fn apply(&self, bus: &mut Bus) -> u8 {
        (self.action)(bus) + self.cycles
    }
}