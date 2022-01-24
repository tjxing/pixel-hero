use web_sys::CanvasRenderingContext2d;
use super::renderer::Renderer;
use crate::rom::Rom;
use crate::emulator::mirroring::{Mirroring, select_mirroring};
use crate::emulator::ppu_registers::{PPUControl, PPUMask, PPUStatus, PPUScroll, PPUAddress};
use crate::emulator::palette::Palette;
use crate::init::{RAW_WIDTH, RAW_HEIGHT};

type Register = (fn(&mut PPU, &Rom) -> u8, fn(&mut PPU, u8, &mut Rom) -> bool);

type Phrase = u8;
const PHRASE_PRE_RENDER: Phrase = 0;
const PHRASE_VISIBLE_RENDER: Phrase = 1;
const PHRASE_POST_RENDER: Phrase = 2;
const PHRASE_START_VBL: Phrase = 3;

const SCANLINE_CLK: u32 = 341;
const K: usize = 1024;


pub struct PPU {
    renderer: Renderer,

    memory: [u8; 2 * K],
    data_buffer: u8,
    palette: Palette,

    oam: [u8; 256],
    secondary_oam: [u8; 32],
    sprite_count: u8,
    sprite_0: bool,
    oam_clear: bool,
    oam_addr: u8,
    oam_index: u8,

    wait_cpu: bool,
    even: bool,

    ppu_ctrl: PPUControl,
    ppu_mask: PPUMask,
    ppu_status: PPUStatus,
    ppu_scroll: PPUScroll,
    ppu_addr: PPUAddress,
    registers: [Register; 8],

    phrase: Phrase,
    phrase_clk: u32,
    clk_counter: u32,

    mirroring: Mirroring
}

