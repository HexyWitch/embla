use super::PlatformContext;
use platform::audio as audio_impl;

pub struct AudioDevice(audio_impl::AudioDevice);

impl AudioDevice {
    pub fn new<T: FnMut(u8, f32, &mut [f32]) + 'static + Send>(
        context: &PlatformContext,
        channels: u8,
        cb: T,
    ) -> AudioDevice {
        AudioDevice(audio_impl::AudioDevice::new(&context.0, channels, cb))
    }
}
