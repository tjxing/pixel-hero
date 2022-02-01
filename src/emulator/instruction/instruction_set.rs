use crate::emulator::bus::Bus;
use super::instruction::Instruction;

pub struct InstructionSet {
    instructions: [Instruction; 256]
}

impl InstructionSet {
    pub fn new() -> InstructionSet {
        let mut instructions = [Instruction::new(default_action); 256];

        // ADC #v
        instructions[0x69] = Instruction::new(
            |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().adc(num);
                bus.cpu_mut().go_forward(2);
                2
            }
        );
        // ADC d
        instructions[0x65] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().adc(num);
                bus.cpu_mut().go_forward(2);
                3
            }
        );
        // ADC d,X
        instructions[0x75] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().adc(num);
                bus.cpu_mut().go_forward(2);
                4
            }
        );
        // ADC a
        instructions[0x6D] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().adc(num);
                bus.cpu_mut().go_forward(3);
                4
            }
        );
        // ADC a,X
        instructions[0x7D] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().adc(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 5 } else { 4 }
            }
        );
        // ADC a,Y
        instructions[0x79] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_y_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().adc(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 5 } else { 4 }
            }
        );
        // ADC (d,X)
        instructions[0x61] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.indexed_indirect_map();
                let num = bus.read(addr);
                bus.cpu_mut().adc(num);
                bus.cpu_mut().go_forward(2);
                6
            }
        );
        // ADC (d),Y
        instructions[0x71] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.indirect_indexed_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().adc(num);
                bus.cpu_mut().go_forward(2);
                if addr.1 { 6 } else { 5 }
            }
        );
        // AND #v
        instructions[0x29] = Instruction::new(
            |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().and(num);
                bus.cpu_mut().go_forward(2);
                2
            }
        );
        // AND d
        instructions[0x25] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().and(num);
                bus.cpu_mut().go_forward(2);
                3
            }
        );
        // AND d,X
        instructions[0x35] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().and(num);
                bus.cpu_mut().go_forward(2);
                4
            }
        );
        // AND a
        instructions[0x2D] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().and(num);
                bus.cpu_mut().go_forward(3);
                4
            }
        );
        // AND a,X
        instructions[0x3D] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().and(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 5 } else { 4 }
            }
        );
        // AND a,Y
        instructions[0x39] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_y_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().and(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 5 } else { 4 }
            }
        );
        // AND (d,X)
        instructions[0x21] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.indexed_indirect_map();
                let num = bus.read(addr);
                bus.cpu_mut().and(num);
                bus.cpu_mut().go_forward(2);
                6
            }
        );
        // AND (d),Y
        instructions[0x31] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.indirect_indexed_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().and(num);
                bus.cpu_mut().go_forward(2);
                if addr.1 { 6 } else { 5 }
            }
        );
        // ASL A
        instructions[0x0A] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().asl_set_a();
                bus.cpu_mut().go_forward(1);
                2
            }
        );
        // ASL d
        instructions[0x06] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().asl(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                5
            }
        );
        // ASL d,X
        instructions[0x16] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().asl(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                6
            }
        );
        // ASL a
        instructions[0x0E] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                let result = bus.cpu_mut().asl(num);
                bus.write(addr, result);
                bus.cpu_mut().go_forward(3);
                6
            }
        );
        // ASL a,X
        instructions[0x1E] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                let result = bus.cpu_mut().asl(num);
                bus.write(addr.0, result);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 8 } else { 7 }
            }
        );
        // BCC
        instructions[0x90] = Instruction::new(
            |bus: &mut Bus| {
                if bus.cpu().test_c_clear() {
                    let offset = bus.relative_map();
                    2 + bus.cpu_mut().branch(offset)
                } else {
                    bus.cpu_mut().go_forward(2);
                    2
                }
            }
        );
        // BCS
        instructions[0xB0] = Instruction::new(
            |bus: &mut Bus| {
                if !bus.cpu().test_c_clear() {
                    let offset = bus.relative_map();
                    2 + bus.cpu_mut().branch(offset)
                } else {
                    bus.cpu_mut().go_forward(2);
                    2
                }
            }
        );
        // BEQ
        instructions[0xF0] = Instruction::new(
            |bus: &mut Bus| {
                if bus.cpu().test_z_set() {
                    let offset = bus.relative_map();
                    2 + bus.cpu_mut().branch(offset)
                } else {
                    bus.cpu_mut().go_forward(2);
                    2
                }
            }
        );
        // BIT d
        instructions[0x24] = Instruction::new(
            |bus: &mut Bus| {
                let index = bus.zero_page_map();
                let num = bus.read_memory(index);
                bus.cpu_mut().test(num);
                bus.cpu_mut().go_forward(2);
                3
            }
        );
        // BIT a
        instructions[0x2C] = Instruction::new(
            |bus: &mut Bus| {
                let index = bus.absolute_map();
                let num = bus.read(index);
                bus.cpu_mut().test(num);
                bus.cpu_mut().go_forward(3);
                4
            }
        );
        // BMI
        instructions[0x30] = Instruction::new(
            |bus: &mut Bus| {
                if bus.cpu().test_n_set() {
                    let offset = bus.relative_map();
                    2 + bus.cpu_mut().branch(offset)
                } else {
                    bus.cpu_mut().go_forward(2);
                    2
                }
            }
        );
        // BNE
        instructions[0xD0] = Instruction::new(
            |bus: &mut Bus| {
                if !bus.cpu().test_z_set() {
                    let offset = bus.relative_map();
                    2 + bus.cpu_mut().branch(offset)
                } else {
                    bus.cpu_mut().go_forward(2);
                    2
                }
            }
        );
        // BPL
        instructions[0x10] = Instruction::new(
            |bus: &mut Bus| {
                if !bus.cpu().test_n_set() {
                    let offset = bus.relative_map();
                    2 + bus.cpu_mut().branch(offset)
                } else {
                    bus.cpu_mut().go_forward(2);
                    2
                }
            }
        );
        // BRK
        instructions[0x00] = Instruction::new(
            |bus: &mut Bus| {
                bus.trigger_brk();
                bus.cpu_mut().go_forward(2);
                7
            }
        );
        // BVC
        instructions[0x50] = Instruction::new(
            |bus: &mut Bus| {
                if !bus.cpu().test_v_set() {
                    let offset = bus.relative_map();
                    bus.cpu_mut().branch(offset)
                } else {
                    bus.cpu_mut().go_forward(2);
                    2
                }
            }
        );
        // BVS
        instructions[0x70] = Instruction::new(
            |bus: &mut Bus| {
                if bus.cpu().test_v_set() {
                    let offset = bus.relative_map();
                    2 + bus.cpu_mut().branch(offset)
                } else {
                    bus.cpu_mut().go_forward(2);
                    2
                }
            }
        );
        // CLC
        instructions[0x18] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().clc();
                bus.cpu_mut().go_forward(1);
                2
            }
        );
        // CLD
        instructions[0xD8] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().cld();
                bus.cpu_mut().go_forward(1);
                2
            }
        );
        // CLI
        instructions[0x58] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().cli();
                bus.cpu_mut().go_forward(1);
                2
            }
        );
        // CLV
        instructions[0xB8] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().clv();
                bus.cpu_mut().go_forward(1);
                2
            }
        );
        // CMP #v
        instructions[0xC9] = Instruction::new(
            |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().cmp(num);
                bus.cpu_mut().go_forward(2);
                2
            }
        );
        // CMP d
        instructions[0xC5] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().cmp(num);
                bus.cpu_mut().go_forward(2);
                3
            }
        );
        // CMP d,X
        instructions[0xD5] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().cmp(num);
                bus.cpu_mut().go_forward(2);
                4
            }
        );
        // CMP a
        instructions[0xCD] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().cmp(num);
                bus.cpu_mut().go_forward(3);
                4
            }
        );
        // CMP a,X
        instructions[0xDD] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().cmp(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 5 } else { 4 }
            }
        );
        // CMP a,Y
        instructions[0xD9] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_y_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().cmp(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 5 } else { 4 }
            }
        );
        // CMP (d,X)
        instructions[0xC1] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.indexed_indirect_map();
                let num = bus.read(addr);
                bus.cpu_mut().cmp(num);
                bus.cpu_mut().go_forward(2);
                6
            }
        );
        // CMP (d),Y
        instructions[0xD1] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.indirect_indexed_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().cmp(num);
                bus.cpu_mut().go_forward(2);
                if addr.1 { 6 } else { 5 }
            }
        );
        // CMX #v
        instructions[0xE0] = Instruction::new(
            |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().cpx(num);
                bus.cpu_mut().go_forward(2);
                2
            }
        );
        // CMX d
        instructions[0xE4] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().cpx(num);
                bus.cpu_mut().go_forward(2);
                3
            }
        );
        // CMX a
        instructions[0xEC] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().cpx(num);
                bus.cpu_mut().go_forward(3);
                4
            }
        );
        // CMY #v
        instructions[0xC0] = Instruction::new(
            |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().cpy(num);
                bus.cpu_mut().go_forward(2);
                2
            }
        );
        // CMY d
        instructions[0xC4] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().cpy(num);
                bus.cpu_mut().go_forward(2);
                3
            }
        );
        // CMY a
        instructions[0xCC] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().cpy(num);
                bus.cpu_mut().go_forward(3);
                4
            }
        );
        // DEC d
        instructions[0xC6] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().dec(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                5
            }
        );
        // DEC d,X
        instructions[0xD6] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().dec(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                6
            }
        );
        // DEC a
        instructions[0xCE] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                let result = bus.cpu_mut().dec(num);
                bus.write(addr, result);
                bus.cpu_mut().go_forward(3);
                6
            }
        );
        // DEC a,X
        instructions[0xDE] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                let result = bus.cpu_mut().dec(num);
                bus.write(addr.0, result);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 7 } else { 6 }
            }
        );
        // DEX
        instructions[0xCA] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().dex();
                bus.cpu_mut().go_forward(1);
                2
            }
        );
        // DEY
        instructions[0x88] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().dey();
                bus.cpu_mut().go_forward(1);
                2
            }
        );
        // EOR #v
        instructions[0x49] = Instruction::new(
            |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().eor(num);
                bus.cpu_mut().go_forward(2);
                2
            }
        );
        // EOR d
        instructions[0x45] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().eor(num);
                bus.cpu_mut().go_forward(2);
                3
            }
        );
        // EOR d,X
        instructions[0x55] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().eor(num);
                bus.cpu_mut().go_forward(2);
                4
            }
        );
        // EOR a
        instructions[0x4D] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().eor(num);
                bus.cpu_mut().go_forward(3);
                4
            }
        );
        // EOR a,X
        instructions[0x5D] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().eor(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 5 } else { 4 }
            }
        );
        // EOR a,Y
        instructions[0x59] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_y_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().eor(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 5 } else { 4 }
            }
        );
        // EOR (d,X)
        instructions[0x41] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.indexed_indirect_map();
                let num = bus.read(addr);
                bus.cpu_mut().eor(num);
                bus.cpu_mut().go_forward(2);
                6
            }
        );
        // EOR (d,X)
        instructions[0x51] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.indirect_indexed_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().eor(num);
                bus.cpu_mut().go_forward(2);
                if addr.1 { 6 } else { 5 }
            }
        );
        // INC d
        instructions[0xE6] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().inc(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                5
            }
        );
        // INC d,X
        instructions[0xF6] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().inc(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                6
            }
        );
        // INC a
        instructions[0xEE] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                let result = bus.cpu_mut().inc(num);
                bus.write(addr, result);
                bus.cpu_mut().go_forward(3);
                6
            }
        );
        // INC a,X
        instructions[0xFE] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                let result = bus.cpu_mut().inc(num);
                bus.write(addr.0, result);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 7 } else { 6 }
            }
        );
        // INX
        instructions[0xE8] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().inx();
                bus.cpu_mut().go_forward(1);
                2
            }
        );
        // INY
        instructions[0xC8] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().iny();
                bus.cpu_mut().go_forward(1);
                2
            }
        );
        // JMP a
        instructions[0x4C] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                bus.cpu_mut().goto(addr);
                3
            }
        );
        // JMP (a)
        instructions[0x6C] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let low = bus.read(addr);
                let high = bus.read(addr + 1);
                let num = (low as u16) | ((high as u16) << 8);
                bus.cpu_mut().goto(num);
                5
            }
        );
        // JSR
        instructions[0x20] = Instruction::new(
            |bus: &mut Bus| {
                let pc = bus.cpu().pc();
                bus.push_word(pc + 2);
                let addr = bus.absolute_map();
                bus.cpu_mut().goto(addr);
                6
            }
        );
        // LDA #v
        instructions[0xA9] = Instruction::new(
            |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().lda(num);
                bus.cpu_mut().go_forward(2);
                2
            }
        );
        // LDA d
        instructions[0xA5] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().lda(num);
                bus.cpu_mut().go_forward(2);
                3
            }
        );
        // LDA d,X
        instructions[0xB5] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().lda(num);
                bus.cpu_mut().go_forward(2);
                4
            }
        );
        // LDA a
        instructions[0xAD] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().lda(num);
                bus.cpu_mut().go_forward(3);
                4
            }
        );
        // LDA a,X
        instructions[0xBD] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().lda(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 5 } else { 4 }
            }
        );
        // LDA a,Y
        instructions[0xB9] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_y_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().lda(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 5 } else { 4 }
            }
        );
        // LDA (d,X)
        instructions[0xA1] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.indexed_indirect_map();
                let num = bus.read(addr);
                bus.cpu_mut().lda(num);
                bus.cpu_mut().go_forward(2);
                6
            }
        );
        // LDA (d),Y
        instructions[0xB1] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.indirect_indexed_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().lda(num);
                bus.cpu_mut().go_forward(2);
                if addr.1 { 6 } else { 5 }
            }
        );
        // LDX #v
        instructions[0xA2] = Instruction::new(
            |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().ldx(num);
                bus.cpu_mut().go_forward(2);
                2
            }
        );
        // LDX d
        instructions[0xA6] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().ldx(num);
                bus.cpu_mut().go_forward(2);
                3
            }
        );
        // LDX d,Y
        instructions[0xB6] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_y_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().ldx(num);
                bus.cpu_mut().go_forward(2);
                4
            }
        );
        // LDX a
        instructions[0xAE] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().ldx(num);
                bus.cpu_mut().go_forward(3);
                4
            }
        );
        // LDX a,Y
        instructions[0xBE] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_y_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().ldx(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 5 } else { 4 }
            }
        );
        // LDY #v
        instructions[0xA0] = Instruction::new(
            |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().ldy(num);
                bus.cpu_mut().go_forward(2);
                2
            }
        );
        // LDY d
        instructions[0xA4] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().ldy(num);
                bus.cpu_mut().go_forward(2);
                3
            }
        );
        // LDY d,X
        instructions[0xB4] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().ldy(num);
                bus.cpu_mut().go_forward(2);
                4
            }
        );
        // LDY a
        instructions[0xAC] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().ldy(num);
                bus.cpu_mut().go_forward(3);
                4
            }
        );
        // LDY a,X
        instructions[0xBC] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().ldy(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 5 } else { 4 }
            }
        );
        // LSR A
        instructions[0x4A] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().lsr_a();
                bus.cpu_mut().go_forward(1);
                2
            }
        );
        // LSR d
        instructions[0x46] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().lsr(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                5
            }
        );
        // LSR d,X
        instructions[0x56] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().lsr(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                6
            }
        );
        // LSR a
        instructions[0x4E] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                let result = bus.cpu_mut().lsr(num);
                bus.write(addr, result);
                bus.cpu_mut().go_forward(3);
                6
            }
        );
        // LSR a,X
        instructions[0x5E] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                let result = bus.cpu_mut().lsr(num);
                bus.write(addr.0, result);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 7 } else { 6 }
            }
        );
        // NOP
        instructions[0xEA] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().go_forward(1);
                2
            }
        );
        // ORA #v
        instructions[0x09] = Instruction::new(
            |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().or(num);
                bus.cpu_mut().go_forward(2);
                2
            }
        );
        // ORA d
        instructions[0x05] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().or(num);
                bus.cpu_mut().go_forward(2);
                3
            }
        );
        // ORA d,X
        instructions[0x15] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().or(num);
                bus.cpu_mut().go_forward(2);
                4
            }
        );
        // ORA a
        instructions[0x0D] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().or(num);
                bus.cpu_mut().go_forward(3);
                4
            }
        );
        // ORA a,X
        instructions[0x1D] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().or(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 5 } else { 4 }
            }
        );
        // ORA a,Y
        instructions[0x19] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_y_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().or(num);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 5 } else { 4 }
            }
        );
        // ORA (d,X)
        instructions[0x01] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.indexed_indirect_map();
                let num = bus.read(addr);
                bus.cpu_mut().or(num);
                bus.cpu_mut().go_forward(2);
                6
            }
        );
        // ORA (d),Y
        instructions[0x11] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.indirect_indexed_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().or(num);
                bus.cpu_mut().go_forward(2);
                if addr.1 { 6 } else { 5 }
            }
        );
        // PHA
        instructions[0x48] = Instruction::new(
            |bus: &mut Bus| {
                let a = bus.cpu().a();
                bus.push(a);
                bus.cpu_mut().go_forward(1);
                3
            }
        );
        // PHP
        instructions[0x08] = Instruction::new(
            |bus: &mut Bus| {
                let p = bus.cpu().p();
                bus.push(p | 0x10);
                bus.cpu_mut().go_forward(1);
                3
            }
        );
        // PLA
        instructions[0x68] = Instruction::new(
            |bus: &mut Bus| {
                let a = bus.pop();
                bus.cpu_mut().lda(a);
                bus.cpu_mut().go_forward(1);
                4
            }
        );
        // PLP
        instructions[0x28] = Instruction::new(
            |bus: &mut Bus| {
                let p = bus.pop();
                bus.cpu_mut().set_p(p);
                bus.cpu_mut().go_forward(1);
                4
            }
        );
        // ROL A
        instructions[0x2A] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().rol_a();
                bus.cpu_mut().go_forward(1);
                2
            }
        );
        // ROL d
        instructions[0x26] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().rol(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                5
            }
        );
        // ROL d,X
        instructions[0x36] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().rol(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                6
            }
        );
        // ROL a
        instructions[0x2E] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                let result = bus.cpu_mut().rol(num);
                bus.write(addr, result);
                bus.cpu_mut().go_forward(3);
                6
            }
        );
        // ROL a,X
        instructions[0x3E] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                let result = bus.cpu_mut().rol(num);
                bus.write(addr.0, result);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 7 } else { 6 }
            }
        );
        // ROR A
        instructions[0x6A] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().ror_a();
                bus.cpu_mut().go_forward(1);
                2
            }
        );
        // ROR d
        instructions[0x66] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().ror(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                5
            }
        );
        // ROR d,X
        instructions[0x76] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                let result = bus.cpu_mut().ror(num);
                bus.write_memory(addr, result);
                bus.cpu_mut().go_forward(2);
                6
            }
        );
        // ROR a
        instructions[0x6E] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                let result = bus.cpu_mut().ror(num);
                bus.write(addr, result);
                bus.cpu_mut().go_forward(3);
                6
            }
        );
        // ROR a,X
        instructions[0x7E] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                let result = bus.cpu_mut().ror(num);
                bus.write(addr.0, result);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 7 } else { 6 }
            }
        );
        // RTI
        instructions[0x40] = Instruction::new(
            |bus: &mut Bus| {
                let p = bus.pop();
                bus.cpu_mut().set_p(p);
                let pc = bus.pop_word();
                bus.cpu_mut().goto(pc);
                6
            }
        );
        // RTS
        instructions[0x60] = Instruction::new(
            |bus: &mut Bus| {
                let pc = bus.pop_word();
                bus.cpu_mut().goto(pc + 1);
                6
            }
        );
        // SBC #v
        instructions[0xE9] = Instruction::new(
            |bus: &mut Bus| {
                let num = bus.immediate_map();
                bus.cpu_mut().adc(num ^ 0xFF);
                bus.cpu_mut().go_forward(2);
                2
            }
        );
        // SBC d
        instructions[0xE5] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().adc(num ^ 0xFF);
                bus.cpu_mut().go_forward(2);
                3
            }
        );
        // SBC d,X
        instructions[0xF5] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let num = bus.read_memory(addr);
                bus.cpu_mut().adc(num ^ 0xFF);
                bus.cpu_mut().go_forward(2);
                4
            }
        );
        // SBC a
        instructions[0xED] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let num = bus.read(addr);
                bus.cpu_mut().adc(num ^ 0xFF);
                bus.cpu_mut().go_forward(3);
                4
            }
        );
        // SBC a,X
        instructions[0xFD] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().adc(num ^ 0xFF);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 5 } else { 4 }
            }
        );
        // SBC a,Y
        instructions[0xF9] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_y_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().adc(num ^ 0xFF);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 5 } else { 4 }
            }
        );
        // SBC (d,X)
        instructions[0xE1] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.indexed_indirect_map();
                let num = bus.read(addr);
                bus.cpu_mut().adc(num ^ 0xFF);
                bus.cpu_mut().go_forward(2);
                6
            }
        );
        // SBC (d),Y
        instructions[0xF1] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.indirect_indexed_map();
                let num = bus.read(addr.0);
                bus.cpu_mut().adc(num ^ 0xFF);
                bus.cpu_mut().go_forward(2);
                if addr.1 { 7 } else { 6 }
            }
        );
        // SEC
        instructions[0x38] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().sec();
                bus.cpu_mut().go_forward(1);
                2
            }
        );
        // SED
        instructions[0xF8] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().sed();
                bus.cpu_mut().go_forward(1);
                2
            }
        );
        // SEI
        instructions[0x78] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().sei();
                bus.cpu_mut().go_forward(1);
                2
            }
        );
        // STA d
        instructions[0x85] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let a = bus.cpu().a();
                bus.write_memory(addr, a);
                bus.cpu_mut().go_forward(2);
                3
            }
        );
        // STA d,X
        instructions[0x95] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let a = bus.cpu().a();
                bus.write_memory(addr, a);
                bus.cpu_mut().go_forward(2);
                4
            }
        );
        // STA a
        instructions[0x8D] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let a = bus.cpu().a();
                bus.write(addr, a);
                bus.cpu_mut().go_forward(3);
                4
            }
        );
        // STA a,X
        instructions[0x9D] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_x_map();
                let a = bus.cpu().a();
                bus.write(addr.0, a);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 5 } else { 4 }
            }
        );
        // STA a,Y
        instructions[0x99] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_y_map();
                let a = bus.cpu().a();
                bus.write(addr.0, a);
                bus.cpu_mut().go_forward(3);
                if addr.1 { 5 } else { 4 }
            }
        );
        // STA (d,X)
        instructions[0x81] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.indexed_indirect_map();
                let a = bus.cpu().a();
                bus.write(addr, a);
                bus.cpu_mut().go_forward(2);
                6
            }
        );
        // STA (d),Y
        instructions[0x91] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.indirect_indexed_map();
                let a = bus.cpu().a();
                bus.write(addr.0, a);
                bus.cpu_mut().go_forward(2);
                if addr.1 { 7 } else { 6 }
            }
        );
        // STX d
        instructions[0x86] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let x = bus.cpu().x();
                bus.write_memory(addr, x);
                bus.cpu_mut().go_forward(2);
                3
            }
        );
        // STX d,Y
        instructions[0x96] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_y_map();
                let x = bus.cpu().x();
                bus.write_memory(addr, x);
                bus.cpu_mut().go_forward(2);
                4
            }
        );
        // STX a
        instructions[0x8E] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let x = bus.cpu().x();
                bus.write(addr, x);
                bus.cpu_mut().go_forward(3);
                4
            }
        );
        // STY d
        instructions[0x84] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_map();
                let y = bus.cpu().y();
                bus.write_memory(addr, y);
                bus.cpu_mut().go_forward(2);
                3
            }
        );
        // STY d,X
        instructions[0x94] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.zero_page_x_map();
                let y = bus.cpu().y();
                bus.write_memory(addr, y);
                bus.cpu_mut().go_forward(2);
                4
            }
        );
        // STY a
        instructions[0x8C] = Instruction::new(
            |bus: &mut Bus| {
                let addr = bus.absolute_map();
                let y = bus.cpu().y();
                bus.write(addr, y);
                bus.cpu_mut().go_forward(3);
                4
            }
        );
        // TAX
        instructions[0xAA] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().tax();
                bus.cpu_mut().go_forward(1);
                2
            }
        );
        // TAY
        instructions[0xA8] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().tay();
                bus.cpu_mut().go_forward(1);
                2
            }
        );
        // TYA
        instructions[0x98] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().tya();
                bus.cpu_mut().go_forward(1);
                2
            }
        );
        // TSX
        instructions[0xBA] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().tsx();
                bus.cpu_mut().go_forward(1);
                2
            }
        );
        // TXA
        instructions[0x8A] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().txa();
                bus.cpu_mut().go_forward(1);
                2
            }
        );
        // TXS
        instructions[0x9A] = Instruction::new(
            |bus: &mut Bus| {
                bus.cpu_mut().txs();
                bus.cpu_mut().go_forward(1);
                2
            }
        );

        InstructionSet {
            instructions
        }
    }

    pub fn find_instruction(&self, code: u8) -> &Instruction {
        &self.instructions[code as usize]
    }
}

fn default_action(_: &mut Bus) -> u8 {
    panic!("Invalid instruction");
}