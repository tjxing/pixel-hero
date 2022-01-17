use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};
use js_sys::Math::random;
use crate::init::{RAW_WIDTH, RAW_HEIGHT};

const CHANNELS: usize = 4;
const DATA_LEN: usize = RAW_WIDTH * RAW_HEIGHT * CHANNELS;
const COLORS: [u8; 192] = [84,84,84,0,30,116,8,16,144,48,0,136,68,0,100,92,0,48,84,4,0,60,24,0,32,42,0,8,58,0,0,64,0,0,60,0,0,50,60,0,0,0,0,0,0,0,0,0,
    152,150,152,8,76,196,48,50,236,92,30,228,136,20,176,160,20,100,152,34,32,120,60,0,84,90,0,40,114,0,8,124,0,0,118,40,0,102,120,0,0,0,0,0,0,0,0,0,
    236,238,236,76,154,236,120,124,236,176,98,236,228,84,236,236,88,180,236,106,100,212,136,32,160,170,0,116,196,0,76,208,32,56,204,108,56,180,204,60,60,60,0,0,0,0,0,0,
    236,238,236,168,204,236,188,188,236,212,178,236,236,174,236,236,174,212,236,180,176,228,196,144,204,210,120,180,222,120,168,226,144,152,226,180,160,214,228,160,162,160,0,0,0,0,0,0];

pub struct Renderer {
    ctx: CanvasRenderingContext2d,
    data: [u8; DATA_LEN],
    bg_buffer: [u8; RAW_WIDTH], // The color indexes. And 0xFF is transparent point
    sprite_buffer: [(u8, bool); RAW_WIDTH] // (color index, is in front of background)
}

impl Renderer {
    pub fn new(ctx: CanvasRenderingContext2d) -> Renderer {
        let mut data: [u8; DATA_LEN] = [0; DATA_LEN];
        let image_data = ctx.get_image_data(0.0, 0.0, RAW_WIDTH as f64, RAW_HEIGHT as f64)
            .unwrap()
            .data();
        for i in 0..image_data.len() as usize {
            data[i] = *image_data.get(i).unwrap();
        }
        Renderer {
            ctx,
            data,
            bg_buffer: [0xFF; RAW_WIDTH],
            sprite_buffer: [(0xFF, false); RAW_WIDTH]
        }
    }

    pub fn clear_buffer(&mut self) {
        self.bg_buffer.fill(0xFF);
        self.sprite_buffer.fill((0xFF, false));
    }

    pub fn set_background(&mut self, index: u8, value: u8) {
        self.bg_buffer[index as usize] = value;
    }

    pub fn set_sprite(&mut self, index: u8, value: u8, front: bool) {
        self.sprite_buffer[index as usize] = (value, front);
    }

    pub fn is_bg_transparent(&mut self, index: u8) -> bool {
        self.bg_buffer[index as usize] == 0xFF
    }

    pub fn merge_line(&mut self, line: u8, bg_color: u8) {
        for i in 0..RAW_WIDTH {
            let bg = self.bg_buffer[i];
            let sprite = self.sprite_buffer[i];
            let color = if bg == 0xFF {
                if sprite.0 == 0xFF {
                    bg_color
                } else {
                    sprite.0
                }
            } else {
                if sprite.0 < 0xFF && sprite.1 {
                    sprite.0
                } else {
                    bg
                }
            };

            let index = CHANNELS * RAW_WIDTH * line as usize + CHANNELS * i;
            let c = color as usize * 3;
            self.data[index] = COLORS[c];
            self.data[index + 1] = COLORS[c + 1];
            self.data[index + 2] = COLORS[c + 2];
            self.data[index + 3] = 255;
        }

    }

    pub fn render(&self) {
        let data = Clamped::<&[u8]>(&self.data);
        let image = ImageData::new_with_u8_clamped_array_and_sh(data,
            RAW_WIDTH as u32,
            RAW_HEIGHT as u32).unwrap();
        self.ctx.put_image_data(&image, 0.0, 0.0).unwrap();
    }
}