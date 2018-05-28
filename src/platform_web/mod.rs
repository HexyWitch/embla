mod audio;
mod console_writer;
mod input;
mod renderer_webgl;
mod websocket;
mod window;

use std::io;

use failure::Error;
use input::Input;
use js;
use js::webgl;

use self::window::Window;

use self::audio::AudioDevice;
pub use self::renderer_webgl::WebGLRenderer as Renderer;
pub use self::websocket::Websocket;

pub struct PlatformContext();

impl PlatformContext {
    pub fn audio<T: FnMut(u8, f32, &mut [f32]) + 'static + Send>(
        &self,
        channels: u8,
        cb: T,
    ) -> AudioDevice {
        AudioDevice::new(channels, cb)
    }
}

pub fn run<
    F: FnOnce(&PlatformContext) -> T,
    T: FnMut(f32, &Input) -> Result<(), Error> + 'static,
>(
    app_factory: F,
) {
    js::bootstrap();

    use self::console_writer::ConsoleWriter;
    io::set_print(Some(Box::new(ConsoleWriter::new())));
    io::set_panic(Some(Box::new(ConsoleWriter::new())));

    let canvas_id = "window";
    let mut window = Window::new(canvas_id).unwrap();
    webgl::set_global_context(webgl::get_canvas_context(canvas_id));

    let mut event_dispatch = window.events();
    let mut input = Input::new();
    let mut main_loop = app_factory(&PlatformContext());
    window.set_main_loop(move || {
        let events = event_dispatch.input_events();
        input.update(&events);

        main_loop(0.016, &input).unwrap();
    });
}
