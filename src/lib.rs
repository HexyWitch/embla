#![feature(set_stdio)]
extern crate png;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
#[macro_use]
extern crate failure;

// Web backend
#[cfg(target_arch = "wasm32")]
extern crate js;

// Native backend
#[cfg(not(target_arch = "wasm32"))]
extern crate gl;
#[cfg(not(target_arch = "wasm32"))]
extern crate sdl2;
#[cfg(not(target_arch = "wasm32"))]
extern crate ws;

// Setup common platform API
#[cfg(not(target_arch = "wasm32"))]
pub mod platform_native;
#[cfg(not(target_arch = "wasm32"))]
pub use platform_native as platform;

#[cfg(target_arch = "wasm32")]
pub mod platform_web;
#[cfg(target_arch = "wasm32")]
pub use platform_web as platform;

pub mod assets;
pub mod ecs;
pub mod graphics;
pub mod input;
pub mod math;
pub mod rendering_api;
pub mod util;

pub use platform::*;
