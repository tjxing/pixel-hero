pub enum Message {
    GlobalObjectNotExists(&'static str),
    CreateElementError(&'static str),
    CanvasContextError,
    CartridgeAlreadyInserted,
    MalformedFileFormat,
}

impl Message {
    fn to_string(&self, locale: &str) -> String {
        to_string(self, locale)
    }
}

include!(concat!(env!("OUT_DIR"), "/messages.rs"));

pub struct I18n {
    locale: String
}

impl I18n {
    pub fn new(locale: String) -> I18n {
        I18n {
            locale
        }
    }

    pub fn to_string(&self, msg: Message) -> String {
        msg.to_string(self.locale.as_str())
    }
}