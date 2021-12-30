use web_sys::CanvasRenderingContext2d;
use super::renderer::Renderer;
use std::rc::Rc;
use std::cell::RefCell;
use crate::log::console_log;
use crate::rom::Rom;
use crate::emulator::mirroring::{Mirroring, select_mirroring};
use std::fmt::Write;

const PPUCTRL: usize = 0;
const PPUMASK: usize = 1;
const PPUSTATUS: usize = 2;
const OAMADDR: usize = 3;
const OAMDATA: usize = 4;
const PPUSCROLL: usize = 5;
const PPUADDR: usize = 6;
const PPUDATA: usize = 7;

type Phrase = u8;
const PHRASE_PRE_RENDER: Phrase = 0;
const PHRASE_VISIBLE_RENDER: Phrase = 1;
const PHRASE_POST_RENDER: Phrase = 2;
const PHRASE_START_VBL: Phrase = 3;

const SCANLINE_CLK: u32 = 341;

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
    memory: [u8; 2 * K],
    oam: [u8; 256],
    palette: [u8; 32],
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
    phrase: Phrase,
    phrase_clk: u32,
    clk_counter: u32,
    mirroring: Mirroring
}

impl PPU {
    pub fn new(ctx: CanvasRenderingContext2d, rom: &Rom) -> PPU {
        PPU {
            renderer: Renderer::new(ctx),
            oam: [0; 256],
            palette: [0; 32],
            memory: [0; 2 * K],
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
            vram_step: 1,
            phrase: PHRASE_PRE_RENDER,
            phrase_clk: SCANLINE_CLK,
            clk_counter: 0,
            mirroring: select_mirroring(&rom)
        }
    }

