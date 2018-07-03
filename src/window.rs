use std::rc::Rc;

use failure::Error;

use math::Vec2;
use platform::window as window_impl;

use rendering::{RenderTarget, Renderer};

use super::PlatformContext;

#[derive(Default)]
pub struct WindowSettings {
    pub title: Option<String>,
    pub size: Option<Vec2<u32>>,
    pub canvas_id: Option<String>,
}

impl WindowSettings {
    pub fn new() -> WindowSettings {
        WindowSettings::default()
    }

    pub fn title(mut self, s: String) -> Self {
        self.title = Some(s);
        self
    }
    pub fn size(mut self, v: Vec2<u32>) -> Self {
        self.size = Some(v);
        self
    }
    pub fn canvas_id(mut self, id: String) -> Self {
        self.canvas_id = Some(id);
        self
    }
}

pub struct GLContext(window_impl::GLContext);

pub struct Window {
    inner: Rc<window_impl::Window>,
    gl_context: Option<GLContext>,
}

impl Window {
    pub fn new(context: &mut PlatformContext, settings: WindowSettings) -> Result<Window, Error> {
        let window = context.0.window(settings)?;
        let gl_context = Some(GLContext(window.gl_create_context()));
        Ok(Window {
            inner: window,
            gl_context,
        })
    }

    pub fn renderer<'a>(&'a self) -> Renderer<'a> {
        Renderer::new(self)
    }

    fn gl_make_current(&self) {
        self.inner
            .gl_set_current(&self.gl_context.as_ref().expect("no gl context set").0);
    }
}

impl RenderTarget for Window {
    fn make_current(&self) {
        self.gl_make_current();
    }
}
