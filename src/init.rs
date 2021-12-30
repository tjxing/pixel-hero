use wasm_bindgen::prelude::{JsValue, wasm_bindgen, Closure};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlCanvasElement, HtmlDivElement, Document, CanvasRenderingContext2d};
use js_sys::Function;
use crate::conf::Configuration;
use crate::i18n::Message;
use crate::emulator::{Emulator, new_emulator};

pub const RAW_WIDTH: usize = 256;
pub const RAW_HEIGHT: usize = 240;

#[wasm_bindgen]
pub fn create_emulator(element: &HtmlElement, o: &JsValue) -> Emulator {
    console_error_panic_hook::set_once();
    if !o.is_object() {
        panic!("Input configuration is not an object.")
    }

    let conf = Configuration::new(o);
    let ctx = init_canvas(element, &conf).unwrap();
    new_emulator(ctx, conf)
}

fn init_canvas(element: &HtmlElement, conf: &Configuration)
    -> Result<CanvasRenderingContext2d, JsValue> {

    let document = document(conf);
    element.set_inner_text("");

    let div = create_div(&document, conf)?;
    let div_style = div.style();
    div_style.set_property("display", "flex")?;
    div_style.set_property("align-items", "center")?;
    div_style.set_property("justify-content", "center")?;
    div_style.set_property("padding", "0px")?;
    div_style.set_property("margin", "0px")?;
    div_style.set_property("min-width", std::format!("{}px", 2 * RAW_WIDTH).as_str())?;
    div_style.set_property("min-height", std::format!("{}px", 2 * RAW_HEIGHT).as_str())?;
    div_style.set_property("width", "100%")?;
    div_style.set_property("background-color", "#082E54")?;
    div_style.set_property("align-items", "center")?;
    element.append_child(&div)?;

    let canvas = create_canvas(&document, conf)?;
    canvas.set_width(RAW_WIDTH as u32);
    canvas.set_height(RAW_HEIGHT as u32);
    let canvas_style = canvas.style();
    canvas_style.set_property("min-width", std::format!("{}px", 2 * RAW_WIDTH).as_str())?;
    canvas_style.set_property("min-height", std::format!("{}px", 2 * RAW_HEIGHT).as_str())?;
    canvas_style.set_property("padding", "0px")?;
    canvas_style.set_property("margin", "0px")?;
    canvas_style.set_property("image-rendering", "pixelated")?;
    div.append_child(&canvas)?;

    let ctx = get_context_2d(&canvas, conf);

    let window = web_sys::window().unwrap();
    let resize = make_resize_callback(div, canvas).into_js_value();
    let resize_func = resize.unchecked_ref::<Function>();
    window.add_event_listener_with_callback("resize", resize_func)?;
    resize_func.call0(window.as_ref())?;

    ctx
}

fn make_resize_callback(div: HtmlDivElement, canvas: HtmlCanvasElement) -> Closure<dyn FnMut()> {
    Closure::wrap(Box::new(move || {
        let window = web_sys::window().unwrap();
        let win_size = (
            window.inner_width().unwrap().as_f64().unwrap(),
            window.inner_height().unwrap().as_f64().unwrap(),
        );
        let div_size = (div.offset_width() as f64, div.offset_height() as f64);

        let mut width = div_size.0;
        if width > win_size.0 {
            width = win_size.0;
        }
        if width < 2.0 * RAW_WIDTH as f64 {
            width = 2.0 * RAW_WIDTH as f64;
        }
        let h = (width * RAW_HEIGHT as f64 / RAW_WIDTH as f64).floor();
        let mut height = h;
        if height > win_size.1 {
            height = win_size.1;
        }
        if height < 2.0 * RAW_HEIGHT as f64 {
            height = 2.0 * RAW_HEIGHT as f64;
        }
        if h as u32 != height as u32 {
            width = (height * RAW_WIDTH as f64 / RAW_HEIGHT as f64).floor();
        }
        let style = canvas.style();
        style.set_property("width", std::format!("{}px", width as u32).as_str()).unwrap();
        style.set_property("height", std::format!("{}px", height as u32).as_str()).unwrap();
    }) as Box<dyn FnMut()>)
}

fn document(conf: &Configuration) -> Document {
    let window = web_sys::window()
        .expect(conf.i18n().to_string(Message::GlobalObjectNotExists("window")).as_str());
    window.document()
        .expect(conf.i18n().to_string(Message::GlobalObjectNotExists("document")).as_str())
}

fn create_div(document: &Document, conf: &Configuration) -> Result<HtmlDivElement, JsValue> {
    let element = document.create_element("div")?;
    element.dyn_into::<HtmlDivElement>()
        .map_err(|_| {
            let msg = conf.i18n().to_string(Message::CreateElementError("div"));
            JsValue::from_str(msg.as_str())
        })
}

fn create_canvas(document: &Document, conf: &Configuration) -> Result<HtmlCanvasElement, JsValue> {
    let element = document.create_element("canvas")?;
    element.dyn_into::<HtmlCanvasElement>()
        .map_err(|_| {
            let msg = conf.i18n().to_string(Message::CreateElementError("canvas"));
            JsValue::from_str(msg.as_str())
        })
}

fn get_context_2d(canvas: &HtmlCanvasElement, conf: &Configuration)
    -> Result<CanvasRenderingContext2d, JsValue> {

    match canvas.get_context("2d") {
        Ok(c) => {
            match c {
                None => Err(JsValue::from_str(conf.i18n()
                    .to_string(Message::CanvasContextError).as_str())),
                Some(ctx) => Ok(ctx.dyn_into::<CanvasRenderingContext2d>()?)
            }
        },
        Err(e) => Err(e)
    }
}
