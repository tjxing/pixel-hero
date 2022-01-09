use web_sys::CanvasRenderingContext2d;
use super::renderer::Renderer;
use std::cell::RefCell;
use crate::log::console_log;
use crate::rom::Rom;
use crate::emulator::mirroring::{Mirroring, select_mirroring};
use std::fmt::Write;
use crate::emulator::ppu_registers::{PPUControl, PPUMask, PPUStatus};

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
    secondary_oam: [u8; 32],
    sprite_count: u8,
    palette: [u8; 32],
    wait_cpu: bool,
    even: bool,
    ppu_ctrl: PPUControl,
    ppu_mask: PPUMask,
    ppu_status: PPUStatus,
    oam_addr: u8,
    oam_index: usize,
    scroll: ScrollPos,
    address: Address,
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
            secondary_oam: [0; 32],
            sprite_count: 0,
            palette: [0; 32],
            memory: [0; 2 * K],
            wait_cpu: true,
            even: true,
            ppu_ctrl: PPUControl::new(),
            ppu_mask: PPUMask::new(),
            ppu_status: PPUStatus::new(),
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
                        self.ppu_status.set_vertical_blank(false);
                        self.ppu_status.set_sprite_overflow(false);
                    }
                },
                PHRASE_VISIBLE_RENDER => {
                    let tick = (self.clk_counter % SCANLINE_CLK) as u16;
                    if tick > 0 {
                        let line = (self.clk_counter / SCANLINE_CLK) as u16;

                        if self.ppu_mask.show_background() && tick < 257 && (tick % 8) == 0 {
                            let nt_offset_x = (tick - 1) / 8;
                            let nt_offset_y = line / 8;

                            let attr = self.read(
                                self.ppu_ctrl.nt_base() + 960 + nt_offset_y / 4 * 8 + nt_offset_x / 4,
                                &rom);
                            let attr_offset = nt_offset_x % 4 / 2 * 4 + nt_offset_y % 4 / 2 * 2;
                            let palette_index= (attr >> attr_offset) & 0x03;
                            let palette_addr = 0x3F01 + 4 * palette_index as u16;

                            let pattern_index = self.read(
                                self.ppu_ctrl.nt_base() + nt_offset_y * 32 + nt_offset_x,
                                rom);
                            let pattern_addr = self.ppu_ctrl.background_pattern()
                                + pattern_index as u16 * 16 + line % 8;
                            let pattern_low = self.read(pattern_addr, rom);
                            let pattern_high = self.read(pattern_addr + 8, rom);
                            for i in 0..8 {
                                let c = (((pattern_high >> (7 - i)) & 0x01) << 1)
                                    | ((pattern_low >> (7 - i)) & 0x01);
                                let color: u8 = if c == 0 {
                                    0x0F
                                } else {
                                    self.read(palette_addr + c as u16 - 1, rom)
                                };

                                self.renderer.set((nt_offset_x * 8 + i) as usize,
                                                  line as usize, color);
                            }
                        }

                        if self.ppu_mask.show_sprite() && line > 0 {
                            if tick == 65 {
                                self.detect_sprite(line as u8);
                                console_log(std::format!("{}", self.sprite_count).as_str());
                            } else if tick > 256 && self.sprite_count > 0 {
                                self.sprite_count -= 1;

                                let y = self.secondary_oam[4 * self.sprite_count as usize] + 1;
                                let pattern_index = self.secondary_oam[4 * self.sprite_count as usize + 1];
                                let sprite_attr = self.secondary_oam[4 * self.sprite_count as usize + 2];
                                let x = self.secondary_oam[4 * self.sprite_count as usize + 3];

                                let pattern_addr = self.ppu_ctrl.sprite_pattern()
                                    + pattern_index as u16 * 16 + line - y as u16;
                                let pattern_low = self.read(pattern_addr, rom);
                                let pattern_high = self.read(pattern_addr + 8, rom);
                                let palette_addr = 0x3F11 + 4 * (sprite_attr & 0x03) as u16;
                                for i in 0..8 {
                                    let c = (((pattern_high >> (7 - i)) & 0x01) << 1)
                                        | ((pattern_low >> (7 - i)) & 0x01);
                                    let color: u8 = if c == 0 {
                                        0x0F
                                    } else {
                                        self.read(palette_addr + c as u16 - 1, rom)
                                    };

                                    self.renderer.set((x + i) as usize, line as usize, color);
                                }
                            }
                        }
                    }
                },
                PHRASE_POST_RENDER => {},
                PHRASE_START_VBL => {
                    if self.clk_counter == 1 {
                        self.ppu_status.set_vertical_blank(true);
                        nmi = self.ppu_ctrl.nmi();
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
                    PHRASE_VISIBLE_RENDER => 240 * SCANLINE_CLK,
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

    fn detect_sprite(&mut self, line: u8) {
        self.sprite_count = 0;
        for i in 0..64 as usize {
            let y = self.oam[4 * i] + 1;
            if line >= y && (line - y) < 8 {
                if self.sprite_count >= 8 {
                    self.ppu_status.set_sprite_overflow(true);
                    break;
                } else {
                    for j in 0..4 as usize {
                        self.secondary_oam[4 * self.sprite_count as usize + j] = self.oam[4 * i + j];
                    }
                    self.sprite_count += 1;
                }
            }
        }
    }

    pub fn read_register(&mut self, index: u8) -> u8 {
        let i = index as usize;
        //console_log(std::format!("read {}", index).as_str());
        match i {
            PPUSTATUS => {
                let status = self.ppu_status.value();
                self.ppu_status.set_vertical_blank(false);
                status
            },
            OAMDATA => {
                self.oam[self.oam_addr as usize]
            },
            PPUDATA => {
                let v = self.memory[self.address.addr as usize];
                self.address.addr += self.ppu_ctrl.vram_step();
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
                self.ppu_ctrl.set(v);
            },
            PPUMASK => if !self.wait_cpu {
                self.ppu_mask.set(v);
            },
            OAMADDR => {
                self.oam_addr = v;
            },
            OAMDATA => {
                if self.ppu_status.vertical_blank() {
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
                self.address.addr += self.ppu_ctrl.vram_step();
            }
            _ => {}
        }
        self.ppu_status.vertical_blank() && self.ppu_ctrl.nmi()
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