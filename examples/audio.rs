extern crate embla;

use std::f32;

fn main() {
    embla::run(|context| {
        let mut phase_l = 0.0;
        let mut phase_r = 0.0;
        let output = context.audio(2, move |channel, sample_rate, out| {
            if channel == 0 {
                let phase_inc = 220.0 / sample_rate;
                for v in out.iter_mut() {
                    *v = (phase_l * f32::consts::PI * 2.0).sin() * 0.15;
                    phase_l = (phase_l + phase_inc) % 1.0;
                }
            } else {
                let phase_inc = 440.0 / sample_rate;
                for v in out.iter_mut() {
                    *v = (phase_r * f32::consts::PI * 2.0).sin() * 0.15;
                    phase_r = (phase_r + phase_inc) % 1.0;
                }
            }
        });
        move |_dt, _input| {
            // hold on to the audio output for the duration of the application
            let _audio_output = &output;

            Ok(())
        }
    });
}
