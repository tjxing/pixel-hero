use web_sys::CanvasRenderingContext2d;
use std::rc::Rc;
use crate::conf::Configuration;
use super::cpu::CPU;
use super::ppu::PPU;
use super::memory::Memory;
use super::registers::Registers;
use crate::rom::Rom;
use crate::log::console_log;

pub struct Bus {
    cpu: CPU,
    ppu: PPU,
    memory: Memory,
    registers: Registers,
    rom: Rom,

    nmi_flag: bool,
    dma_clk: u16
}

impl Bus {
    pub fn new(rom: Rom, ctx: Option<CanvasRenderingContext2d>, conf: &Configuration) -> Bus {
        let mut bus = Bus {
            cpu: CPU::new(),
            ppu: PPU::new(ctx, &rom),
            memory: Memory::new(),
            registers: Registers::new(),
            rom,
            nmi_flag: false,
            dma_clk: 0
        };
        let pc = bus.read(0xFFFC) as u16 | (bus.read(0xFFFD) as u16) << 8;
        bus.cpu.goto(pc);
        bus
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        let mark = addr & 0xF000;
        if mark == 0 || mark == 0x1000 {
            self.memory.read(addr)
        } else if mark == 0x2000 || mark == 0x3000 {
            let index = (addr & 0x07) as u8;
            self.ppu.read_register(index, &self.rom)
        } else if addr < 0x401F {
            self.registers.read(addr)
        } else {
            self.rom.mapper().read_prg(addr)
        }
    }

    pub fn write(&mut self, addr: u16, v: u8) {
        let mark = addr & 0xF000;
        if mark == 0 || mark == 0x1000 {
            self.memory.write(addr, v);
        } else if mark == 0x2000 || mark == 0x3000 {
            let index = (addr & 0x07) as u8;
            if self.ppu.write_register(index, v, &mut self.rom) {
                self.nmi_flag = true;
            }
        } else if addr < 0x401F {
            if addr == 0x4014 {
                let base = (v as u16) << 8;
                for i in 0x00..0xFF as u16 {
                    let v = self.read(base | i);
                    if self.ppu.fill_oam(v) == 0 {
                        break;
                    }
                }
                self.dma_clk = 513;
            } else {
                self.registers.write(addr, v);
            }
        } else {
            self.rom.mapper_mut().write_prg(addr, v);
        }
    }

    pub fn read_memory(&self, addr: u8) -> u8 {
        self.memory.read_zero_page(addr)
    }

    pub fn write_memory(&mut self, addr: u8, v: u8) {
        self.memory.write_zero_page(addr, v);
    }

    pub fn current_instruction(&mut self) -> u8 {
        self.read(self.cpu.pc())
    }

    pub fn immediate_map(&mut self) -> u8 {
        self.read(self.cpu.pc() + 1)
    }

    pub fn zero_page_map(&mut self) -> u8 {
        self.immediate_map()
    }

    pub fn zero_page_x_map(&mut self) -> u8 {
        ((self.immediate_map() as u16 + self.cpu.x() as u16) & 0x00FF) as u8
    }

    pub fn zero_page_y_map(&mut self) -> u8 {
        ((self.immediate_map() as u16 + self.cpu.y() as u16) & 0x00FF) as u8
    }

    pub fn absolute_map(&mut self) -> u16 {
        let low = self.read(self.cpu.pc() + 1);
        let high = self.read(self.cpu.pc() + 2);
        (low as u16) | ((high as u16) << 8)
    }

    pub fn absolute_x_map(&mut self) -> (u16, bool) {
        let low = self.read(self.cpu.pc() + 1);
        let temp = low as u16 + self.cpu.x() as u16;
        let high = self.read(self.cpu.pc() + 2);
        let addr = temp + ((high as u16) << 8);
        (addr, temp & 0x0100 != 0)
    }

    pub fn absolute_y_map(&mut self) -> (u16, bool) {
        let low = self.read(self.cpu.pc() + 1);
        let temp = low as u16 + self.cpu.y() as u16;
        let high = self.read(self.cpu.pc() + 2);
        let addr = temp + ((high as u16) << 8);
        (addr, temp & 0x0100 != 0)
    }

    pub fn indexed_indirect_map(&mut self) -> u16 {
        let arg = self.read(self.cpu.pc() + 1) as u16 + self.cpu.x() as u16;
        let low = self.memory.read_zero_page((arg & 0x00FF) as u8);
        let high = self.memory.read_zero_page(((arg + 1) & 0x00FF) as u8);
        (low as u16) | ((high as u16) << 8)
    }

    pub fn indirect_indexed_map(&mut self) -> (u16, bool) {
        let arg = self.read(self.cpu.pc() + 1);
        let low = self.memory.read_zero_page(arg);
        let high = self.memory.read_zero_page(((arg as u16 + 1) & 0x00FF) as u8);
        let temp = low as u16 + self.cpu.y() as u16;
        let addr = temp + ((high as u16) << 8);
        (addr, temp & 0x0100 != 0)
    }

    pub fn relative_map(&mut self) -> i8 {
        let n = self.read(self.cpu.pc() + 1);
        let data: [u8; 1] = [n; 1];
        i8::from_le_bytes(data)
    }

    pub fn push(&mut self, num: u8) {
        let addr = self.cpu.push();
        self.write_memory(addr, num);
    }

    pub fn push_word(&mut self, num: u16) {
        self.push(((num & 0xFF00) >> 8) as u8);
        self.push((num & 0x00FF) as u8);
    }

    pub fn pop(&mut self) -> u8 {
        let addr = self.cpu.pop();
        self.read_memory(addr)
    }

    pub fn pop_word(&mut self) -> u16 {
        let low = self.pop();
        let high = self.pop();
        (low as u16) | ((high as u16) << 8)
    }

    pub fn ppu_ticks(&mut self, t: u8) -> bool {
        let result = self.ppu.ticks(t, &self.rom);
        if result.1 {
            self.nmi_flag = true;
        }
        result.0
    }

    pub fn nmi(&mut self) {
        self.push_word(self.cpu.pc());
        self.push(self.cpu.p());
        let pc = (self.read(0xFFFA) as u16) | ((self.read(0xFFFB) as u16) << 8);
        self.cpu.goto(pc);
        self.nmi_flag = false;
    }

    pub fn check_interrupt(&mut self) {
        if self.nmi_flag {
            self.nmi();
        }
    }

    pub fn check_dma(&mut self) -> u8 {
        if self.dma_clk >= 110 {
            self.dma_clk -= 110;
            110
        } else {
            let clk = self.dma_clk;
            self.dma_clk = 0;
            clk as u8
        }
    }

    pub fn cpu(&self) -> &CPU {
        &self.cpu
    }

    pub fn cpu_mut(&mut self) -> &mut CPU {
        &mut self.cpu
    }

    pub fn ppu(&self) -> &PPU {
        &self.ppu
    }

    pub fn ppu_mut(&mut self) -> &mut PPU {
        &mut self.ppu
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn mock() -> Bus {
        let conf = crate::conf::tests::mock();
        let rom = crate::rom::tests::mock();
        Bus::new(rom, None, &conf)
    }
}