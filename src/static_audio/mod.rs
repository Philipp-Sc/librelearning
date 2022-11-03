use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub enum StaticSounds {
    DialogWarning,
    BeginningOfLine,
    Prompt,
    MessageNewInstant,
    ServiceLogout,
    Start,
}

pub struct StaticAudio {
    dialog_warning: Vec<u8>,
    beginning_of_line: Vec<u8>,
    prompt: Vec<u8>,
    message_new_instant: Vec<u8>,
    service_logout: Vec<u8>,
    start: Vec<u8>,
}

impl Default for StaticAudio {
    fn default() -> Self {
        StaticAudio::new()
    }
}

impl StaticAudio {
    pub fn new() -> StaticAudio {
        StaticAudio {
            dialog_warning: include_bytes!("../../assets/dialog-warning.oga").to_vec(),
            beginning_of_line: include_bytes!("../../assets/beginning-of-line").to_vec(),
            prompt: include_bytes!("../../assets/prompt.wav").to_vec(),
            message_new_instant: include_bytes!("../../assets/message-new-instant.oga").to_vec(),
            service_logout: include_bytes!("../../assets/service-logout.oga").to_vec(),
            start: include_bytes!("../../assets/start").to_vec(),
        }
    }

    pub fn play_audio(&self, static_sound: &StaticSounds) {
        let sound = match static_sound {
            StaticSounds::DialogWarning => &self.dialog_warning,
            StaticSounds::BeginningOfLine => &self.beginning_of_line,
            StaticSounds::Prompt => &self.prompt,
            StaticSounds::MessageNewInstant => &self.message_new_instant,
            StaticSounds::ServiceLogout => &self.service_logout,
            StaticSounds::Start => &self.start,
        };
        let length = sound.len() as u32;
        let array = js_sys::Uint8Array::new_with_length(length);
        array.copy_from(&sound[..]);

        let context = web_sys::AudioContext::new().unwrap();

        let source: web_sys::AudioBufferSourceNode = context.create_buffer_source().unwrap();
        let destination: web_sys::AudioDestinationNode = context.destination();

        let f = Closure::once_into_js(move |buf: JsValue| {
            let array_buffer = web_sys::AudioBuffer::from(buf);
            source.set_buffer(Some(&array_buffer));
            source.connect_with_audio_node(&destination).ok();
            source.start().ok();
        });

        let _promise = context
            .decode_audio_data_with_success_callback(
                &array.buffer(),
                f.dyn_ref::<js_sys::Function>().unwrap(),
            )
            .unwrap();
        //let result = wasm_bindgen_futures::JsFuture::from(promise).await?; // async
    }
}
