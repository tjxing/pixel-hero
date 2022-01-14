use std::rc::Rc;
use crate::conf::Configuration;
use super::instruction::InstructionSet;
use crate::emulator::bus::Bus;

#[allow(non_snake_case)]
pub struct CPU {
    A: u8,
    X: u8,
    Y: u8,
    S: u8,
    PC: u16,
    P: Flags
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            A: 0, X: 0, Y: 0, S: 0xFD, PC: 0,
            P: Flags::from_byte(0x34)
        }
    }

    pub fn pc(&self) -> u16 {
        self.PC
    }

    pub fn goto(&mut self, pc: u16) {
        self.PC = pc;
    }

    pub fn go_forward(&mut self, steps: i16) {
        let new = self.PC as i32 + steps as i32;
        self.PC = new as u16;
    }

    pub fn a(&self) -> u8 {
        self.A
    }

    pub fn set_a(&mut self, a: u8) {
        self.A = a;
    }

    pub fn x(&self) -> u8 {
        self.X
    }

    pub fn y(&self) -> u8 {
        self.Y
    }

    pub fn p(&self) -> u8 {
        self.P.get()
    }

    pub fn set_p(&mut self, p: u8) {
        self.P.set(p);
    }

    pub fn adc(&mut self, num: u8) {
        let result = self.A as u16 + num as u16 + self.P.C as u16;
        self.P.C = ((result & 0x0100) >> 8) as u8;
        self.P.N = (result & 0x80) != 0;
        let a = (result & 0xFF) as u8;
        self.P.Z = a == 0;
        self.P.V = (num ^ self.A & 0x80) == 0 && (num ^ a & 0x80) != 0;
        self.A = a;

    }

    pub fn and(&mut self, num: u8) {
        self.A &= num;
        self.P.N = (self.A & 0x80) != 0;
        self.P.Z = self.A == 0;
    }

    pub fn or(&mut self, num: u8) {
        self.A |= num;
        self.P.N = (self.A & 0x80) != 0;
        self.P.Z = self.A == 0;
    }

    pub fn asl(&mut self, num: u8) -> u8 {
        let result = num << 1;
        self.P.C = (num & 0x80) >> 7;
        self.P.N = (result & 0x80) != 0;
        self.P.Z = result == 0;
        result as u8
    }

    pub fn asl_set_a(&mut self) {
        let result = self.asl(self.A);
        self.A = result;
    }

    pub fn branch(&mut self, offset: i8) -> u8 {
        let pc = self.PC;
        self.go_forward(offset as i16 + 2);
        let new_pc = self.PC;
        if (pc & 0xFF00) == (new_pc & 0xFF00) {1} else {2}
    }

    pub fn test_c_clear(&self) -> bool {
        self.P.C == 0
    }

    pub fn test_z_set(&self) -> bool {
        self.P.Z
    }

    pub fn test_n_set(&self) -> bool {
        self.P.N
    }

    pub fn test_v_set(&self) -> bool {
        self.P.V
    }

    pub fn test(&mut self, num: u8) {
        let result = self.A ^ num;
        self.P.N = (result & 0x80) != 0;
        self.P.V = (result & 0x01) != 0;
        self.P.Z = result == 0;
    }

    pub fn push(&mut self) -> u8 {
        let s = self.S;
        self.S -= 1;
        s
    }

    pub fn pop(&mut self) -> u8 {
        self.S += 1;
        self.S
    }

    pub fn clc(&mut self) {
        self.P.C = 0;
    }

    pub fn cld(&mut self) {
        self.P.D = false;
    }

    pub fn cli(&mut self) {
        self.P.I = false;
    }

    pub fn clv(&mut self) {
        self.P.V = false;
    }

    pub fn cmp(&mut self, num: u8) {
        self.compare(self.A, num);
    }

    pub fn cpx(&mut self, num: u8) {
        self.compare(self.X, num);
    }

    pub fn cpy(&mut self, num: u8) {
        self.compare(self.Y, num);
    }

    fn compare(&mut self, n1: u8, n2: u8) {
        if n1 == n2 {
            self.P.C = 0;
            self.P.N = false;
            self.P.Z = true;
        } else if n1 > n2 {
            self.P.C = 0;
            self.P.N = (n1 - n2) & 0x80 != 0;
            self.P.Z = false;
        } else if n1 < n2 {
            self.P.C = 1;
            self.P.N = (n2 - n1) <= 128;
            self.P.Z = false;
        }
    }

    pub fn dec(&mut self, num: u8) -> u8 {
        let result = if num > 0 {
            num - 1
        } else {
            255
        };
        self.P.N = (result & 0x80) != 0;
        self.P.Z = result == 0;
        result
    }

    pub fn dex(&mut self) {
        let x = self.dec(self.X);
        self.X = x;
    }

    pub fn dey(&mut self) {
        let y = self.dec(self.Y);
        self.Y = y;
    }

    pub fn eor(&mut self, num: u8) {
        let result = self.A ^ num;
        self.P.Z = result == 0;
        self.P.N = (result & 0x80) != 0;
        self.A = result;
    }

    pub fn inc(&mut self, num: u8) -> u8 {
        let result = if num < 255 {
            self.P.Z = false;
            num + 1
        } else {
            self.P.Z = true;
            0
        };
        self.P.N = (result & 0x80) != 0;
        result
    }

    pub fn inx(&mut self) {
        let x = self.inc(self.X);
        self.X = x;
    }

    pub fn iny(&mut self) {
        let y = self.inc(self.Y);
        self.Y = y;
    }

    pub fn lda(&mut self, num: u8) {
        self.A = num;
        self.P.N = (num & 0x80) != 0;
        self.P.Z = num == 0;
    }

    pub fn ldx(&mut self, num: u8) {
        self.X = num;
        self.P.N = (num & 0x80) != 0;
        self.P.Z = num == 0;
    }

    pub fn ldy(&mut self, num: u8) {
        self.Y = num;
        self.P.N = (num & 0x80) != 0;
        self.P.Z = num == 0;
    }

    pub fn lsr(&mut self, num: u8) -> u8 {
        let result = num >> 1;
        self.P.N = false;
        self.P.Z = result == 0;
        self.P.C = num & 0x01;
        result
    }

    pub fn lsr_a(&mut self) {
        let result = self.lsr(self.A);
        self.A = result;
    }

    pub fn rol(&mut self, num: u8) -> u8 {
        let result = (num << 1) | self.P.C;
        self.P.C = (num & 0x80) >> 7;
        self.P.N = (result & 0x80) != 0;
        self.P.Z = result == 0;
        result
    }

    pub fn rol_a(&mut self) {
        let a = self.rol(self.A);
        self.A = a;
    }

    pub fn ror(&mut self, num: u8) -> u8 {
        let result = (num >> 1) | (self.P.C << 7);
        self.P.N = self.P.C == 1;
        self.P.Z = result == 0;
        self.P.C = num & 0x01;
        result
    }

    pub fn ror_a(&mut self) {
        let a = self.ror(self.A);
        self.A = a;
    }

    pub fn sbc(&mut self, num: u8) {
        let result = (self.A as i16 - num as i16 - (1 - self.P.C) as i16) as u16;
        self.P.N = (result & 0x0080) != 0;
        self.P.Z = result == 0;
        self.P.C = ((result & 0x0100) >> 8) as u8;
        let a = (result & 0xFF) as u8;
        self.P.V = ((num & 0x80) == (self.A & 0x80)) && ((num & 0x80) != (a & 0x80));
        self.A = a;
    }

    pub fn sec(&mut self) {
        self.P.C = 1;
    }

    pub fn sed(&mut self) {
        self.P.D = true;
    }

    pub fn sei(&mut self) {
        self.P.I = true;
    }

    pub fn tax(&mut self) {
        self.X = self.A;
        self.P.N = (self.X & 0x80) != 0;
        self.P.Z = self.X == 0;
    }

    pub fn tay(&mut self) {
        self.Y = self.A;
        self.P.N = (self.Y & 0x80) != 0;
        self.P.Z = self.Y == 0;
    }

    pub fn tya(&mut self) {
        self.A = self.Y;
        self.P.N = (self.A & 0x80) != 0;
        self.P.Z = self.A == 0;
    }

    pub fn tsx(&mut self) {
        self.X = self.S;
        self.P.N = (self.X & 0x80) != 0;
        self.P.Z = self.X == 0;
    }

    pub fn txa(&mut self) {
        self.A = self.X;
        self.P.N = (self.A & 0x80) != 0;
        self.P.Z = self.A == 0;
    }

    pub fn txs(&mut self) {
        self.S = self.X;
        self.P.N = (self.S & 0x80) != 0;
        self.P.Z = self.S == 0;
    }
}

#[allow(non_snake_case)]
struct Flags {
    pub N: bool,
    pub V: bool,
    pub B: bool,
    pub D: bool,
    pub I: bool,
    pub Z: bool,
    pub C: u8
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            N: false,
            V: false,
            B: false,
            D: false,
            I: false,
            Z: false,
            C: 0
        }
    }

    pub fn from_byte(flag: u8) -> Flags {
        let mut f = Flags::new();
        f.set(flag);
        f
    }

    pub fn set(&mut self, flags: u8) {
        self.N = (flags & 0x80) != 0;
        self.V = (flags & 0x40) != 0;
        self.B = (flags & 0x10) != 0;
        self.D = (flags & 0x08) != 0;
        self.I = (flags & 0x04) != 0;
        self.Z = (flags & 0x02) != 0;
        self.C = flags & 0x01;
    }

    pub fn get(&self) -> u8 {
        let mut result: u8 = self.C;
        if self.Z {
            result |= 0x02;
        }
        if self.I {
            result |= 0x04;
        }
        if self.D {
            result |= 0x08;
        }
        if self.B {
            result |= 0x10;
        }
        if self.V {
            result |= 0x40;
        }
        if self.N {
            result |= 0x80;
        }
        result
    }
}