    // Return (end-of-frame, nmi)
    pub fn ticks(&mut self, t: u8, rom: &Rom) -> (bool, bool) {
        let mut end_frame = false;
        let mut nmi = false;
        for _tick in 0..t {
            match self.phrase {
                PHRASE_PRE_RENDER => {
                    if self.clk_counter == 1 {
                        self.ppu_status &= 0x7F;
                    }
                },
                PHRASE_VISIBLE_RENDER => {
                    let line = (self.clk_counter / SCANLINE_CLK) as u16;
                    let tick = (self.clk_counter % SCANLINE_CLK) as u16;
                    if self.clk_counter == 0 {
                        let mut updated = false;
                        if self.ppu_mask & 0x08 > 0 {
                            let nt_base = (self.ppu_ctrl as u16 & 0x03) * 0x0400 + 0x2000;
                            let p_base: u16 = if self.ppu_ctrl & 0x10 == 0 {
                                0
                            } else {
                                0x1000
                            };
                            for i in 0..30 as u16 {
                                for j in 0..32 as u16 {
                                    let attr = self.read(nt_base + 960 + i / 4 * 8 + j / 4, &rom);
                                    let y = i % 4 / 2;
                                    let x = j % 4 / 2;
                                    let index = if x == 0 && y == 0 {
                                        attr & 0x03
                                    } else if x == 0 && y == 1 {
                                        (attr & 0x0C) >> 2
                                    } else if x == 1 && y == 0 {
                                        (attr & 0x30) >> 4
                                    } else {
                                        (attr & 0xC0) >> 6
                                    };
                                    let p_addr = 0x3F01 + 4 * index as u16;
                                    let colors: [u8; 3] = [
                                        self.read(p_addr, rom),
                                        self.read(p_addr + 1, rom),
                                        self.read(p_addr + 2, rom)
                                    ];

                                    let nt = self.read(nt_base + 32 * i + j, &rom);
                                    let t_base = p_base + nt as u16 * 16;
                                    for n in 0..8 as u16 {
                                        let l_addr = t_base + n;
                                        let l = self.read(l_addr, &rom);
                                        let h = self.read(l_addr + 8, &rom);
                                        for m in 0..8 {
                                            let p = (((h >> (7 - m)) & 0x01) << 1)
                                                | ((l >> (7 - m)) & 0x01);
                                            let c = if p == 0 {
                                                0x0F
                                            } else {
                                                colors[p as usize - 1]
                                            };
                                            self.renderer.set((j * 8 + m) as usize, (i * 8 + n) as usize, c);
                                        }
                                    }
                                }
                            }
                            updated = true;
                        }
                        if self.ppu_mask & 0x10 > 0 {
                            for i in 0..64 as u8 {
                                let s = 4 * i;
                                let y = self.oam[s as usize];
                                let x = self.oam[(s + 3) as usize];
                                if self.ppu_ctrl & 0x20 == 0 {
                                    let p_base: u16 = if self.ppu_ctrl & 0x08 == 0 {
                                        0
                                    } else {
                                        0x1000
                                    };
                                    let t_base = p_base + self.oam[s as usize + 1] as u16 * 16;
                                    let index = self.oam[s as usize + 2] & 0x02;
                                    let p_addr = 0x3F11 + 4 * index as u16;
                                    let colors: [u8; 3] = [
                                        self.read(p_addr, rom),
                                        self.read(p_addr + 1, rom),
                                        self.read(p_addr + 2, rom)
                                    ];
                                    for n in 0..8 as u8 {
                                        let l_addr = t_base + n as u16;
                                        let l = self.read(l_addr, rom);
                                        let h = self.read(l_addr + 8, rom);
                                        for m in 0..8 as u8 {
                                            let p = (((h >> (7 - m)) & 0x01) << 1)
                                                | ((l >> (7 - m)) & 0x01);
                                            if p > 0 {
                                                self.renderer.set(
                                                    (x + m) as usize,
                                                    (y + n) as usize,
                                                    colors[p as usize - 1]);
                                            }
                                        }
                                    }

                                } else {

                                }
                            }
                            updated = true;
                        }
                        if updated {
                            self.renderer.render();
                        }
                    }
                },
                PHRASE_POST_RENDER => {},
                PHRASE_START_VBL => {
                    if self.clk_counter == 1 {
                        self.ppu_status |= 0x80;
                        nmi = (self.ppu_ctrl & 0x80) > 0;
                    }
                },
                _ => panic!("Invalid PPU phrase.")
            };

            self.clk_counter += 1;
            if self.clk_counter == self.phrase_clk {
                self.phrase = (self.phrase + 1) % 4;
                self.clk_counter = 0;
                self.phrase_clk = match self.phrase {
                    PHRASE_PRE_RENDER => {
                        end_frame = true;
                        self.even = !self.even;
                        if self.even {
                            SCANLINE_CLK
                        } else {
                            SCANLINE_CLK - 1
                        }
                    },
                    PHRASE_VISIBLE_RENDER => 340 * SCANLINE_CLK,
                    PHRASE_POST_RENDER => {
                        self.renderer.render();
                        SCANLINE_CLK
                    },
                    PHRASE_START_VBL => 20 * SCANLINE_CLK,
                    _ => panic!("Invalid PPU phrase.")
                };
            }
        }

        (end_frame, nmi)
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

    pub fn write_register(&mut self, index: u8, v: u8, rom: &mut Rom) -> bool {
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
                self.write(self.address.addr, v, rom);
                self.address.addr += self.vram_step;
            }
            _ => {}
        }
        self.ppu_status & self.ppu_ctrl & 0x80 != 0
    }

    fn read(&self, addr: u16, rom: &Rom) -> u8 {
        let mark = addr & 0xF000;
        if mark == 0x1000 {
            rom.mapper().read_chr(addr)
        } else if addr < 0x3EFF {
            let address = addr & 0x0FFF;
            (self.mirroring.read())(address, &self.memory, &rom)
        } else {
            self.palette[(addr & 0x001F) as usize]
        }
    }

    fn write(&mut self, addr: u16, v: u8, rom: &mut Rom) {
        let mark = addr & 0xF000;
        if mark == 0x1000 {
            rom.mapper_mut().write_chr(addr, v);
        } else if addr < 0x3EFF {
            let address = addr & 0x0FFF;
            (self.mirroring.write())(address, v, &mut self.memory, rom)
        } else {
            self.palette[(addr & 0x001F) as usize] = v;
        }
    }

    pub fn vram_address(&self) -> u16 {
        self.address.addr
    }

    pub fn stop_waiting(&mut self) {
        self.wait_cpu = false;
    }

    pub fn fill_oam(&mut self, v: u8) {
        self.oam[self.oam_index] = v;
        self.oam_index = (self.oam_index + 1) % 256;
    }

}