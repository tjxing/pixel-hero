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
    data: [u8; DATA_LEN]
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
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: u8) {
        if x < RAW_WIDTH && y < RAW_HEIGHT {
            let index = CHANNELS * RAW_WIDTH * y + CHANNELS * x;
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