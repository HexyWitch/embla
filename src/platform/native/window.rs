use failure::Error;
use gl;
use sdl2;
use sdl2::video::{GLProfile, SwapInterval};

use super::Context;
use window::WindowSettings;

pub struct Window(sdl2::video::Window);

pub struct GLContext(sdl2::video::GLContext);

impl Window {
    pub fn new(context: &mut Context, settings: WindowSettings) -> Result<Window, Error> {
        let WindowSettings { title, size, .. } = settings;

        let title = title.ok_or_else(|| format_err!("missing title in WindowSettings"))?;
        let size = size.ok_or_else(|| format_err!("missing size in WindowSettings"))?;
        let window = context
            .video
            .window(&title, size.x, size.y)
            .opengl()
            .build()?;

        let gl_attr = context.video.gl_attr();
        gl_attr.set_context_major_version(2);
        gl_attr.set_context_minor_version(0);
        gl_attr.set_context_profile(GLProfile::GLES);
        gl_attr.set_double_buffer(false);
        gl_attr.set_depth_size(1);

        let _gl_context = window.gl_create_context();
        gl::load_with(|name| context.video.gl_get_proc_address(name) as *const _);
        context
            .video
            .gl_set_swap_interval(SwapInterval::VSync)
            .map_err(|e| format_err!("{}", e))?;

        Ok(Window(window))
    }

    pub fn gl_create_context(&self) -> GLContext {
        GLContext(
            self.0
                .gl_create_context()
                .expect("could not create gl context"),
        )
    }

    pub fn gl_set_current(&self, gl_context: &GLContext) {
        self.0
            .gl_make_current(&gl_context.0)
            .expect("could not set window as current gl context")
    }

    pub fn gl_finish(&self) {
        self.0.gl_swap_window();
        unsafe { gl::Finish() };
    }
}
