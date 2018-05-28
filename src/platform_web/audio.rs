use js;

pub struct AudioDevice(js::audio::AudioOutputHandle);

impl AudioDevice {
    pub fn new<T: FnMut(u8, f32, &mut [f32]) + 'static + Send>(
        channels: u8,
        callback: T,
    ) -> AudioDevice {
        AudioDevice(js::audio::create_audio_output(
            channels,
            js::audio::AudioProcessCallback::new(callback),
        ))
    }
}

impl Drop for AudioDevice {
    fn drop(&mut self) {
        js::audio::destroy_audio_output(&self.0);
    }
}
