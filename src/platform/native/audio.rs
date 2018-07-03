use sdl2;
use sdl2::audio::{AudioCallback, AudioSpecDesired};

use super::Context;

struct AudioOutputCallback {
    channels: u8,
    sample_rate: f32,
    cb: Box<FnMut(u8, f32, &mut [f32]) + 'static + Send>,
}

impl AudioCallback for AudioOutputCallback {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for i in 0..self.channels {
            let mut channel_buffer = vec![0.0; out.len() / self.channels as usize];
            (self.cb)(i, self.sample_rate, &mut channel_buffer);
            for (channel_i, v) in channel_buffer.into_iter().enumerate() {
                out[i as usize + (channel_i * self.channels as usize)] = v;
            }
        }
    }
}

pub struct AudioDevice(sdl2::audio::AudioDevice<AudioOutputCallback>);

impl AudioDevice {
    pub fn new<T: FnMut(u8, f32, &mut [f32]) + 'static + Send>(
        context: &Context,
        channels: u8,
        cb: T,
    ) -> AudioDevice {
        let desired_spec = AudioSpecDesired {
            freq: Some(48000),
            channels: Some(channels), // mono
            samples: None,            // default sample size
        };

        let device = context
            .audio
            .open_playback(None, &desired_spec, |spec| AudioOutputCallback {
                sample_rate: spec.freq as f32,
                channels: spec.channels,
                cb: Box::new(cb),
            })
            .unwrap();
        device.resume();

        AudioDevice(device)
    }
}
