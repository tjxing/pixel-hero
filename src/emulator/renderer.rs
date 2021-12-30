use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};
use js_sys::Math::random;
use crate::init::{RAW_WIDTH, RAW_HEIGHT};

const CHANNELS: usize = 4;
const DATA_LEN: usize = RAW_WIDTH * RAW_HEIGHT * CHANNELS;

pub struct Renderer {
    ctx: CanvasRenderingContext2d,
    data: [u8; DATA_LEN]
}

impl Renderer {
    pub fn new(ctx: CanvasRenderingContext2d) -> Renderer {
        Renderer {
            ctx,
            data: [0; DATA_LEN]
        }
    }

    pub fn set(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        let index = CHANNELS * RAW_WIDTH * y + CHANNELS * x;
        self.data[index] = r;
        self.data[index + 1] = g;
        self.data[index + 2] = b;
        self.data[index + 3] = 255;
    }

    pub fn render(&self) {
        let data = Clamped::<&[u8]>(&self.data);
        let image = ImageData::new_with_u8_clamped_array_and_sh(data,
            RAW_WIDTH as u32,
            RAW_HEIGHT as u32).unwrap();
        self.ctx.put_image_data(&image, 0.0, 0.0).unwrap();
    }
}