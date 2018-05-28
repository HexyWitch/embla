#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
#![allow(non_camel_case_types)]

extern crate wasm_bindgen;

pub mod audio;
pub mod webgl;
pub mod websocket;
pub mod window;

use wasm_bindgen::prelude::*;

pub fn bootstrap() {
    audio::bootstrap();
    webgl::bootstrap();
    websocket::bootstrap();
    window::bootstrap();
}

#[wasm_bindgen]
extern "C" {
    pub type console;

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    pub fn eval(s: &str);
}
