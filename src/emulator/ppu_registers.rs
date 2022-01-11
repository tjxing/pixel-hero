// PPU_CTRL
pub struct PPUControl {
    nt_base: u16,
    vram_step: u16,
    large_sprite: bool,
    sprite_pattern: u16,
    background_pattern: u16,
    nmi: bool
}

impl PPUControl {
    pub fn new() -> PPUControl {
        PPUControl {
            nt_base: 0x2000,
            vram_step: 1,
            large_sprite: false,
            sprite_pattern: 0,
            background_pattern: 0,
            nmi: false
        }
    }

    pub fn set(&mut self, value: u8) {
        self.nt_base = 0x2000 + 0x0400 * (value & 0x03) as u16;
        self.vram_step = if value & 0x04 == 0 { 1 } else { 32 };
        self.large_sprite = (value & 0x20) > 0;
        if !self.large_sprite {
            self.sprite_pattern = if value & 0x08 == 0 { 0 } else { 0x1000 };
        }
        self.background_pattern = if value & 0x10 == 0 { 0 } else { 0x1000 };
        self.nmi = value & 0x80 > 0;
    }

    pub fn nt_base(&self) -> u16 {
        self.nt_base
    }

    pub fn vram_step(&self) -> u16 {
        self.vram_step
    }

    pub fn large_sprite(&self) -> bool {
        self.large_sprite
    }

    pub fn sprite_pattern(&self) -> u16 {
        self.sprite_pattern
    }

    pub fn background_pattern(&self) -> u16 {
        self.background_pattern
    }

    pub fn nmi(&self) -> bool {
        self.nmi
    }
}

// PPU_Mask
pub struct PPUMask {
    normal_color: bool,
    show_background_left: bool,
    show_sprite_left: bool,
    show_background: bool,
    show_sprite: bool
}

impl PPUMask {
    pub fn new() -> PPUMask {
        PPUMask {
            normal_color: true,
            show_background_left: false,
            show_sprite_left: false,
            show_background: false,
            show_sprite: false
        }
    }

    pub fn set(&mut self, value: u8) {
        self.normal_color = value & 0x01 == 0;
        self.show_background_left = value & 0x02 > 0;
        self.show_sprite_left = value & 0x04 > 0;
        self.show_background = value & 0x08 > 0;
        self.show_sprite = value & 0x10 > 0;
    }

    pub fn normal_color(&self) -> bool {
        self.normal_color
    }

    pub fn show_background_left(&self) -> bool {
        self.show_background_left
    }

    pub fn show_sprite_left(&self) -> bool {
        self.show_sprite_left
    }

    pub fn show_background(&self) -> bool {
        self.show_background
    }

    pub fn show_sprite(&self) -> bool {
        self.show_sprite
    }
}

// PPU_Status
pub struct PPUStatus {
    value: u8,
    vertical_blank: bool
}

impl PPUStatus {
    pub fn new() -> PPUStatus {
        PPUStatus {
            value: 0,
            vertical_blank: false
        }
    }

    pub fn set(&mut self, value: u8) {
        self.value = value;
        self.vertical_blank = value & 0x80 > 0;
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn vertical_blank(&self) -> bool {
        self.vertical_blank
    }

    pub fn set_vertical_blank(&mut self, value: bool) {
        self.vertical_blank = value;
        if value {
            self.value |= 0x80;
        } else {
            self.value &= 0x7F;
        }
    }

    pub fn set_sprite_overflow(&mut self, value: bool) {
        if value {
            self.value |= 0x20;
        } else {
            self.value &= 0xDF;
        }
    }

    pub fn set_sprite_0_hit(&mut self, value: bool) {
        if value {
            self.value |= 0x40;
        } else {
            self.value &= 0xBF;
        }
    }
}