impl PPU {
    pub fn new(ctx: Option<CanvasRenderingContext2d>, rom: &Rom) -> PPU {
        PPU {
            renderer: Renderer::new(ctx),
            oam: [0; 256],
            secondary_oam: [0; 32],
            sprite_count: 0,
            sprite_0: false,
            oam_clear: false,
            oam_addr: 0,
            oam_index: 0,
            palette: Palette::new(),
            memory: [0; 2 * K],
            data_buffer: 0,
            wait_cpu: true,
            even: true,
            ppu_ctrl: PPUControl::new(),
            ppu_mask: PPUMask::new(),
            ppu_status: PPUStatus::new(),
            ppu_scroll: PPUScroll::new(),
            ppu_addr: PPUAddress::new(),
            registers: make_register_read_write(),
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
                        self.ppu_status.set_sprite_0_hit(false);
                    }
                },
                PHRASE_VISIBLE_RENDER => {
                    let tick = (self.clk_counter % SCANLINE_CLK) as u16;
                    if tick == 0 {
                        self.renderer.clear_buffer();
                    } else {
                        let line = (self.clk_counter / SCANLINE_CLK) as u16;

                        if self.ppu_mask.show_background() && tick < 257 {
                            let mut x = tick - 1 + self.ppu_scroll.x() as u16;
                            let mut y = line + self.ppu_scroll.y() as u16;
                            let mut nt_base = self.ppu_ctrl.nt_base();
                            if x >= RAW_WIDTH as u16 {
                                x -= RAW_WIDTH as u16;
                                nt_base ^= 0x0400;
                            }
                            if y >= RAW_HEIGHT as u16 {
                                y -= RAW_HEIGHT as u16;
                                nt_base ^= 0x0800;
                            }

                            let nt_offset_x = x / 8;
                            let nt_offset_y = y / 8;

                            let attr = self.read(
                                nt_base + 960 + nt_offset_y / 4 * 8 + nt_offset_x / 4,
                                rom);
                            let attr_offset = nt_offset_x % 4 / 2 * 4 + nt_offset_y % 4 / 2 * 2;
                            let palette_index= (attr >> attr_offset) & 0x03;
                            let palette_addr = 0x3F00 + 4 * palette_index as u16;

                            let pattern_index = self.read(
                                nt_base + nt_offset_y * 32 + nt_offset_x,
                                rom);
                            let pattern_addr = self.ppu_ctrl.background_pattern()
                                + pattern_index as u16 * 16 + y % 8;
                            let pattern_low = self.read(pattern_addr, rom);
                            let pattern_high = self.read(pattern_addr + 8, rom);

                            let pattern_offset = x % 8;
                            let c = (((pattern_high >> (7 - pattern_offset)) & 0x01) << 1)
                                     | ((pattern_low >> (7 - pattern_offset)) & 0x01);
                            if c > 0 {
                                let color = self.palette.read(palette_addr + c as u16);
                                self.renderer.set_background((tick - 1) as u8, color);
                            }
                        }

                        if self.ppu_mask.show_sprite() && line > 0 {
                            if tick == 1 {
                                self.oam_clear = true;
                            } else if tick == 65 {
                                self.oam_clear = false;
                                self.detect_sprite(line as u8);
                            } else if tick > 256 && self.sprite_count > 0 && (tick - 1) % 8 == 0 {
                                self.sprite_count -= 1;

                                let y = self.secondary_oam[4 * self.sprite_count as usize] + 1;
                                let pattern_index = self.secondary_oam[4 * self.sprite_count as usize + 1];
                                let sprite_attr = self.secondary_oam[4 * self.sprite_count as usize + 2];
                                let x = self.secondary_oam[4 * self.sprite_count as usize + 3];

                                let pattern_addr_base = self.ppu_ctrl.sprite_pattern()
                                    + pattern_index as u16 * 16;
                                let pattern_addr = if sprite_attr & 0x80 == 0 {
                                    pattern_addr_base + line - y as u16
                                } else {
                                    pattern_addr_base + y as u16 - line + 7
                                };
                                let pattern_low = self.read(pattern_addr, rom);
                                let pattern_high = self.read(pattern_addr + 8, rom);
                                let palette_addr = 0x3F10 + 4 * (sprite_attr & 0x03) as u16;
                                for i in 0..8 {
                                    let c = if sprite_attr & 0x40 == 0 {
                                        (((pattern_high >> (7 - i)) & 0x01) << 1)
                                            | ((pattern_low >> (7 - i)) & 0x01)
                                    } else {
                                        (((pattern_high >> i) & 0x01) << 1)
                                            | ((pattern_low >> i) & 0x01)
                                    };
                                    if c > 0 {
                                        let color = self.palette.read(palette_addr + c as u16);
                                        let x_index = x as u16 + i as u16;
                                        if x_index < RAW_WIDTH as u16 {
                                            let front = sprite_attr & 0x20 == 0;
                                            self.renderer.set_sprite(x_index as u8, color, front);
                                            if !self.ppu_status.sprite_0_hit()
                                                && self.sprite_0 && self.sprite_count == 0
                                                && front
                                                && !self.renderer.is_bg_transparent(x_index as u8) {
                                                self.ppu_status.set_sprite_0_hit(true);
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if tick == SCANLINE_CLK as u16 - 1
                            && (self.ppu_mask.show_sprite() || self.ppu_mask.show_background()) {
                            self.renderer.merge_line(line as u8, self.palette.read(0x3F00));
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
        self.sprite_0 = false;
        let mut i = self.oam_addr;
        loop {
            let dist = 255 - i;
            if dist < 3 {
                break;
            }
            let y = self.oam[i as usize] + 1;
            if line >= y && (line - y) < 8 {
                if self.sprite_count >= 8 {
                    self.ppu_status.set_sprite_overflow(true);
                    break;
                } else {
                    for j in 0..4 as usize {
                        self.secondary_oam[4 * self.sprite_count as usize + j] = self.oam[i as usize + j];
                    }
                    if i == self.oam_addr {
                        self.sprite_0 = true;
                    }
                    self.sprite_count += 1;
                }
            }
            if dist == 3 {
                break;
            } else {
                i += 4;
            }
        }
    }

    pub fn read_register(&mut self, addr: u16, rom: &Rom) -> u8 {
        let index = addr & 0x07;
        (self.registers[index as usize].0)(self, rom)
    }

    pub fn write_register(&mut self, addr: u16, v: u8, rom: &mut Rom) -> bool {
        let index = addr & 0x07;
        (self.registers[index as usize].1)(self, v, rom)
    }

    fn read(&self, addr: u16, rom: &Rom) -> u8 {
        let mark = addr & 0xF000;
        if mark == 0 || mark == 0x1000 {
            rom.mapper().read_chr(addr)
        } else if addr < 0x3EFF {
            let address = addr & 0x0FFF;
            (self.mirroring.read())(address, &self.memory, &rom)
        } else {
            self.palette.read(addr)
        }
    }

    fn write(&mut self, addr: u16, v: u8, rom: &mut Rom) {
        let mark = addr & 0xF000;
        if mark == 0 || mark == 0x1000 {
            rom.mapper_mut().write_chr(addr, v);
        } else if addr < 0x3EFF {
            let address = addr & 0x0FFF;
            (self.mirroring.write())(address, v, &mut self.memory, rom);
        } else {
            self.palette.write(addr, v);
        }
    }

    pub fn stop_waiting(&mut self) {
        self.wait_cpu = false;
    }

    pub fn fill_oam(&mut self, v: u8) -> u8 {
        self.oam[self.oam_index as usize] = v;
        self.oam_index = if self.oam_index == 255 {
            0
        } else {
            self.oam_index + 1
        };
        self.oam_index
    }

}

fn make_register_read_write() -> [Register; 8] {
    [
        // PPU_CTRL
        (
            |_, _| -> u8 {
                panic!("$2000 is not readable.");
            },
            |ppu, value, _| -> bool {
                if !ppu.wait_cpu {
                    let prev_nmi = ppu.ppu_ctrl.nmi();
                    ppu.ppu_ctrl.set(value);
                    if ppu.ppu_status.vertical_blank() && !prev_nmi && ppu.ppu_ctrl.nmi() {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        ),
        // PPU_MASK
        (
            |_, _| -> u8 {
                panic!("$2001 is not readable.");
            },
            |ppu, value, _| -> bool {
                if !ppu.wait_cpu {
                    ppu.ppu_mask.set(value);
                }
                false
            }
        ),
        // PPU_STATUS
        (
            |ppu, _| -> u8 {
                let status = ppu.ppu_status.value();
                ppu.ppu_status.set_vertical_blank(false);
                ppu.ppu_addr.reset();
                status
            },
            |_, _, _| {
                panic!("$2002 is not writable.");
            }
        ),
        // OAM_ADDR
        (
            |_, _| -> u8 {
                panic!("$2003 is not readable.");
            },
            |ppu, value, _| -> bool {
                ppu.oam_addr = value;
                ppu.oam_index = value;
                false
            }
        ),
        // OAM_DATA
        (
            |ppu, _| -> u8 {
                if ppu.oam_clear {
                    0xFF
                } else {
                    ppu.oam[ppu.oam_addr as usize]
                }
            },
            |ppu, value, _| -> bool {
                if ppu.ppu_status.vertical_blank() {
                    ppu.oam[ppu.oam_addr as usize] = value;
                }
                ppu.oam_addr += 1;
                false
            }
        ),
        // PPU_SCROLL
        (
            |_, _| -> u8 {
                panic!("$2005 is not readable.");
            },
            |ppu, value, _| -> bool {
                if !ppu.wait_cpu {
                    ppu.ppu_scroll.write(value);
                }
                false
            }
        ),
        // PPU_ADDR
        (
            |_, _| -> u8 {
                panic!("$2006 is not readable.");
            },
            |ppu, value, _| -> bool {
                if !ppu.wait_cpu {
                    ppu.ppu_addr.write(value);
                }
                false
            }
        ),
        // PPU_DATA
        (
            |ppu, rom| -> u8 {
                let v = ppu.read(ppu.ppu_addr.addr(), rom);
                let buffer = ppu.data_buffer;
                ppu.data_buffer = v;
                ppu.ppu_addr.go_forward_mirroring(ppu.ppu_ctrl.vram_step());
                buffer
            },
            |ppu, value, rom| {
                ppu.write(ppu.ppu_addr.addr(), value, rom);
                ppu.ppu_addr.go_forward(ppu.ppu_ctrl.vram_step());
                false
            }
        )
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_ctrl_read() {
        let rom = crate::rom::tests::mock();
        let mut ppu = PPU::new(None, &rom);
        ppu.read_register(0x2000, &rom);
    }

    #[test]
    fn test_ctrl_write() {
        let mut rom = crate::rom::tests::mock();
        let mut ppu = PPU::new(None, &rom);

        ppu.write_register(0x3000, 0xAA, &mut rom);
        assert_eq!(ppu.ppu_ctrl.nmi(), false);

        ppu.stop_waiting();

        ppu.write_register(0x3000, 0xAA, &mut rom);
        assert_eq!(ppu.ppu_ctrl.nmi(), true);
        assert_eq!(ppu.ppu_ctrl.large_sprite(), true);
        assert_eq!(ppu.ppu_ctrl.background_pattern(), 0);
        assert_eq!(ppu.ppu_ctrl.vram_step(), 1);
        assert_eq!(ppu.ppu_ctrl.nt_base(), 0x2800);

        ppu.write_register(0x3000, 0x55, &mut rom);
        assert_eq!(ppu.ppu_ctrl.nmi(), false);
        assert_eq!(ppu.ppu_ctrl.large_sprite(), false);
        assert_eq!(ppu.ppu_ctrl.background_pattern(), 0x1000);
        assert_eq!(ppu.ppu_ctrl.sprite_pattern(), 0);
        assert_eq!(ppu.ppu_ctrl.vram_step(), 32);
        assert_eq!(ppu.ppu_ctrl.nt_base(), 0x2400);
    }

    #[test]
    #[should_panic]
    fn test_mask_read() {
        let rom = crate::rom::tests::mock();
        let mut ppu = PPU::new(None, &rom);
        ppu.read_register(0x2101, &rom);
    }

    #[test]
    fn test_mask_write() {
        let mut rom = crate::rom::tests::mock();
        let mut ppu = PPU::new(None, &rom);

        ppu.write_register(0x3001, 0xAA, &mut rom);
        assert_eq!(ppu.ppu_mask.show_background(), false);

        ppu.stop_waiting();

        ppu.write_register(0x3001, 0xAA, &mut rom);
        assert_eq!(ppu.ppu_mask.show_background(), true);
        assert_eq!(ppu.ppu_mask.show_sprite_left(), false);
        assert_eq!(ppu.ppu_mask.show_background_left(), true);
        assert_eq!(ppu.ppu_mask.normal_color(), true);
    }

    #[test]
    fn test_status_read() {
        let rom = crate::rom::tests::mock();
        let mut ppu = PPU::new(None, &rom);
        let status = ppu.read_register(0x2002, &rom);
        assert_eq!(status, 0);
    }

    #[test]
    #[should_panic]
    fn test_status_write() {
        let mut rom = crate::rom::tests::mock();
        let mut ppu = PPU::new(None, &rom);
        ppu.write_register(0x2002, 0, &mut rom);
    }

    #[test]
    #[should_panic]
    fn test_scroll_read() {
        let rom = crate::rom::tests::mock();
        let mut ppu = PPU::new(None, &rom);
        ppu.read_register(0x2005, &rom);
    }

    #[test]
    fn test_scroll_write() {
        let mut rom = crate::rom::tests::mock();
        let mut ppu = PPU::new(None, &rom);

        ppu.write_register(0x2005, 1, &mut rom);
        assert_eq!(ppu.ppu_scroll.x(), 0);

        ppu.stop_waiting();

        ppu.write_register(0x2005, 1, &mut rom);
        ppu.write_register(0x2005, 2, &mut rom);
        assert_eq!(ppu.ppu_scroll.x(), 1);
        assert_eq!(ppu.ppu_scroll.y(), 2);

        ppu.write_register(0x2005, 3, &mut rom);
        ppu.write_register(0x2005, 4, &mut rom);
        assert_eq!(ppu.ppu_scroll.x(), 3);
        assert_eq!(ppu.ppu_scroll.y(), 4);
    }
}