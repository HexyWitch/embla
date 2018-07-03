use std::cell::RefCell;
use std::rc::Rc;

use failure::Error;

use input::InputEvent;
use js;
use js::window::{CanvasWindow, InputHandler as JsInputHandler};
use window::WindowSettings;

use super::input::{to_key, to_mouse_button};
use super::Context;

pub struct GLContext(js::window::GLContext);

type InputEvents = Rc<RefCell<Vec<InputEvent>>>;

pub struct Window {
    js_window: CanvasWindow,
    input_events: InputEvents,
}

impl Window {
    pub fn new(_: &mut Context, settings: WindowSettings) -> Result<Window, Error> {
        let input_events = Rc::new(RefCell::new(Vec::new()));
        let handler = input_handler(&input_events);
        let WindowSettings { canvas_id, .. } = settings;

        let canvas_id =
            canvas_id.ok_or_else(|| format_err!("missing canvas id in WindowSettings"))?;
        Ok(Window {
            js_window: js::window::create_canvas_window(&canvas_id, handler),
            input_events,
        })
    }

    pub fn events(&self) -> impl Iterator<Item = InputEvent> {
        let mut events = self.input_events.borrow_mut();
        let events = events.drain(0..).collect::<Vec<InputEvent>>();
        events.into_iter()
    }

    pub fn gl_create_context(&self) -> GLContext {
        GLContext(js::window::get_window_context(&self.js_window))
    }

    pub fn gl_set_current(&self, gl_context: &GLContext) {
        js::window::gl_set_current_context(&gl_context.0);
    }
}

fn input_handler(input_events: &Rc<RefCell<Vec<InputEvent>>>) -> JsInputHandler {
    let mut handler = JsInputHandler::new();

    let events = Rc::clone(input_events);
    handler.set_mouse_move(move |x, y| {
        events.borrow_mut().push(InputEvent::MouseMove(x, y));
    });

    let events = Rc::clone(input_events);
    handler.set_mouse_down(move |button, x, y| {
        events.borrow_mut().push(InputEvent::MouseDown {
            button: to_mouse_button(button),
            position: (x, y),
        });
    });

    let events = Rc::clone(input_events);
    handler.set_mouse_up(move |button, x, y| {
        events.borrow_mut().push(InputEvent::MouseUp {
            button: to_mouse_button(button),
            position: (x, y),
        });
    });

    let events = Rc::clone(input_events);
    handler.set_key_down(move |key| {
        events.borrow_mut().push(InputEvent::KeyDown(to_key(key)));
    });

    let events = Rc::clone(input_events);
    handler.set_key_up(move |key| {
        events.borrow_mut().push(InputEvent::KeyUp(to_key(key)));
    });

    handler
}
