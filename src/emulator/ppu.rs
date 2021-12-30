use web_sys::CanvasRenderingContext2d;
use super::renderer::Renderer;
use std::rc::Rc;
use std::cell::RefCell;
use crate::log::console_log;

const PPUCTRL: usize = 0;
const PPUMASK: usize = 1;
const PPUSTATUS: usize = 2;
const OAMADDR: usize = 3;
const OAMDATA: usize = 4;
const PPUSCROLL: usize = 5;
const PPUADDR: usize = 6;
const PPUDATA: usize = 7;

const SCANLINE_CLK: i32 = 341;

const K: usize = 1024;

struct ScrollPos {
    x: u8,
    y: u8,
    to_x: bool
}

struct Address {
    addr: u16,
    high: bool
}

pub struct PPU {
    renderer: Renderer,
    memory: [u8; 16 * K],
    oam: [u8; 256],
    wait_cpu: bool,
    even: bool,
    ppu_ctrl: u8,
    ppu_mask: u8,
    ppu_status: u8,
    oam_addr: u8,
    oam_index: usize,
    scroll: ScrollPos,
    address: Address,
    vram_step: u16,
}

impl PPU {
    pub fn new(ctx: CanvasRenderingContext2d) -> PPU {
        PPU {
            renderer: Renderer::new(ctx),
            oam: [0; 256],
            memory: [0; 16 * K],
            wait_cpu: true,
            even: true,
            ppu_ctrl: 0,
            ppu_mask: 0,
            ppu_status: 0,
            oam_addr: 0,
            oam_index: 0,
            scroll: ScrollPos {
                x: 0,
                y: 0,
                to_x: true
            },
            address: Address {
                addr: 0,
                high: true
            },
            vram_step: 1
        }
    }

    pub fn ticks(&mut self, t: u16) -> bool {
        false
    }

    pub fn read_register(&mut self, index: u8) -> u8 {
        let i = index as usize;
        //console_log(std::format!("read {}", index).as_str());
        match i {
            PPUSTATUS => {
                let status = self.ppu_status;
                self.ppu_status = status & 0x7F;
                status
            },
            OAMDATA => {
                self.oam[self.oam_addr as usize]
            },
            PPUDATA => {
                let v = self.memory[self.address.addr as usize];
                self.address.addr += self.vram_step;
                v
            }
            _ => 0
        }
    }

    pub fn write_register(&mut self, index: u8, v: u8) -> bool {
        let i = index as usize;
        //console_log(std::format!("write {} {}", index, v).as_str());
        match i {
            PPUCTRL => if !self.wait_cpu {
                self.ppu_ctrl = v;
                self.vram_step = if (v & 0x04) == 0 { 1 } else { 32 }
            },
            PPUMASK => if !self.wait_cpu {
                self.ppu_mask = v;
            },
            OAMADDR => {
                self.oam_addr = v;
            },
            OAMDATA => {
                if (self.ppu_status | 0x80) > 0 {
                    self.oam[self.oam_addr as usize] = v;
                }
                self.oam_addr += 1;
            },
            PPUSCROLL => if !self.wait_cpu {
                if self.scroll.to_x {
                    self.scroll.x = v;
                } else {
                    self.scroll.y = v;
                }
                self.scroll.to_x = !self.scroll.to_x;
            },
            PPUADDR => if !self.wait_cpu {
                let high = self.address.high;
                if high {
                    self.address.addr = (v as u16)  << 8;
                } else {
                    self.address.addr |= v as u16;
                }
                self.address.high = !high;
            },
            PPUDATA => {
                self.memory[self.address.addr as usize] = v;
                self.address.addr += self.vram_step;
            }
            _ => {}
        }
        self.ppu_status & self.ppu_ctrl & 0x80 != 0
    }

    pub fn vram_address(&self) -> u16 {
        self.address.addr
    }

    pub fn stop_waiting(&mut self) {
        self.wait_cpu = false;
    }

    pub fn pre_render(&mut self) -> i32 {
        self.ppu_status &= 0x7F;
        self.even = !self.even;

        if self.even {
            SCANLINE_CLK
        } else {
            SCANLINE_CLK - 1
        }

    }

    pub fn visible_render(&mut self) -> i32 {
        if self.ppu_mask & 0x08 > 0 {
            let nt_addr: u16 = match self.ppu_ctrl & 0x03 {
                0 => 0x2000,
                1 => 0x2400,
                2 => 0x2800,
                _ => 0x2C00
            };
            console_log(std::format!("0x{:X}", nt_addr).as_str());
        }
        240 * SCANLINE_CLK
    }

    pub fn post_render(&mut self) -> i32 {
        SCANLINE_CLK
    }

    pub fn start_vbl(&mut self) -> (i32, bool) {
        self.ppu_status |= 0x80;
        (20 * SCANLINE_CLK, self.ppu_ctrl | 0x80 > 0)
    }

    pub fn fill_oam(&mut self, v: u8) {
        self.oam[self.oam_index] = v;
        self.oam_index = (self.oam_index + 1) % 256;
    }

    pub fn draw(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        self.renderer.set(x, y, r, g, b);
    }

    pub fn render(&self) {
        self.renderer.render();
    }

}