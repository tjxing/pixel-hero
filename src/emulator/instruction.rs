use super::bus::Bus;
use crate::log::console_log;

#[derive(Copy, Clone)]
pub struct Instruction {
    pub cycles: u8,
    action: fn(&mut Bus) -> u8
}

impl Instruction {
    pub fn apply(&self, bus: &mut Bus) -> u8 {
        (self.action)(bus) + self.cycles
    }
}

pub struct InstructionSet {
    instructions: [Instruction; 256]
}

impl InstructionSet {
    pub fn new() -> InstructionSet {
        let mut instructions: [Instruction; 256] = [Instruction {
            cycles: 0,
            action: |_: &mut Bus| {
                panic!("Invalid instruction")
            }
        }; 256];

        // ADC #v
        instructions[0x69] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().adc(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // ADC d
        instructions[0x65] = Instruction {
            cycles: 3,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().adc(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // ADC d,X
        instructions[0x75] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().adc(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // ADC a
        instructions[0x6D] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().adc(num);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // ADC a,X
        instructions[0x7D] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().adc(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 {1} else {0}
            }
        };
        // ADC a,Y
        instructions[0x79] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_y_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().adc(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 {1} else {0}
            }
        };
        // ADC (d,X)
        instructions[0x61] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.indexed_indirect_map();
                let num = bus.read(addr);
                bus.cpu_mut().adc(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // ADC (d),Y
        instructions[0x71] = Instruction {
            cycles: 5,
            action: |bus: &mut Bus| {
                let addr = bus.indirect_indexed_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().adc(num);
                bus.cpu_mut().go_forward(2);
                if addr.1 {1} else {0}
            }
        };
        // AND #v
        instructions[0x29] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().and(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // AND d
        instructions[0x25] = Instruction {
            cycles: 3,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().and(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // AND d,X
        instructions[0x35] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().and(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // AND a
        instructions[0x2D] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().and(num);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // AND a,X
        instructions[0x3D] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().and(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 {1} else {0}
            }
        };
        // AND a,Y
        instructions[0x39] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_y_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().and(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 {1} else {0}
            }
        };
        // AND (d,X)
        instructions[0x21] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.indexed_indirect_map();
                let num = bus.read(addr);
                bus.cpu_mut().and(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // AND (d),Y
        instructions[0x31] = Instruction {
            cycles: 5,
            action: |bus: &mut Bus| {
                let addr = bus.indirect_indexed_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().and(num);
                bus.cpu_mut().go_forward(2);
                if addr.1 {1} else {0}
            }
        };
        // ASL A
        instructions[0x0A] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().asl_set_a();
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // ASL d
        instructions[0x06] = Instruction {
            cycles: 5,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().asl(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // ASL d,X
        instructions[0x16] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().asl(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // ASL a
        instructions[0x0E] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                let result = bus.cpu_mut().asl(num);
                bus.write(addr, result);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // ASL a,X
        instructions[0x1E] = Instruction {
            cycles: 7,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                let result = bus.cpu_mut().asl(num);
                bus.write(addr.0, result);
                bus.cpu_mut().go_forward(3);
                if addr.1 {1} else {0}
            }
        };
        // BCC
        instructions[0x90] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                if bus.cpu().test_c_clear() {
                    let offset = bus.relative_map();
                    bus.cpu_mut().branch(offset)
                } else {
                    bus.cpu_mut().go_forward(2);
                    0
                }
            }
        };
        // BCS
        instructions[0xB0] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                if !bus.cpu().test_c_clear() {
                    let offset = bus.relative_map();
                    bus.cpu_mut().branch(offset)
                } else {
                    bus.cpu_mut().go_forward(2);
                    0
                }
            }
        };
        // BEQ
        instructions[0xF0] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                if bus.cpu().test_z_set() {
                    let offset = bus.relative_map();
                    bus.cpu_mut().branch(offset)
                } else {
                    bus.cpu_mut().go_forward(2);
                    0
                }
            }
        };
        // BIT d
        instructions[0x24] = Instruction {
            cycles: 3,
            action: |bus: &mut Bus| {
                let index = bus.zero_page_map();
                let num = bus.read_memory(index);
                bus.cpu_mut().test(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // BIT a
        instructions[0x2C] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let index = bus.absolute_map();
                let num = bus.read(index);
                bus.cpu_mut().test(num);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // BMI
        instructions[0x30] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                if bus.cpu().test_n_set() {
                    let offset = bus.relative_map();
                    bus.cpu_mut().branch(offset)
                } else {
                    bus.cpu_mut().go_forward(2);
                    0
                }
            }
        };
        // BNE
        instructions[0xD0] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                if !bus.cpu().test_z_set() {
                    let offset = bus.relative_map();
                    bus.cpu_mut().branch(offset)
                } else {
                    bus.cpu_mut().go_forward(2);
                    0
                }
            }
        };
        // BPL
        instructions[0x10] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                if !bus.cpu().test_n_set() {
                    let offset = bus.relative_map();
                    bus.cpu_mut().branch(offset)
                } else {
                    bus.cpu_mut().go_forward(2);
                    0
                }
            }
        };
        // BRK
        instructions[0x00] = Instruction {
            cycles: 7,
            action: |bus: &mut Bus| {
                let pc = bus.cpu().pc();
                bus.push_word(pc + 2);
                let p = bus.cpu().p();
                bus.push(p);
                let new_pc = (bus.read(0xFFFE) as u16)
                    | ((bus.read(0xFFFF) as u16) << 8);
                bus.cpu_mut().goto(new_pc);
                0
            }
        };
        // BVC
        instructions[0x50] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                if !bus.cpu().test_v_set() {
                    let offset = bus.relative_map();
                    bus.cpu_mut().branch(offset)
                } else {
                    bus.cpu_mut().go_forward(2);
                    0
                }
            }
        };
        // BVS
        instructions[0x70] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                if bus.cpu().test_v_set() {
                    let offset = bus.relative_map();
                    bus.cpu_mut().branch(offset)
                } else {
                    bus.cpu_mut().go_forward(2);
                    0
                }
            }
        };
        // CLC
        instructions[0x18] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().clc();
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // CLD
        instructions[0xD8] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().cld();
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // CLI
        instructions[0x58] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().cli();
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // CLV
        instructions[0xB8] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().clv();
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // CMP #v
        instructions[0xC9] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().cmp(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // CMP d
        instructions[0xC5] = Instruction {
            cycles: 3,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().cmp(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // CMP d,X
        instructions[0xD5] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().cmp(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // CMP a
        instructions[0xCD] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().cmp(num);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // CMP a,X
        instructions[0xDD] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().cmp(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 {1} else {0}
            }
        };
        // CMP a,Y
        instructions[0xD9] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_y_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().cmp(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 {1} else {0}
            }
        };
        // CMP (d,X)
        instructions[0xC1] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.indexed_indirect_map();
                let num = bus.read(addr);
                bus.cpu_mut().cmp(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // CMP (d),Y
        instructions[0xD1] = Instruction {
            cycles: 5,
            action: |bus: &mut Bus| {
                let addr = bus.indirect_indexed_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().cmp(num);
                bus.cpu_mut().go_forward(2);
                if addr.1 {1} else {0}
            }
        };
        // CMX #v
        instructions[0xE0] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().cpx(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // CMX d
        instructions[0xE4] = Instruction {
            cycles: 3,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().cpx(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // CMX a
        instructions[0xEC] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().cpx(num);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // CMY #v
        instructions[0xC0] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().cpy(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // CMY d
        instructions[0xC4] = Instruction {
            cycles: 3,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().cpy(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // CMY a
        instructions[0xCC] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().cpy(num);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // DEC d
        instructions[0xC6] = Instruction {
            cycles: 5,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().dec(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // DEC d,X
        instructions[0xD6] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().dec(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // DEC a
        instructions[0xCE] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                let result = bus.cpu_mut().dec(num);
                bus.write(addr, result);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // DEC a,X
        instructions[0xDE] = Instruction {
            cycles: 7,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                let result = bus.cpu_mut().dec(num);
                bus.write(addr.0, result);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // DEX
        instructions[0xCA] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().dex();
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // DEY
        instructions[0x88] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().dey();
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // EOR #v
        instructions[0x49] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().eor(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // EOR d
        instructions[0x45] = Instruction {
            cycles: 3,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().eor(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // EOR d,X
        instructions[0x55] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().eor(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // EOR a
        instructions[0x4D] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().eor(num);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // EOR a,X
        instructions[0x5D] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().eor(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 {1} else {0}
            }
        };
        // EOR a,9
        instructions[0x5D] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_y_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().eor(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 {1} else {0}
            }
        };
        // EOR (d,X)
        instructions[0x41] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.indexed_indirect_map();
                let num = bus.read(addr);
                bus.cpu_mut().eor(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // EOR (d,X)
        instructions[0x51] = Instruction {
            cycles: 5,
            action: |bus: &mut Bus| {
                let addr = bus.indirect_indexed_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().eor(num);
                bus.cpu_mut().go_forward(2);
                if addr.1 {1} else {0}
            }
        };
        // INC d
        instructions[0xE6] = Instruction {
            cycles: 5,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().inc(num);
                bus.write_memory(addr, num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // INC d,X
        instructions[0xF6] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().inc(num);
                bus.write_memory(addr, num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // INC a
        instructions[0xEE] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().inc(num);
                bus.write(addr, num);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // INC a,X
        instructions[0xFE] = Instruction {
            cycles: 7,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().inc(num);
                bus.write(addr.0, num);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // INX
        instructions[0xE8] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().inx();
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // INY
        instructions[0xC8] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().iny();
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // JMP a
        instructions[0x4C] = Instruction {
            cycles: 3,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                bus.cpu_mut().goto(addr);
                0
            }
        };
        // JMP (a)
        instructions[0x6C] = Instruction {
            cycles: 3,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let low = bus.read(addr);
                let high = bus.read(addr + 1);
                let num = (low as u16) | ((high as u16) << 8);
                bus.cpu_mut().goto(num);
                0
            }
        };
        // JSR
        instructions[0x20] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let pc = bus.cpu().pc();
                bus.push_word(pc + 2);
                let addr = bus.absolute_map();
                bus.cpu_mut().goto(addr);
                0
            }
        };
        // LDA #v
        instructions[0xA9] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().lda(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // LDA d
        instructions[0xA5] = Instruction {
            cycles: 3,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().lda(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // LDA d,X
        instructions[0xB5] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().lda(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // LDA a
        instructions[0xAD] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().lda(num);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // LDA a,X
        instructions[0xBD] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().lda(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 {1} else {0}
            }
        };
        // LDA a,Y
        instructions[0xB9] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_y_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().lda(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 {1} else {0}
            }
        };
        // LDA (d,X)
        instructions[0xA1] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.indexed_indirect_map();
                let num = bus.read(addr);
                bus.cpu_mut().lda(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // LDA (d),Y
        instructions[0xB1] = Instruction {
            cycles: 5,
            action: |bus: &mut Bus| {
                let addr = bus.indirect_indexed_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().lda(num);
                bus.cpu_mut().go_forward(2);
                if addr.1 {1} else {0}
            }
        };
        // LDX #v
        instructions[0xA2] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().ldx(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // LDX d
        instructions[0xA6] = Instruction {
            cycles: 3,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().ldx(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // LDX d,Y
        instructions[0xB6] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_y_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().ldx(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // LDX a
        instructions[0xAE] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().ldx(num);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // LDX a,Y
        instructions[0xBE] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_y_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().ldx(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 {1} else {0}
            }
        };
        // LDY #v
        instructions[0xA0] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().ldy(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // LDY d
        instructions[0xA4] = Instruction {
            cycles: 3,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().ldy(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // LDY d,X
        instructions[0xB4] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().ldx(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // LDY a
        instructions[0xAC] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().ldy(num);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // LDY a,X
        instructions[0xBC] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().ldy(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 {1} else {0}
            }
        };
        // LSR A
        instructions[0x4A] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().lsr_a();
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // LSR d
        instructions[0x46] = Instruction {
            cycles: 5,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().lsr(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // LSR d,X
        instructions[0x56] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().lsr(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // LSR a
        instructions[0x4E] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                let result = bus.cpu_mut().lsr(num);
                bus.write(addr, result);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // LSR a,X
        instructions[0x5E] = Instruction {
            cycles: 7,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                let result = bus.cpu_mut().lsr(num);
                bus.write(addr.0, result);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // NOP
        instructions[0xEA] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // ORA #v
        instructions[0x09] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().or(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // ORA d
        instructions[0x05] = Instruction {
            cycles: 3,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().or(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // ORA d,X
        instructions[0x15] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().or(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // ORA a
        instructions[0x0D] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().or(num);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // ORA a,X
        instructions[0x1D] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().or(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 {1} else {0}
            }
        };
        // ORA a,Y
        instructions[0x19] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_y_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().or(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 {1} else {0}
            }
        };
        // ORA (d,X)
        instructions[0x01] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.indexed_indirect_map();
                let num = bus.read(addr);
                bus.cpu_mut().or(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // ORA (d),Y
        instructions[0x11] = Instruction {
            cycles: 5,
            action: |bus: &mut Bus| {
                let addr = bus.indirect_indexed_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().or(num);
                bus.cpu_mut().go_forward(2);
                if addr.1 {1} else {0}
            }
        };
        // PHA
        instructions[0x48] = Instruction {
            cycles: 3,
            action: |bus: &mut Bus| {
                let a = bus.cpu().a();
                bus.push(a);
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // PHP
        instructions[0x08] = Instruction {
            cycles: 3,
            action: |bus: &mut Bus| {
                let p = bus.cpu().p();
                bus.push(p);
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // PLA
        instructions[0x68] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let a = bus.pop();
                bus.cpu_mut().set_a(a);
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // PLP
        instructions[0x28] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let p = bus.pop();
                bus.cpu_mut().set_p(p);
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // ROL A
        instructions[0x2A] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().rol_a();
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // ROL d
        instructions[0x26] = Instruction {
            cycles: 5,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().rol(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // ROL d,X
        instructions[0x36] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().rol(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // ROL a
        instructions[0x2E] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                let result = bus.cpu_mut().rol(num);
                bus.write(addr, result);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // ROL a,X
        instructions[0x3E] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                let result = bus.cpu_mut().rol(num);
                bus.write(addr.0, result);
                bus.cpu_mut().go_forward(3);
                if addr.1 {1} else {0}
            }
        };
        // ROR A
        instructions[0x6A] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().ror_a();
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // ROR d
        instructions[0x66] = Instruction {
            cycles: 5,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().ror(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // ROR d,X
        instructions[0x76] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().ror(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // ROR a
        instructions[0x6E] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                let result = bus.cpu_mut().ror(num);
                bus.write(addr, result);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // ROR a,X
        instructions[0x7E] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                let result = bus.cpu_mut().ror(num);
                bus.write(addr.0, result);
                bus.cpu_mut().go_forward(3);
                if addr.1 {1} else {0}
            }
        };
        // RTI
        instructions[0x40] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let p = bus.pop();
                bus.cpu_mut().set_p(p);
                let pc = bus.pop_word();
                bus.cpu_mut().goto(pc);
                0
            }
        };
        // RTS
        instructions[0x60] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let pc = bus.pop_word();
                bus.cpu_mut().goto(pc + 1);
                0
            }
        };
        // SBC #v
        instructions[0xE9] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().sbc(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // SBC d
        instructions[0xE5] = Instruction {
            cycles: 3,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().sbc(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // SBC d,X
        instructions[0xF5] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().sbc(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // SBC a
        instructions[0xED] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().sbc(num);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // SBC a,X
        instructions[0xFD] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().sbc(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 {1} else {0}
            }
        };
        // SBC a,Y
        instructions[0xF9] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_y_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().sbc(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 {1} else {0}
            }
        };
        // SBC (d,X)
        instructions[0xE1] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.indexed_indirect_map();
                let num = bus.read(addr);
                bus.cpu_mut().sbc(num);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // SBC (d),Y
        instructions[0xE1] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.indirect_indexed_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().sbc(num);
                bus.cpu_mut().go_forward(2);
                if addr.1 {1} else {0}
            }
        };
        // SEC
        instructions[0x38] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().sec();
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // SED
        instructions[0xF8] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().sed();
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // SEI
        instructions[0x78] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().sei();
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // STA d
        instructions[0x85] = Instruction {
            cycles: 3,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let a = bus.cpu().a();
                bus.write_memory(addr, a);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // STA d,X
        instructions[0x95] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let a = bus.cpu().a();
                bus.write_memory(addr, a);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // STA a
        instructions[0x8D] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let a = bus.cpu().a();
                bus.write(addr, a);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // STA a,X
        instructions[0x9D] = Instruction {
            cycles: 5,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let a = bus.cpu().a();
                bus.write(addr.0, a);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // STA a,Y
        instructions[0x99] = Instruction {
            cycles: 5,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_y_map();
                let a = bus.cpu().a();
                bus.write(addr.0, a);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // STA (d,X)
        instructions[0x81] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.indexed_indirect_map();
                let a = bus.cpu().a();
                bus.write(addr, a);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // STA (d),Y
        instructions[0x91] = Instruction {
            cycles: 6,
            action: |bus: &mut Bus| {
                let addr = bus.indirect_indexed_map();
                let a = bus.cpu().a();
                bus.write(addr.0, a);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // STX d
        instructions[0x86] = Instruction {
            cycles: 3,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let x = bus.cpu().x();
                bus.write_memory(addr, x);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // STX d,Y
        instructions[0x96] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_y_map();
                let x = bus.cpu().x();
                bus.write_memory(addr, x);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // STX a
        instructions[0x8E] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let x = bus.cpu().x();
                bus.write(addr, x);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // STY d
        instructions[0x84] = Instruction {
            cycles: 3,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let y = bus.cpu().y();
                bus.write_memory(addr, y);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // STY d,X
        instructions[0x94] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let y = bus.cpu().y();
                bus.write_memory(addr, y);
                bus.cpu_mut().go_forward(2);
                0
            }
        };
        // STX a
        instructions[0x8C] = Instruction {
            cycles: 4,
            action: |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let y = bus.cpu().y();
                bus.write(addr, y);
                bus.cpu_mut().go_forward(3);
                0
            }
        };
        // TAX
        instructions[0xAA] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().tax();
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // TAY
        instructions[0xA8] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().tay();
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // TYA
        instructions[0x98] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().tya();
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // TSX
        instructions[0xBA] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().tsx();
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // TXA
        instructions[0x8A] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().txa();
                bus.cpu_mut().go_forward(1);
                0
            }
        };
        // TXS
        instructions[0x9A] = Instruction {
            cycles: 2,
            action: |bus: &mut Bus| {
                bus.cpu_mut().txs();
                bus.cpu_mut().go_forward(1);
                0
            }
        };

        InstructionSet {
            instructions
        }
    }

    pub fn find_instruction(&self, code: u8) -> &Instruction {
        &self.instructions[code as usize]
    }
}