use failure::Error;

use js::webgl;
use js::webgl::types::*;

use assets::Image;
use rendering::{TextureFiltering, Vertex, VertexAttributeType};

struct VertexShader {
    handle: webgl::Shader,
}

impl VertexShader {
    fn new(src: &str) -> Result<VertexShader, Error> {
        Ok(VertexShader {
            handle: compile_shader(src, webgl::VERTEX_SHADER)?,
        })
    }
    fn handle<'a>(&'a self) -> &'a webgl::Shader {
        &self.handle
    }
}

impl Drop for VertexShader {
    fn drop(&mut self) {
        webgl::gl_delete_shader(self.handle())
    }
}

struct FragmentShader {
    handle: webgl::Shader,
}

impl FragmentShader {
    fn new(src: &str) -> Result<FragmentShader, Error> {
        Ok(FragmentShader {
            handle: compile_shader(src, webgl::FRAGMENT_SHADER)?,
        })
    }
    fn handle<'a>(&'a self) -> &'a webgl::Shader {
        &self.handle
    }
}

impl Drop for FragmentShader {
    fn drop(&mut self) {
        webgl::gl_delete_shader(self.handle())
    }
}

pub struct VertexBuffer(webgl::Buffer);

impl VertexBuffer {
    fn new(buffer: webgl::Buffer) -> VertexBuffer {
        VertexBuffer(buffer)
    }
    fn handle<'a>(&'a self) -> &'a webgl::Shader {
        &self.0
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        webgl::gl_delete_buffer(&self.0);
    }
}

#[derive(Clone)]
pub enum Uniform {
    Vec2((f32, f32)),
    Texture(Texture),
}

pub struct Program {
    uniforms: Vec<(String, Uniform)>,
    handle: webgl::Program,
}

impl Program {
    fn new(vertex_shader: VertexShader, frag_shader: FragmentShader) -> Result<Program, Error> {
        Ok(Program {
            uniforms: Vec::new(),
            handle: link_program(&vertex_shader, &frag_shader)?,
        })
    }
    fn handle<'a>(&'a self) -> &webgl::Program {
        &self.handle
    }

    pub fn set_uniform(&mut self, name: &str, uniform: Uniform) {
        self.uniforms.push((name.into(), uniform));
    }
    pub fn uniforms(&self) -> impl Iterator<Item = &(String, Uniform)> {
        self.uniforms.iter()
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        webgl::gl_delete_program(self.handle())
    }
}

#[derive(Clone)]
pub struct Texture(webgl::Texture);

impl Texture {
    fn new(size: (u32, u32), filtering: Option<GLenum>) -> Texture {
        let handle = webgl::gl_create_texture();
        webgl::gl_bind_texture(webgl::TEXTURE_2D, &handle);
        webgl::gl_tex_parameter_i(
            webgl::TEXTURE_2D,
            webgl::TEXTURE_MIN_FILTER,
            filtering.unwrap_or(webgl::LINEAR) as GLint,
        );
        webgl::gl_tex_parameter_i(
            webgl::TEXTURE_2D,
            webgl::TEXTURE_MAG_FILTER,
            filtering.unwrap_or(webgl::LINEAR) as GLint,
        );

        webgl::gl_tex_image_2d_empty(
            webgl::TEXTURE_2D,
            0,
            webgl::RGBA,
            size.0 as GLsizei,
            size.1 as GLsizei,
            0 as GLint,
            webgl::RGBA,
            webgl::UNSIGNED_BYTE,
        );
        Texture(handle)
    }
    fn handle<'a>(&'a self) -> &'a webgl::Texture {
        &self.0
    }

    pub fn set_region(&self, image: &Image, offset: (u32, u32)) {
        webgl::gl_bind_texture(webgl::TEXTURE_2D, self.handle());
        webgl::gl_tex_sub_image_2d_u8(
            webgl::TEXTURE_2D,
            0,
            offset.0 as GLint,
            offset.1 as GLint,
            image.width as GLsizei,
            image.height as GLsizei,
            webgl::RGBA,
            webgl::UNSIGNED_BYTE,
            &image.data,
        );
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        webgl::gl_delete_texture(self.handle())
    }
}

pub fn screen_size() -> (i32, i32) {
    let width = webgl::gl_drawing_buffer_width();
    let height = webgl::gl_drawing_buffer_height();
    (width, height)
}
pub fn create_vertex_buffer() -> Result<VertexBuffer, Error> {
    let vbo = VertexBuffer::new(webgl::gl_create_buffer());

    Ok(vbo)
}
pub fn create_program(vs: &str, fs: &str) -> Result<Program, Error> {
    let vs = VertexShader::new(vs)?;
    let fs = FragmentShader::new(fs)?;

    Ok(Program::new(vs, fs)?)
}
pub fn create_texture(
    size: (u32, u32),
    filtering: Option<TextureFiltering>,
) -> Result<Texture, Error> {
    let filtering = filtering.map(|f| match f {
        TextureFiltering::Linear => webgl::LINEAR,
        TextureFiltering::Nearest => webgl::NEAREST,
    });
    Ok(Texture::new(size, filtering))
}

