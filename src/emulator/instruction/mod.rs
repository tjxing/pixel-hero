mod instruction;
mod instruction_set;

pub use self::instruction::Instruction;
pub use self::instruction_set::InstructionSet;

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn test_adc_immediate() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x69);

        bus.write_memory(1, 0x10);
        bus.write_memory(3, 0x80);
        bus.write_memory(5, 0x40);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0x10);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x20);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x00);
        assert_eq!(cycles, 2);

        bus.cpu_mut().lda(0x80);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x00);
        assert_eq!(bus.cpu().p() & 0xC3, 0x43);

        bus.cpu_mut().lda(0x40);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x81);
        assert_eq!(bus.cpu().p() & 0xC3, 0xC0);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_adc_zero_page() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x65);

        bus.write_memory(1, 0x10);
        bus.write_memory(0x10, 0x80);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0x10);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x90);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 3);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_adc_zero_page_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x75);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x10, 0xCC);
        bus.write_memory(0x30, 0x80);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x10);
        bus.cpu_mut().lda(0x10);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x90);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xF0);
        bus.cpu_mut().lda(0xD0);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x9c);
        assert_eq!(bus.cpu().p() & 0xC3, 0x81);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_adc_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x6D);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1020, 0x06);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0x60);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x66);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_adc_absolute_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x7D);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0x06);
        bus.write(0x1100, 0x60);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x1);
        bus.cpu_mut().lda(0x60);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x66);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xE0);
        bus.cpu_mut().lda(0x60);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xC0);
        assert_eq!(bus.cpu().p() & 0xC3, 0xC0);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_adc_absolute_y() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x79);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0x06);
        bus.write(0x1100, 0x60);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0x1);
        bus.cpu_mut().lda(0x60);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x66);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0xE0);
        bus.cpu_mut().lda(0x60);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xC0);
        assert_eq!(bus.cpu().p() & 0xC3, 0xC0);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_adc_index_indirect() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x61);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write_memory(3, 0x03);
        bus.write(0x0310, 0x06);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xE2);
        bus.cpu_mut().lda(0x60);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x66);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_adc_indirect_index() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x71);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x20, 0x10);
        bus.write_memory(0x21, 0x03);
        bus.write(0x0311, 0x06);
        bus.write(0x0401, 0x07);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0x01);
        bus.cpu_mut().lda(0x60);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x66);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 5);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0xF1);
        bus.cpu_mut().lda(0x60);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x67);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_and_immediate() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x29);

        bus.write_memory(1, 0xA5);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0x83);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x81);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 2);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0x5A);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_and_zero_page() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x25);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x20, 0xA5);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0x83);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x81);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 3);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_and_zero_page_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x35);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x21, 0xA5);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(1);
        bus.cpu_mut().lda(0x83);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x81);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_and_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x2D);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1020, 0xA5);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0x83);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x81);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_and_absolute_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x3D);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0xA5);
        bus.write(0x1112, 0x01);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(1);
        bus.cpu_mut().lda(0x83);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x81);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xF2);
        bus.cpu_mut().lda(0x83);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x01);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_and_absolute_y() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x39);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0xA5);
        bus.write(0x1112, 0x01);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(1);
        bus.cpu_mut().lda(0x83);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x81);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0xF2);
        bus.cpu_mut().lda(0x83);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x01);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_and_index_indirect() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x21);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write_memory(3, 0x03);
        bus.write(0x0310, 0xA5);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xE2);
        bus.cpu_mut().lda(0x83);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x81);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_and_indirect_index() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x31);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x20, 0x10);
        bus.write_memory(0x21, 0x03);
        bus.write(0x0311, 0xA5);
        bus.write(0x0401, 0x01);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0x01);
        bus.cpu_mut().lda(0x83);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x81);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 5);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0xF1);
        bus.cpu_mut().lda(0x83);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x01);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_asl_a() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x0A);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0x01);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x02);
        assert_eq!(bus.cpu().pc(), 1);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 2);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0x80);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x03);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0x72);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xE4);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_asl_zero_page() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x06);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x20, 0x81);

        bus.cpu_mut().goto(0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x20), 0x02);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_asl_zero_page_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x16);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x21, 0x81);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(1);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x21), 0x02);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_asl_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x0E);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1020, 0x81);

        bus.cpu_mut().goto(0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read(0x1020), 0x02);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_asl_absolute_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x1E);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0x81);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(1);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read(0x1021), 0x02);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 7);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_bcc() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x90);

        bus.write_memory(0x11, 0x05);
        bus.write_memory(0x21, 0xFC);
        bus.write_memory(0x31, 0x02);

        bus.cpu_mut().goto(0x10);
        bus.cpu_mut().set_p(0x01);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x12);

        bus.cpu_mut().goto(0x20);
        bus.cpu_mut().set_p(0);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x1E);

        bus.cpu_mut().goto(0x30);
        bus.cpu_mut().set_p(0);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x34);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_bcs() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xB0);

        bus.write_memory(0x11, 0x05);
        bus.write_memory(0x21, 0xFC);
        bus.write_memory(0x31, 0x02);

        bus.cpu_mut().goto(0x10);
        bus.cpu_mut().set_p(0);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x12);

        bus.cpu_mut().goto(0x20);
        bus.cpu_mut().set_p(1);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x1E);

        bus.cpu_mut().goto(0x30);
        bus.cpu_mut().set_p(1);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x34);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_beq() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xF0);

        bus.write_memory(0x11, 0x05);
        bus.write_memory(0x21, 0xFC);
        bus.write_memory(0x31, 0x02);

        bus.cpu_mut().goto(0x10);
        bus.cpu_mut().set_p(0);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x12);

        bus.cpu_mut().goto(0x20);
        bus.cpu_mut().set_p(2);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x1E);

        bus.cpu_mut().goto(0x30);
        bus.cpu_mut().set_p(2);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x34);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_bne() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xD0);

        bus.write_memory(0x11, 0x05);
        bus.write_memory(0x21, 0xFC);
        bus.write_memory(0x31, 0x02);

        bus.cpu_mut().goto(0x10);
        bus.cpu_mut().set_p(2);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x12);

        bus.cpu_mut().goto(0x20);
        bus.cpu_mut().set_p(0);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x1E);

        bus.cpu_mut().goto(0x30);
        bus.cpu_mut().set_p(0);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x34);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_bit_zero_page() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x24);

        bus.write_memory(1, 0x20);
        bus.write_memory(3, 0x30);
        bus.write_memory(0x20, 0xA5);
        bus.write_memory(0x30, 0xC0);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0xF0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 3);

        bus.cpu_mut().lda(0x0F);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 4);
        assert_eq!(bus.cpu().p() & 0xC3, 0xC2);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_bit_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x2C);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1020, 0x07);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0xF0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_bmi() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x30);

        bus.write_memory(0x11, 0x05);
        bus.write_memory(0x21, 0xFC);
        bus.write_memory(0x31, 0x02);

        bus.cpu_mut().goto(0x10);
        bus.cpu_mut().set_p(0x7F);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x12);

        bus.cpu_mut().goto(0x20);
        bus.cpu_mut().set_p(0x80);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x1E);

        bus.cpu_mut().goto(0x30);
        bus.cpu_mut().set_p(0x80);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x34);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_bpl() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x10);

        bus.write_memory(0x11, 0x05);
        bus.write_memory(0x21, 0xFC);
        bus.write_memory(0x31, 0x02);

        bus.cpu_mut().goto(0x10);
        bus.cpu_mut().set_p(0x80);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x12);

        bus.cpu_mut().goto(0x20);
        bus.cpu_mut().set_p(0x7F);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x1E);

        bus.cpu_mut().goto(0x30);
        bus.cpu_mut().set_p(0x7F);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x34);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_bvc() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x50);

        bus.write_memory(0x11, 0x05);
        bus.write_memory(0x21, 0xFC);
        bus.write_memory(0x31, 0x02);

        bus.cpu_mut().goto(0x10);
        bus.cpu_mut().set_p(0x40);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x12);

        bus.cpu_mut().goto(0x20);
        bus.cpu_mut().set_p(0xBF);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x1E);

        bus.cpu_mut().goto(0x30);
        bus.cpu_mut().set_p(0xBF);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x34);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_bvs() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x70);

        bus.write_memory(0x11, 0x05);
        bus.write_memory(0x21, 0xFC);
        bus.write_memory(0x31, 0x02);

        bus.cpu_mut().goto(0x10);
        bus.cpu_mut().set_p(0xBF);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x12);

        bus.cpu_mut().goto(0x20);
        bus.cpu_mut().set_p(0x40);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x1E);

        bus.cpu_mut().goto(0x30);
        bus.cpu_mut().set_p(0x40);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x34);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_clc() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x18);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().set_p(0xFF);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 1);
        assert_eq!(bus.cpu().p() & 0xCF, 0xCE);
        assert_eq!(cycles, 2);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_cld() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xD8);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().set_p(0xFF);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 1);
        assert_eq!(bus.cpu().p() & 0xCF, 0xC7);
        assert_eq!(cycles, 2);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_cli() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x58);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().set_p(0xFF);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 1);
        assert_eq!(bus.cpu().p() & 0xCF, 0xCB);
        assert_eq!(cycles, 2);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_clv() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xB8);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().set_p(0xFF);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 1);
        assert_eq!(bus.cpu().p() & 0xCF, 0x8F);
        assert_eq!(cycles, 2);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_cmp_immediate() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xC9);

        bus.write_memory(1, 0x10);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().lda(0x20);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 2);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().lda(0x10);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().p() & 0xC3, 0x03);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().lda(0x08);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_cmp_zero_page() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xC5);

        bus.write_memory(1, 0x10);
        bus.write_memory(0x10, 0xFF);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().lda(0x20);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 3);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_cmp_zero_page_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xD5);

        bus.write_memory(1, 0x10);
        bus.write_memory(0x12, 0x20);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().ldx(2);
        bus.cpu_mut().lda(0xFF);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x81);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_cmp_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xCD);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1020, 0x20);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().lda(0x21);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_cmp_absolute_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xDD);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0x21);
        bus.write(0x1112, 0x1F);
        bus.cpu_mut().lda(0x20);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().ldx(1);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().ldx(0xF2);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_cmp_absolute_y() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xD9);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0x21);
        bus.write(0x1112, 0x1F);
        bus.cpu_mut().lda(0x20);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().ldy(1);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().ldy(0xF2);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_cmp_index_indirect() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xC1);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write_memory(3, 0x03);
        bus.write(0x0310, 0x06);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xE2);
        bus.cpu_mut().lda(0x10);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_cmp_indirect_index() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xD1);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x20, 0x10);
        bus.write_memory(0x21, 0x03);
        bus.write(0x0311, 0x2F);
        bus.write(0x0401, 0x0A);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0x01);
        bus.cpu_mut().lda(0x10);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 5);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0xF1);
        bus.cpu_mut().lda(0x10);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_cpx_immediate() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xE0);

        bus.write_memory(1, 0x10);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().ldx(0x20);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 2);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().ldx(0x10);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().p() & 0xC3, 0x03);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().ldx(0x08);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_cpx_zero_page() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xE4);

        bus.write_memory(1, 0x10);
        bus.write_memory(0x10, 0xFF);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().ldx(0x20);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 3);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_cpx_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xEC);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1020, 0x20);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().ldx(0x21);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_cpy_immediate() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xC0);

        bus.write_memory(1, 0x10);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().ldy(0x20);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 2);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().ldy(0x10);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().p() & 0xC3, 0x03);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().ldy(0x08);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_cpy_zero_page() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xC4);

        bus.write_memory(1, 0x10);
        bus.write_memory(0x10, 0xFF);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().ldy(0x20);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 3);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_cpy_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xCC);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1020, 0x20);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().ldy(0x21);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_dec_zero_page() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xC6);

        bus.write_memory(1, 0x10);
        bus.write_memory(0x10, 0x10);
        bus.write_memory(3, 0x20);
        bus.write_memory(0x20, 0x85);
        bus.write_memory(5, 0x30);
        bus.write_memory(0x30, 0x01);
        bus.write_memory(7, 0x40);
        bus.write_memory(0x40, 0);
        bus.cpu_mut().goto(0x0);

        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.read_memory(0x10), 0x0F);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 5);

        inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x20), 0x84);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);

        inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x30), 0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);

        inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x40), 0xFF);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_dec_zero_page_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xD6);

        bus.write_memory(1, 0x10);
        bus.write_memory(0x12, 0x10);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().ldx(2);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.read_memory(0x12), 0x0F);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_dec_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xCE);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1020, 0x04);

        bus.cpu_mut().goto(0x0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.read(0x1020), 0x03);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_dec_absolute_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xDE);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0xF0);
        bus.write(0x1112, 0x0);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().ldx(1);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.read(0x1021), 0xEF);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 6);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().ldx(0xF2);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.read(0x1112), 0xFF);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 7);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_dex() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xCA);

        bus.cpu_mut().goto(0x0);

        bus.cpu_mut().ldx(0x10);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 1);
        assert_eq!(bus.cpu().x(), 0x0F);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 2);

        bus.cpu_mut().ldx(0x85);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().x(), 0x84);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);

        bus.cpu_mut().ldx(0x01);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().x(), 0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);

        bus.cpu_mut().ldx(0);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().x(), 0xFF);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_dey() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x88);

        bus.cpu_mut().goto(0x0);

        bus.cpu_mut().ldy(0x10);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 1);
        assert_eq!(bus.cpu().y(), 0x0F);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 2);

        bus.cpu_mut().ldy(0x85);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().y(), 0x84);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);

        bus.cpu_mut().ldy(0x01);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().y(), 0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);

        bus.cpu_mut().ldy(0);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().y(), 0xFF);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_eor_immediate() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x49);

        bus.write_memory(1, 0x10);
        bus.write_memory(3, 0x80);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0x10);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
        assert_eq!(cycles, 2);

        bus.cpu_mut().lda(0x01);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x81);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_eor_zero_page() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x45);

        bus.write_memory(1, 0x10);
        bus.write_memory(0x10, 0x85);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0x90);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x15);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 3);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_eor_zero_page_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x55);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x10, 0x0F);
        bus.write_memory(0x30, 0x10);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x10);
        bus.cpu_mut().lda(0xFF);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xEF);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xF0);
        bus.cpu_mut().lda(0xFF);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xF0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_eor_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x4D);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1020, 0xF0);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0xFF);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x0F);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_eor_absolute_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x5D);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0xF0);
        bus.write(0x1100, 0x0F);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x1);
        bus.cpu_mut().lda(0xFF);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x0F);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xE0);
        bus.cpu_mut().lda(0xFF);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xF0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_eor_absolute_y() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x59);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0xF0);
        bus.write(0x1100, 0x0F);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0x1);
        bus.cpu_mut().lda(0xFF);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x0F);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0xE0);
        bus.cpu_mut().lda(0xFF);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xF0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_eor_index_indirect() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x41);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write_memory(3, 0x03);
        bus.write(0x0310, 0xFF);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xE2);
        bus.cpu_mut().lda(0x0F);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xF0);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_eor_indirect_index() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x51);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x20, 0x10);
        bus.write_memory(0x21, 0x03);
        bus.write(0x0311, 0x0F);
        bus.write(0x0401, 0xF0);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0x01);
        bus.cpu_mut().lda(0xFF);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xF0);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 5);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0xF1);
        bus.cpu_mut().lda(0xFF);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x0F);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_inc_zero_page() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xE6);

        bus.write_memory(1, 0x10);
        bus.write_memory(0x10, 0x10);
        bus.write_memory(3, 0x20);
        bus.write_memory(0x20, 0xEF);
        bus.write_memory(5, 0x30);
        bus.write_memory(0x30, 0x84);
        bus.write_memory(7, 0x40);
        bus.write_memory(0x40, 0xFF);
        bus.cpu_mut().goto(0x0);

        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.read_memory(0x10), 0x11);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 5);

        inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x20), 0xF0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);

        inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x30), 0x85);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);

        inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x40), 0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_inc_zero_page_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xF6);

        bus.write_memory(1, 0x10);
        bus.write_memory(0x12, 0);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().ldx(2);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.read_memory(0x12), 0x01);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_inc_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xEE);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1020, 0x9F);

        bus.cpu_mut().goto(0x0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.read(0x1020), 0xa0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_inc_absolute_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xFE);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0xFF);
        bus.write(0x1112, 0x0F);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().ldx(1);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.read(0x1021), 0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
        assert_eq!(cycles, 6);

        bus.cpu_mut().goto(0x0);
        bus.cpu_mut().ldx(0xF2);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.read(0x1112), 0x10);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 7);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_inx() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xE8);

        bus.cpu_mut().goto(0x0);

        bus.cpu_mut().ldx(0x10);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 1);
        assert_eq!(bus.cpu().x(), 0x11);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 2);

        bus.cpu_mut().ldx(0xEF);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().x(), 0xF0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);

        bus.cpu_mut().ldx(0x84);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().x(), 0x85);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);

        bus.cpu_mut().ldx(0xFF);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().x(), 0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_iny() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xC8);

        bus.cpu_mut().goto(0x0);

        bus.cpu_mut().ldy(0x10);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 1);
        assert_eq!(bus.cpu().y(), 0x11);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 2);

        bus.cpu_mut().ldy(0xEF);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().y(), 0xF0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);

        bus.cpu_mut().ldy(0x84);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().y(), 0x85);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);

        bus.cpu_mut().ldy(0xFF);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().y(), 0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_jmp_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x4C);

        bus.write_memory(4, 0x33);
        bus.write_memory(5, 0x23);
        bus.cpu_mut().goto(0x3);

        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x2333);
        assert_eq!(cycles, 3);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_jmp_indirect() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x6C);

        bus.write_memory(4, 0x10);
        bus.write_memory(5, 0);
        bus.write_memory(0x10, 0x33);
        bus.write_memory(0x11, 0x23);
        bus.cpu_mut().goto(0x3);

        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x2333);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_jsr() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x20);

        bus.write_memory(4, 0x22);
        bus.write_memory(5, 0x20);
        bus.cpu_mut().goto(3);

        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x2022);
        assert_eq!(bus.pop_word(), 5);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_lda_immediate() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xA9);

        bus.write_memory(1, 0x10);
        bus.write_memory(3, 0xA2);
        bus.write_memory(5, 0x0);

        bus.cpu_mut().goto(0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x10);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x00);
        assert_eq!(cycles, 2);

        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xA2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);

        bus.cpu_mut().lda(0x40);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_lda_zero_page() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xA5);

        bus.write_memory(1, 0x10);
        bus.write_memory(0x10, 0x80);

        bus.cpu_mut().goto(0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x80);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 3);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_lda_zero_page_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xB5);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x10, 0xCC);
        bus.write_memory(0x30, 0x80);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x10);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x80);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xF0);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xCC);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_lda_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xAD);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1020, 0);

        bus.cpu_mut().goto(0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_lda_absolute_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xBD);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0x0F);
        bus.write(0x1100, 0xF0);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x1);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x0F);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xE0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xF0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_lda_absolute_y() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xB9);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0x0F);
        bus.write(0x1100, 0xF0);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0x1);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x0F);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0xE0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xF0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_lda_index_indirect() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xA1);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write_memory(3, 0x03);
        bus.write(0x0310, 0x06);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xE2);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x06);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_lda_indirect_index() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xB1);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x20, 0x10);
        bus.write_memory(0x21, 0x03);
        bus.write(0x0311, 0xF0);
        bus.write(0x0401, 0x0F);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0x01);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xF0);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 5);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0xF1);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x0F);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ldx_immediate() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xA2);

        bus.write_memory(1, 0x10);
        bus.write_memory(3, 0xA2);
        bus.write_memory(5, 0x0);

        bus.cpu_mut().goto(0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().x(), 0x10);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x00);
        assert_eq!(cycles, 2);

        inst.apply(&mut bus);
        assert_eq!(bus.cpu().x(), 0xA2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);

        bus.cpu_mut().lda(0x40);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().x(), 0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ldx_zero_page() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xA6);

        bus.write_memory(1, 0x10);
        bus.write_memory(0x10, 0x80);

        bus.cpu_mut().goto(0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().x(), 0x80);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 3);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ldx_zero_page_y() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xB6);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x10, 0xCC);
        bus.write_memory(0x30, 0x80);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0x10);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().x(), 0x80);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0xF0);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().x(), 0xCC);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ldx_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xAE);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1020, 0);

        bus.cpu_mut().goto(0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().x(), 0);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ldx_absolute_y() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xBE);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0x0F);
        bus.write(0x1100, 0xF0);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0x1);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().x(), 0x0F);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0xE0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().x(), 0xF0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ldy_immediate() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xA0);

        bus.write_memory(1, 0x10);
        bus.write_memory(3, 0xA2);
        bus.write_memory(5, 0x0);

        bus.cpu_mut().goto(0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().y(), 0x10);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x00);
        assert_eq!(cycles, 2);

        inst.apply(&mut bus);
        assert_eq!(bus.cpu().y(), 0xA2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);

        bus.cpu_mut().lda(0x40);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().y(), 0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ldy_zero_page() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xA4);

        bus.write_memory(1, 0x10);
        bus.write_memory(0x10, 0x80);

        bus.cpu_mut().goto(0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().y(), 0x80);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 3);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ldy_zero_page_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xB4);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x10, 0xCC);
        bus.write_memory(0x30, 0x80);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x10);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().y(), 0x80);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xF0);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().y(), 0xCC);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ldy_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xAC);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1020, 0);

        bus.cpu_mut().goto(0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().y(), 0);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ldy_absolute_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xBC);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0x0F);
        bus.write(0x1100, 0xF0);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x1);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().y(), 0x0F);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xE0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().y(), 0xF0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_lsr_a() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x4A);

        bus.cpu_mut().goto(3);

        bus.cpu_mut().lda(0x86);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x43);
        assert_eq!(bus.cpu().pc(), 4);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 2);

        bus.cpu_mut().lda(0xA5);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x52);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);

        bus.cpu_mut().lda(0x01);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x03);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_lsr_zero_page() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x46);

        bus.write_memory(1, 0x10);
        bus.write_memory(0x10, 0xFF);

        bus.cpu_mut().goto(0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x10), 0x7F);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_lsr_zero_page_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x56);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x10, 0xCC);
        bus.write_memory(0x30, 0x81);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x10);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x30), 0x40);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 6);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xF0);
        inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x10), 0x66);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_lsr_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x4E);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1020, 0);

        bus.cpu_mut().goto(0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read(0x1020), 0);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_lsr_absolute_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x5E);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0x0F);
        bus.write(0x1100, 0xF0);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x1);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read(0x1021), 0x07);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 6);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xE0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read(0x1100), 0x78);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 7);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_nop() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xEA);

        bus.cpu_mut().goto(0x10);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x11);
        assert_eq!(cycles, 2);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ora_immediate() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x09);

        bus.write_memory(1, 0x59);
        bus.write_memory(3, 0x80);
        bus.write_memory(5, 0);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0x21);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x79);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x00);
        assert_eq!(cycles, 2);

        bus.cpu_mut().lda(0x44);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xC4);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);

        bus.cpu_mut().lda(0);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ora_zero_page() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x05);

        bus.write_memory(1, 0x10);
        bus.write_memory(0x10, 0x44);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0x80);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xC4);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 3);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ora_zero_page_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x15);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x10, 0xCC);
        bus.write_memory(0x30, 0x80);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x10);
        bus.cpu_mut().lda(0x10);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x90);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xF0);
        bus.cpu_mut().lda(0xD1);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xDD);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ora_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x0D);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1020, 0x06);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0x24);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x26);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ora_absolute_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x1D);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0xA5);
        bus.write(0x1100, 0x44);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x1);
        bus.cpu_mut().lda(0x12);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xB7);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xE0);
        bus.cpu_mut().lda(0x11);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x55);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ora_absolute_y() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x19);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0xA5);
        bus.write(0x1100, 0x44);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0x1);
        bus.cpu_mut().lda(0x12);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xB7);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0xE0);
        bus.cpu_mut().lda(0x11);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x55);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ora_index_indirect() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x01);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write_memory(3, 0x03);
        bus.write(0x0310, 0xA5);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xE2);
        bus.cpu_mut().lda(0x68);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xED);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ora_indirect_index() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x11);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x20, 0x10);
        bus.write_memory(0x21, 0x03);
        bus.write(0x0311, 0xF2);
        bus.write(0x0401, 0x71);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0x01);
        bus.cpu_mut().lda(0x01);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xF3);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 5);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0xF1);
        bus.cpu_mut().lda(0x02);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x73);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_pha() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x48);

        bus.cpu_mut().goto(5);
        bus.cpu_mut().lda(0xEF);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.pop(), 0xEF);
        assert_eq!(bus.cpu().pc(), 6);
        assert_eq!(cycles, 3);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_php() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x08);

        bus.cpu_mut().goto(5);
        bus.cpu_mut().set_p(0xEF);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.pop() & 0xCF, 0xCF);
        assert_eq!(bus.cpu().pc(), 6);
        assert_eq!(cycles, 3);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_pla() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x68);

        bus.push(0);
        bus.push(0xA5);
        bus.push(0x64);
        bus.cpu_mut().goto(5);

        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x64);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(bus.cpu().pc(), 6);
        assert_eq!(cycles, 4);

        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xA5);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);

        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_plp() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x28);

        bus.push(0x85);
        bus.cpu_mut().goto(5);

        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().p() & 0xCF, 0x85);
        assert_eq!(bus.cpu().pc(), 6);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_rol_a() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x2A);

        bus.cpu_mut().goto(3);

        bus.cpu_mut().set_p(0);
        bus.cpu_mut().lda(0x80);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0);
        assert_eq!(bus.cpu().pc(), 4);
        assert_eq!(bus.cpu().p() & 0xC3, 0x03);
        assert_eq!(cycles, 2);

        bus.cpu_mut().set_p(1);
        bus.cpu_mut().set_p(0);
        bus.cpu_mut().lda(0xA5);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x4A);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);

        bus.cpu_mut().set_p(1);
        bus.cpu_mut().lda(0x18);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x31);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_rol_zero_page() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x26);

        bus.write_memory(1, 0x10);
        bus.write_memory(0x10, 0x75);

        bus.cpu_mut().set_p(0x01);
        bus.cpu_mut().goto(0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x10), 0xEB);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_rol_zero_page_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x36);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x10, 0xCC);
        bus.write_memory(0x30, 0x81);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x10);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x30), 0x02);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 6);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xF0);
        inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x10), 0x99);
        assert_eq!(bus.cpu().p() & 0xC3, 0x81);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_rol_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x2E);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1020, 0x32);

        bus.cpu_mut().goto(0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read(0x1020), 0x64);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_rol_absolute_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x3E);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0xF0);
        bus.write(0x1100, 0x0F);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x1);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read(0x1021), 0xE0);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x81);
        assert_eq!(cycles, 6);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xE0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read(0x1100), 0x1F);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 7);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ror_a() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x6A);

        bus.cpu_mut().goto(3);

        bus.cpu_mut().set_p(0);
        bus.cpu_mut().lda(0x07);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x03);
        assert_eq!(bus.cpu().pc(), 4);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 2);

        bus.cpu_mut().set_p(1);
        bus.cpu_mut().lda(0xA5);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xD2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x81);

        bus.cpu_mut().set_p(0);
        bus.cpu_mut().lda(0x01);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x03);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ror_zero_page() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x66);

        bus.write_memory(1, 0x10);
        bus.write_memory(0x10, 0x75);

        bus.cpu_mut().set_p(0x01);
        bus.cpu_mut().goto(0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x10), 0xBA);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x81);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ror_zero_page_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x76);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x10, 0xCC);
        bus.write_memory(0x30, 0x81);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x10);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x30), 0x40);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 6);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xF0);
        inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x10), 0xe6);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ror_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x6E);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1020, 0x32);

        bus.cpu_mut().goto(0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read(0x1020), 0x19);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_ror_absolute_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x7E);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0x0F);
        bus.write(0x1100, 0xF0);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x1);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read(0x1021), 0x07);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 6);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xE0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read(0x1100), 0xF8);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(cycles, 7);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_rti() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x40);

        bus.push(0x01);
        bus.push(0x02);
        bus.push(0x03);

        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x0102);
        assert_eq!(bus.cpu().p() & 0xC3, 0x03);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_rts() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x60);

        bus.push(0x01);
        bus.push(0x02);

        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().pc(), 0x0103);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sbc_immediate() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xE9);

        bus.write_memory(1, 0x10);
        bus.write_memory(3, 0x80);
        bus.write_memory(5, 0x40);

        bus.cpu_mut().set_p(1);
        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0x10);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x0);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x03);
        assert_eq!(cycles, 2);

        bus.cpu_mut().lda(0x40);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xC0);
        assert_eq!(bus.cpu().p() & 0xC3, 0xC0);

        bus.cpu_mut().lda(0x53);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x12);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sbc_zero_page() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xE5);

        bus.write_memory(1, 0x10);
        bus.write_memory(0x10, 0xFF);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
        assert_eq!(cycles, 3);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sbc_zero_page_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xF5);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x10, 0x4B);
        bus.write_memory(0x30, 0xA4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x10);
        bus.cpu_mut().lda(0x5A);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xB5);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0xC0);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xF0);
        bus.cpu_mut().lda(0x5A);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x0E);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sbc_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xED);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1020, 0x80);

        bus.cpu_mut().set_p(1);
        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x80);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0xC0);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sbc_absolute_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xFD);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0x05);
        bus.write(0x1100, 0x30);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x1);
        bus.cpu_mut().lda(0x82);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x7c);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x41);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xE0);
        bus.cpu_mut().lda(0xA4);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x74);
        assert_eq!(bus.cpu().p() & 0xC3, 0x41);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sbc_absolute_y() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xF9);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write(0x1021, 0x05);
        bus.write(0x1100, 0x30);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0x1);
        bus.cpu_mut().lda(0x82);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x7c);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(bus.cpu().p() & 0xC3, 0x41);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0xE0);
        bus.cpu_mut().lda(0xA4);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x74);
        assert_eq!(bus.cpu().p() & 0xC3, 0x41);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sbc_index_indirect() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xE1);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write_memory(3, 0x03);
        bus.write(0x0310, 0x06);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xE2);
        bus.cpu_mut().lda(0x60);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x59);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sbc_indirect_index() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xF1);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x20, 0x10);
        bus.write_memory(0x21, 0x03);
        bus.write(0x0311, 0x06);
        bus.write(0x0401, 0x07);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0x01);
        bus.cpu_mut().lda(0x60);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x59);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 6);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0xF1);
        bus.cpu_mut().lda(0x60);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x59);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 7);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sec() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x38);

        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().p() & 0xC3, 0x01);
        assert_eq!(cycles, 2);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sed() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xF8);

        bus.cpu_mut().set_p(0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().p() & 0xCF, 0x08);
        assert_eq!(cycles, 2);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sei() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x78);

        bus.cpu_mut().set_p(0);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().p() & 0xCF, 0x04);
        assert_eq!(cycles, 2);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sta_zero_page() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x85);

        bus.write_memory(1, 0x10);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0xC2);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x10), 0xC2);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(cycles, 3);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sta_zero_page_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x95);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x10, 0x4B);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x10);
        bus.cpu_mut().lda(0x5A);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x30), 0x5A);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xF0);
        bus.cpu_mut().lda(0x5B);
        inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x10), 0x5B);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sta_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x8D);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);

        bus.cpu_mut().set_p(1);
        bus.cpu_mut().goto(0);
        bus.cpu_mut().lda(0x23);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read(0x1020), 0x23);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sta_absolute_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x9D);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x1);
        bus.cpu_mut().lda(0x82);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read(0x1021), 0x82);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xE0);
        bus.cpu_mut().lda(0xA4);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read(0x1100), 0xA4);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sta_absolute_y() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x99);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0x1);
        bus.cpu_mut().lda(0x82);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read(0x1021), 0x82);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0xE0);
        bus.cpu_mut().lda(0xA4);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read(0x1100), 0xA4);
        assert_eq!(cycles, 5);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sta_index_indirect() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x81);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);
        bus.write_memory(3, 0x03);
        bus.write(0x0310, 0x06);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xE2);
        bus.cpu_mut().lda(0x60);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read(0x0310), 0x60);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(cycles, 6);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sta_indirect_index() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x91);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x20, 0x10);
        bus.write_memory(0x21, 0x03);
        bus.write(0x0311, 0x06);
        bus.write(0x0401, 0x07);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0x01);
        bus.cpu_mut().lda(0x60);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read(0x0311), 0x60);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(cycles, 6);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0xF1);
        bus.cpu_mut().lda(0x61);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read(0x0401), 0x61);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(cycles, 7);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_stx_zero_page() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x86);

        bus.write_memory(1, 0x10);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xC2);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x10), 0xC2);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(cycles, 3);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_stx_zero_page_y() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x96);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x10, 0x4B);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0x10);
        bus.cpu_mut().ldx(0x5A);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x30), 0x5A);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0xF0);
        bus.cpu_mut().ldx(0x5B);
        inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x10), 0x5B);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_stx_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x8E);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x23);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read(0x1020), 0x23);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sty_zero_page() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x84);

        bus.write_memory(1, 0x10);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0xC2);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x10), 0xC2);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(cycles, 3);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sty_zero_page_x() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x94);

        bus.write_memory(1, 0x20);
        bus.write_memory(0x10, 0x4B);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0x10);
        bus.cpu_mut().ldy(0x5A);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x30), 0x5A);
        assert_eq!(bus.cpu().pc(), 2);
        assert_eq!(cycles, 4);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldx(0xF0);
        bus.cpu_mut().ldy(0x5B);
        inst.apply(&mut bus);
        assert_eq!(bus.read_memory(0x10), 0x5B);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sty_absolute() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x8C);

        bus.write_memory(1, 0x20);
        bus.write_memory(2, 0x10);

        bus.cpu_mut().goto(0);
        bus.cpu_mut().ldy(0x23);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.read(0x1020), 0x23);
        assert_eq!(bus.cpu().pc(), 3);
        assert_eq!(cycles, 4);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_tax() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xAA);

        bus.cpu_mut().goto(5);
        bus.cpu_mut().lda(0x23);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().x(), 0x23);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(bus.cpu().pc(), 6);
        assert_eq!(cycles, 2);

        bus.cpu_mut().lda(0xA8);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().x(), 0xA8);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);

        bus.cpu_mut().lda(0);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().x(), 0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_txa() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x8A);

        bus.cpu_mut().goto(5);
        bus.cpu_mut().ldx(0x23);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x23);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(bus.cpu().pc(), 6);
        assert_eq!(cycles, 2);

        bus.cpu_mut().ldx(0xA8);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xA8);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);

        bus.cpu_mut().ldx(0);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_tay() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xA8);

        bus.cpu_mut().goto(5);
        bus.cpu_mut().lda(0x23);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().y(), 0x23);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(bus.cpu().pc(), 6);
        assert_eq!(cycles, 2);

        bus.cpu_mut().lda(0xA8);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().y(), 0xA8);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);

        bus.cpu_mut().lda(0);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().y(), 0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_tya() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x98);

        bus.cpu_mut().goto(5);
        bus.cpu_mut().ldy(0x23);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0x23);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(bus.cpu().pc(), 6);
        assert_eq!(cycles, 2);

        bus.cpu_mut().ldy(0xA8);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0xA8);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);

        bus.cpu_mut().ldy(0);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu().a(), 0);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
    }
    #[test]
    #[wasm_bindgen_test]
    fn test_tsx() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0xBA);

        bus.cpu_mut().goto(5);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu().x(), 0xFD);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);
        assert_eq!(bus.cpu().pc(), 6);
        assert_eq!(cycles, 2);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_txs() {
        let instructions = InstructionSet::new();
        let mut bus = crate::emulator::bus::tests::mock();
        let inst = instructions.find_instruction(0x9A);

        bus.cpu_mut().goto(5);
        bus.cpu_mut().ldx(0x10);
        let cycles = inst.apply(&mut bus);
        assert_eq!(bus.cpu_mut().push(), 0x10);
        assert_eq!(bus.cpu().p() & 0xC3, 0);
        assert_eq!(bus.cpu().pc(), 6);
        assert_eq!(cycles, 2);

        bus.cpu_mut().ldx(0xF2);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu_mut().push(), 0xF2);
        assert_eq!(bus.cpu().p() & 0xC3, 0x80);

        bus.cpu_mut().ldx(0);
        inst.apply(&mut bus);
        assert_eq!(bus.cpu_mut().pop(), 1);
        assert_eq!(bus.cpu().p() & 0xC3, 0x02);
    }
}