pub mod audio;
mod input;
pub mod rand;
pub mod rendering;
//pub mod websocket;
pub mod window;

use std::cell::RefCell;
use std::rc::Rc;

use failure::Error;
use sdl2;
use sdl2::event::Event;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use input::{Input, InputEvent};
use window::WindowSettings;

use self::input::{to_key, to_mouse_button};
use self::window::Window;

pub struct Context {
    video: sdl2::VideoSubsystem,
    audio: sdl2::AudioSubsystem,
    windows: Rc<RefCell<Vec<Rc<Window>>>>,
}

impl Context {
    pub fn window(&mut self, settings: WindowSettings) -> Result<Rc<Window>, Error> {
        let window = Rc::new(Window::new(self, settings)?);
        self.windows.borrow_mut().push(window.clone());
        Ok(window)
    }
}

pub fn init<F: FnOnce(Context) -> T, T: FnMut(f64, &Input) -> Result<(), Error> + 'static>(
    app_factory: F,
) {
    let sdl_context = sdl2::init().unwrap();
    let sdl_video = sdl_context.video().unwrap();

    let refresh_rate = sdl_video
        .current_display_mode(0)
        .expect("could not get display mode for current display")
        .refresh_rate;

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut input = Input::new();

    let windows = Rc::new(RefCell::new(Vec::new()));
    let context = Context {
        video: sdl_video,
        audio: sdl_context.audio().unwrap(),
        windows: windows.clone(),
    };

    let mut main_loop = app_factory(context);
    let mut start_frame = Instant::now();
    let mut frame_delay = 0.0;
    let target_frame_time = 1.0 / refresh_rate as f64;
    'main: loop {
        let frame_elapsed = start_frame.elapsed();
        start_frame = Instant::now();
        let frame_dt = frame_elapsed.as_secs() as f64
            + (frame_elapsed.subsec_nanos() as f64 / 1_000_000_000.0);
        frame_delay += target_frame_time - frame_dt;

        let mut input_events = Vec::new();
        for event in event_pump.poll_iter() {
            match event {
                Event::MouseMotion { x, y, .. } => {
                    input_events.push(InputEvent::MouseMove(x, y));
                }
                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => {
                    input_events.push(InputEvent::MouseDown {
                        button: to_mouse_button(mouse_btn),
                        position: (x, y),
                    });
                }
                Event::MouseButtonUp {
                    mouse_btn, x, y, ..
                } => {
                    input_events.push(InputEvent::MouseUp {
                        button: to_mouse_button(mouse_btn),
                        position: (x, y),
                    });
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => input_events.push(InputEvent::KeyDown(to_key(key))),
                Event::KeyUp {
                    keycode: Some(key), ..
                } => input_events.push(InputEvent::KeyUp(to_key(key))),
                _ => {}
            }
        }
        input.update(input_events);

        let start_update = Instant::now();
        main_loop(frame_dt, &input).unwrap();
        let update_elapsed = start_update.elapsed();
        let update_dt = update_elapsed.as_secs() as f64
            + (update_elapsed.subsec_nanos() as f64 / 1_000_000_000.0);

        let sleep = target_frame_time - update_dt + frame_delay - 0.002;
        if sleep > 0.0 {
            thread::sleep(Duration::new(
                sleep.floor() as u64,
                (sleep * 1_000_000_000.0).floor() as u32,
            ));
        } else {
            frame_delay = 0.0;
        }

        for w in windows.borrow().iter() {
            w.gl_finish();
        }
    }
}
