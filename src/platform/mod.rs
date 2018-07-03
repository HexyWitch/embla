#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
use self::native as platform_impl;

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
use self::web as platform_impl;

pub use self::platform_impl::{audio, init, rand, rendering, window, Context};
