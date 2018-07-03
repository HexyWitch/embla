#![feature(set_stdio)]
extern crate png;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
#[macro_use]
extern crate failure;
extern crate num_traits;

// Web backend
#[cfg(target_arch = "wasm32")]
extern crate js;

// Native backend
#[cfg(not(target_arch = "wasm32"))]
extern crate gl;
#[cfg(not(target_arch = "wasm32"))]
extern crate rand;
#[cfg(not(target_arch = "wasm32"))]
extern crate sdl2;
#[cfg(not(target_arch = "wasm32"))]
extern crate ws;

mod platform;

pub mod assets;
pub mod audio;
pub mod ecs;
pub mod graphics;
pub mod input;
pub mod math;
pub mod rendering;
pub mod util;
pub mod window;

use failure::Error;
use input::Input;

use audio::AudioDevice;
use window::{Window, WindowSettings};

pub struct PlatformContext(platform::Context);

impl PlatformContext {
    pub fn window(&mut self, settings: WindowSettings) -> Result<Window, Error> {
        Window::new(self, settings)
    }
    pub fn audio<T: FnMut(u8, f32, &mut [f32]) + 'static + Send>(
        &self,
        channels: u8,
        cb: T,
    ) -> AudioDevice {
        AudioDevice::new(self, channels, cb)
    }
}

pub fn rand() -> f32 {
    platform::rand::rand()
}

pub fn init<
    F: FnOnce(PlatformContext) -> T,
    T: FnMut(f64, &Input) -> Result<(), Error> + 'static,
>(
    app_factory: F,
) {
    platform::init(|ctx| app_factory(PlatformContext(ctx)))
}
