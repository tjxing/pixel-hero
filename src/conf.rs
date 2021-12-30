use wasm_bindgen::prelude::*;
use js_sys::Reflect;
use crate::i18n::I18n;

const DEFAULT_LOCALE: &str = "en";

pub struct Configuration {
    i18n: I18n,
    fps: Option<f64>
}

impl Configuration {
    pub fn new(conf: &JsValue) -> Configuration {
        let lang = get_conf_string(conf, "locale").or_else(|| {
            match web_sys::window() {
                None => None,
                Some(w) => w.navigator().language()
            }
        });
        let i18n = match lang {
            None => I18n::new(DEFAULT_LOCALE.to_string()),
            Some(l) => I18n::new (l)
        };

        Configuration {
            i18n,
            fps: get_conf_integer(conf, "fps")
        }
    }

    pub fn i18n(&self) -> &I18n {
        &self.i18n
    }

    pub fn fps(&self) -> Option<f64> {
        self.fps
    }
}

fn get_conf_string(conf: &JsValue, key: &str) -> Option<String> {
    match Reflect::get(conf, &JsValue::from_str(key)) {
        Ok(v) => v.as_string(),
        Err(_) => None
    }
}

fn get_conf_integer(conf: &JsValue, key: &str) -> Option<f64> {
    match Reflect::get(conf, &JsValue::from_str(key)) {
        Ok(v) => v.as_f64(),
        Err(_) => None
    }
}