pub fn render_vertices<V: Vertex>(
    vertex_buffer: &VertexBuffer,
    program: &Program,
    vertices: &Vec<V>,
) -> Result<(), Error> {
    webgl::gl_blend_func(webgl::SRC_ALPHA, webgl::ONE_MINUS_SRC_ALPHA);
    webgl::gl_enable(webgl::BLEND);

    // push vertex data
    webgl::gl_bind_buffer(webgl::ARRAY_BUFFER, vertex_buffer.handle());
    unsafe {
        let data = ::std::slice::from_raw_parts(
            vertices.as_ptr() as *const u8,
            vertices.len() * V::stride(),
        );
        webgl::gl_buffer_data(webgl::ARRAY_BUFFER, data, webgl::STATIC_DRAW);
    }

    webgl::gl_use_program(program.handle());

    // set uniforms
    let mut texture_index = 0;
    for &(ref name, ref uniform) in program.uniforms() {
        let attr = webgl::gl_get_uniform_location(program.handle(), name);
        match uniform {
            &Uniform::Vec2(gl_vec2) => webgl::gl_uniform2f(&attr, gl_vec2.0, gl_vec2.1),
            &Uniform::Texture(ref gl_texture) => {
                webgl::gl_active_texture(webgl::TEXTURE0 + texture_index);
                webgl::gl_bind_texture(webgl::TEXTURE_2D, gl_texture.handle());
                webgl::gl_uniform1i(&attr, texture_index as GLint);
                texture_index += 1;
            }
        }
    }

    // define vertex format
    let mut step = 0;
    for (attr_name, attr_count, attr_type) in V::attributes() {
        let attr = webgl::gl_get_attrib_location(program.handle(), &attr_name);
        if attr < 0 {
            return Err(format_err!(
                "could not find location of attribute {}",
                attr_name
            ));
        }
        let attr = attr as u32;
        webgl::gl_enable_vertex_attrib_array(attr as u32);
        match attr_type {
            VertexAttributeType::Float => {
                webgl::gl_vertex_attrib_pointer(
                    attr,
                    attr_count as GLsizei,
                    webgl::FLOAT,
                    false,
                    V::stride() as GLsizei,
                    step,
                );
            }
            VertexAttributeType::Unsigned => {
                webgl::gl_vertex_attrib_pointer(
                    attr,
                    attr_count as GLsizei,
                    webgl::UNSIGNED_INT,
                    false,
                    V::stride() as GLsizei,
                    step,
                );
            }
        }

        step += (attr_count * attr_type.size()) as GLsizei;
    }

    webgl::gl_draw_arrays(webgl::TRIANGLES, 0, vertices.len() as GLsizei);

    Ok(())
}

pub fn clear(color: Option<(f32, f32, f32, f32)>) {
    let (r, g, b, a) = color.unwrap_or((0.0, 0.0, 0.0, 1.0));
    webgl::gl_clear_color(r, g, b, a);
    webgl::gl_clear(webgl::COLOR_BUFFER_BIT);
}

fn compile_shader(src: &str, t: GLenum) -> Result<webgl::Shader, Error> {
    let shader;
    shader = webgl::gl_create_shader(t);
    webgl::gl_shader_source(&shader, src);
    webgl::gl_compile_shader(&shader);

    let status = webgl::gl_get_shader_parameter(&shader, webgl::COMPILE_STATUS);
    if status != (webgl::TRUE as GLint) {
        let log = webgl::gl_get_shader_info_log(&shader);
        return Err(format_err!("Error compiling shader: {}", log));
    }
    Ok(shader)
}

fn link_program(vs: &VertexShader, fs: &FragmentShader) -> Result<webgl::Program, Error> {
    let program = webgl::gl_create_program();
    webgl::gl_attach_shader(&program, vs.handle());
    webgl::gl_attach_shader(&program, fs.handle());
    webgl::gl_link_program(&program);

    let status = webgl::gl_get_program_parameter(&program, webgl::LINK_STATUS);
    if status != (webgl::TRUE as GLint) {
        let log = webgl::gl_get_program_info_log(&program);
        return Err(format_err!("Error linking program: {}", log));
    }
    Ok(program)
}
