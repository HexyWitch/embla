use wasm_bindgen::prelude::*;

pub fn bootstrap() {
    use super::eval;
    eval(include_str!("../js/audio.js"));
}

#[wasm_bindgen]
pub struct AudioProcessCallback(Box<FnMut(u8, f32, &mut [f32]) + 'static>);

impl AudioProcessCallback {
    pub fn new<T: FnMut(u8, f32, &mut [f32]) + 'static>(f: T) -> AudioProcessCallback {
        AudioProcessCallback(Box::new(f))
    }
}

#[wasm_bindgen]
impl AudioProcessCallback {
    pub fn call(&mut self, channel: u8, sample_rate: f32, out: &mut [f32]) {
        (self.0)(channel, sample_rate, out)
    }
}

pub type AudioOutputHandle = JsValue;

#[wasm_bindgen]
extern "C" {
    pub fn create_audio_output(channels: u8, callback: AudioProcessCallback) -> AudioOutputHandle;
    pub fn destroy_audio_output(device: &AudioOutputHandle);
}
