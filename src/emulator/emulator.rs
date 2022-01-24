use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use web_sys::{CanvasRenderingContext2d, ImageData, window};
use crate::conf::Configuration;
use crate::i18n::Message;
use crate::rom::{Rom, Timing};
use super::bus::Bus;
use crate::emulator::instruction::{InstructionSet, Instruction};
use js_sys::Function;
use wasm_bindgen::{JsCast, Clamped};
use js_sys::Math::random;
use crate::init::{RAW_WIDTH, RAW_HEIGHT};

#[wasm_bindgen]
pub struct Emulator {
    conf: Configuration,
    screen: CanvasRenderingContext2d,
    frame: Option<Closure<dyn FnMut()>>,
    timer: Option<i32>
}

#[wasm_bindgen]
impl Emulator {
    pub fn insert(&mut self, cartridge: Box<[u8]>) {
        if self.frame.is_some() {
            panic!("{}", self.conf.i18n().to_string(Message::CartridgeAlreadyInserted));
        }

        let rom = Rom::parse(cartridge).map_err(|e| {
            self.conf.i18n().to_string(e)
        }).unwrap();
        let fps = make_fps(&self.conf, &rom);
        let mut bus = Bus::new(rom, Some(self.screen.clone()), &self.conf);
        let inst = InstructionSet::new();

        wait_ppu(&mut bus, &inst);

        let frame = make_frame(bus, inst);
        self.frame = Some(frame);
        let timer = window().unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                self.frame.as_ref().unwrap().as_ref().unchecked_ref::<Function>(),
                (1000f64 / fps).floor() as i32
            ).unwrap();
        self.timer = Some(timer);
    }

    pub fn stop(&mut self) {
        match self.timer {
            Some(timer) => {
                window().unwrap().clear_interval_with_handle(timer);
                self.timer = None;
            },
            None => ()
        }
        self.frame = None;
    }
}

fn make_fps(conf: &Configuration, rom: &Rom) -> f64 {
    match conf.fps() {
        Some(f) => if f < 30f64 {
            30f64
        } else if f > 100f64 {
            100f64
        } else {
            f
        },
        None => match rom.timing() {
            Timing::NTSC => 60.0988,
            Timing::PAL | Timing::Dendy => 50.007,
            Timing::MultipleRegion => 60.0985
        }
    }
}

fn current_instruction<'a>(bus: &mut Bus, instructions: &'a InstructionSet) -> &'a Instruction {
    let pc = bus.cpu().pc();
    let code = bus.read(pc);
    instructions.find_instruction(code)
}

fn wait_ppu(bus: &mut Bus, instructions: &InstructionSet) {
    const CYCLES: i32 = 29658;
    let mut c = CYCLES;
    while c > 0 {
        let inst = current_instruction(bus, instructions);
        c -= inst.apply(bus) as i32;
    }
    bus.ppu_mut().stop_waiting();
}

fn make_frame(mut bus: Bus, instructions: InstructionSet) -> Closure<dyn FnMut()> {
    Closure::wrap(Box::new(move || {
        loop {
            let dma_clk = bus.check_dma();
            let finish = if dma_clk > 0 {
                bus.ppu_ticks(3 * dma_clk)
            } else {
                bus.check_interrupt();
                let inst = current_instruction(&mut bus, &instructions);
                let cpu_cycles = inst.apply(&mut bus);
                bus.ppu_ticks(3 * cpu_cycles)
            };
            if finish {
                break;
            }
        }
    }) as Box<dyn FnMut()>)
}

pub fn new_emulator(ctx: CanvasRenderingContext2d, conf: Configuration) -> Emulator {
    draw_splash(&ctx, &conf);
    Emulator {
        conf,
        screen: ctx,
        frame: None,
        timer: None
    }
}

fn draw_splash(ctx: &CanvasRenderingContext2d, _conf: &Configuration) {
    const DATA_LEN: usize = RAW_WIDTH * RAW_HEIGHT * 4;
    let mut data: [u8; DATA_LEN] = [0; DATA_LEN];
    for i in 0..RAW_WIDTH {
        for j in 0..RAW_HEIGHT {
            let index = 4 * RAW_WIDTH * j + 4 * i;
            let value = (random() * 255f64) as u8;
            for n in 0..3 {
                data[index + n] = value
            }
            data[index + 3] = 255;
        }
    }

    let data = Clamped::<&[u8]>(&data);
    let image = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped::<&[u8]>(&data),
        RAW_WIDTH as u32,
        RAW_HEIGHT as u32).unwrap();
    ctx.put_image_data(&image, 0.0, 0.0).unwrap();
}