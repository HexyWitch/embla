use std::marker::PhantomData;

use failure::Error;

use assets::Image;
use platform::rendering as render_impl;

#[derive(Debug, Clone, Copy)]
pub enum TextureFiltering {
    Nearest,
    Linear,
}

#[derive(Clone, Copy)]
pub enum VertexAttributeType {
    Float,
    Unsigned,
}

impl VertexAttributeType {
    pub fn size(self) -> usize {
        match self {
            VertexAttributeType::Float => 4,
            VertexAttributeType::Unsigned => 4,
        }
    }
}

pub trait Vertex {
    fn stride() -> usize {
        Self::attributes()
            .iter()
            .fold(0, |sum, a| sum + (a.1 * a.2.size()))
    }
    fn attributes() -> Vec<(String, usize, VertexAttributeType)>;
}

pub enum Uniform {
    Vec2((f32, f32)),
    Texture(Texture),
}

impl From<render_impl::Uniform> for Uniform {
    fn from(uniform: render_impl::Uniform) -> Self {
        match uniform {
            render_impl::Uniform::Vec2(v) => Uniform::Vec2(v),
            render_impl::Uniform::Texture(t) => Uniform::Texture(Texture(t)),
        }
    }
}

impl Into<render_impl::Uniform> for Uniform {
    fn into(self) -> render_impl::Uniform {
        match self {
            Uniform::Vec2(v) => render_impl::Uniform::Vec2(v),
            Uniform::Texture(t) => render_impl::Uniform::Texture(t.0),
        }
    }
}

pub struct VertexBuffer(render_impl::VertexBuffer);

pub struct Program<V: Vertex> {
    inner: render_impl::Program,
    vertex_format: PhantomData<V>,
}

impl<V: Vertex> Program<V> {
    pub fn set_uniform(&mut self, name: &str, uniform: Uniform) {
        self.inner.set_uniform(name, uniform.into())
    }
    pub fn uniforms<'a>(&'a self) -> impl Iterator<Item = (String, Uniform)> + 'a {
        self.inner
            .uniforms()
            .map(|(n, u)| (n.clone(), u.clone().into()))
    }
}

#[derive(Clone)]
pub struct Texture(render_impl::Texture);

impl Into<render_impl::Texture> for Texture {
    fn into(self) -> render_impl::Texture {
        self.0
    }
}

impl Texture {
    pub fn set_region(&self, image: &Image, offset: (u32, u32)) {
        self.0.set_region(image, offset)
    }
}

pub trait RenderTarget {
    fn make_current(&self);
}

pub struct Renderer<'a> {
    target: &'a RenderTarget,
}

impl<'a> Renderer<'a> {
    pub fn new(target: &'a RenderTarget) -> Renderer<'a> {
        Renderer { target }
    }

    pub fn screen_size(&self) -> (i32, i32) {
        self.target.make_current();

        render_impl::screen_size()
    }

    pub fn create_vertex_buffer(&self) -> Result<VertexBuffer, Error> {
        self.target.make_current();

        Ok(VertexBuffer(render_impl::create_vertex_buffer()?))
    }

    pub fn create_program<V: Vertex>(&self, vs: &str, fs: &str) -> Result<Program<V>, Error> {
        self.target.make_current();

        Ok(Program {
            inner: render_impl::create_program(vs, fs)?,
            vertex_format: PhantomData,
        })
    }

    pub fn create_texture(
        &self,
        size: (u32, u32),
        filtering: Option<TextureFiltering>,
    ) -> Result<Texture, Error> {
        self.target.make_current();

        Ok(Texture(render_impl::create_texture(size, filtering)?))
    }

    pub fn render_vertices<V: Vertex>(
        &self,
        vertex_buffer: &VertexBuffer,
        program: &Program<V>,
        vertices: &Vec<V>,
    ) -> Result<(), Error> {
        self.target.make_current();

        render_impl::render_vertices(&vertex_buffer.0, &program.inner, vertices)?;
        Ok(())
    }

    pub fn clear(&self, color: Option<(f32, f32, f32, f32)>) {
        self.target.make_current();

        render_impl::clear(color);
    }
}
