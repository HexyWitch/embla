pub mod audio;
mod console_writer;
mod input;
pub mod rand;
pub mod rendering;
#[allow(unused)]
pub mod websocket;
pub mod window;

use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use failure::Error;

use js;
use js::MainLoopCallback;

use self::console_writer::ConsoleWriter;
use input::Input;
use window::WindowSettings;

use self::window::Window;

pub struct Context {
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
    js::bootstrap();

    io::set_print(Some(Box::new(ConsoleWriter::new())));
    io::set_panic(Some(Box::new(ConsoleWriter::new())));

    let mut input = Input::new();
    let windows = Rc::new(RefCell::new(Vec::new()));
    let context = Context {
        windows: Rc::clone(&windows),
    };
    let mut main_loop = app_factory(context);
    js::set_main_loop(MainLoopCallback(Box::new(move || {
        input.update(
            windows
                .borrow_mut()
                .iter_mut()
                .flat_map(|w| w.events())
                .collect(),
        );

        main_loop(0.016, &input).unwrap();
    })));
}
