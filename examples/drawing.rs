extern crate embla;
extern crate failure;

use std::rc::Rc;

use failure::Error;

use embla::assets::image_from_png;
use embla::graphics::{TextureAtlas, TextureImage};
use embla::math::Vec2;
use embla::rand;
use embla::rendering::{Texture, TextureFiltering, Uniform, Vertex, VertexAttributeType};
use embla::window::WindowSettings;

const EMBLA_LOGO: &'static [u8] = include_bytes!("assets/embla.png");

const VERTEX_SHADER: &'static str = include_str!("assets/vertex_shader.glsl");
const FRAGMENT_SHADER: &'static str = include_str!("assets/fragment_shader.glsl");

fn main() {
    embla::init(|mut context| {
        let window = context
            .window(
                WindowSettings::new()
                    .title("Embla Drawing Example".to_string())
                    .size(Vec2::new(640, 480))
                    .canvas_id("window".to_string()),
            )
            .unwrap();

        let mut atlas = TextureAtlas::new((4096, 4096));
        let texture_size = (4096, 4096);
        let (program, atlas_texture, vertex_buffer) = {
            let renderer = window.renderer();

            let texture = renderer
                .create_texture(texture_size, Some(TextureFiltering::Nearest))
                .unwrap();
            let mut program = renderer
                .create_program::<TexturedVertex>(VERTEX_SHADER, FRAGMENT_SHADER)
                .unwrap();

            let screen_size = renderer.screen_size();
            program.set_uniform(
                "screen_size",
                Uniform::Vec2((screen_size.0 as f32, screen_size.1 as f32)),
            );
            program.set_uniform(
                "texture_size",
                Uniform::Vec2((texture_size.0 as f32, texture_size.1 as f32)),
            );
            program.set_uniform("texture", Uniform::Texture(texture.clone()));

            let vertex_buffer = renderer.create_vertex_buffer().unwrap();

            (program, texture, vertex_buffer)
        };

        let logo = TextureImage::new(Rc::new(image_from_png(EMBLA_LOGO).unwrap()));

        let mut logos: Vec<(Vec2<f32>, Vec2<f32>)> = (0..100)
            .map(|_| {
                (
                    Vec2::new(50.0 + rand() * 540.0, 10.0 + rand() * 440.0),
                    Vec2::new(84.45, 84.45) * (rand() - 0.5).signum(),
                )
            })
            .collect();

        move |dt, _input| {
            let mut vertices = Vec::new();

            for (p, v) in logos.iter_mut() {
                if p.x < 50.0 {
                    v.x = v.x.abs();
                } else if p.x > 590.0 {
                    v.x = -v.x.abs();
                }
                if p.y < 10.0 {
                    v.y = v.y.abs();
                } else if p.y > 460.0 {
                    v.y = -v.y.abs();
                }
                *p += *v * dt as f32;

                draw_texture(&mut vertices, &mut atlas, &atlas_texture, &logo, *p).unwrap();
            }

            let renderer = window.renderer();
            renderer.clear(Some((0.0, 0.0, 0.0, 1.0)));
            renderer
                .render_vertices(&vertex_buffer, &program, &vertices)
                .unwrap();

            Ok(())
        }
    });
}

#[repr(C)]
pub struct TexturedVertex {
    pub position: (f32, f32),
    pub tex_coord: (f32, f32),
}

impl Vertex for TexturedVertex {
    fn attributes() -> Vec<(String, usize, VertexAttributeType)> {
        vec![
            ("position".into(), 2, VertexAttributeType::Float),
            ("tex_coord".into(), 2, VertexAttributeType::Float),
        ]
    }
}

fn draw_texture(
    vertices: &mut Vec<TexturedVertex>,
    atlas: &mut TextureAtlas,
    atlas_texture: &Texture,
    texture: &TextureImage,
    position: Vec2<f32>,
) -> Result<(), Error> {
    let tex_region = match atlas.get_texture_block(texture) {
        Some(region) => region,
        None => {
            let region = atlas.add_texture(texture)?;
            atlas_texture.set_region(texture.image(), (region[0], region[1]));
            region
        }
    };
    let size = (tex_region[2] - tex_region[0], tex_region[3] - tex_region[1]);

    let rect = (
        size.0 as f32 / -2.0,
        size.1 as f32 / -2.0,
        size.0 as f32 / 2.0,
        size.1 as f32 / 2.0,
    );

    let ll = (position.x + rect.0, position.y + rect.1);
    let ul = (position.x + rect.0, position.y + rect.3);
    let ur = (position.x + rect.2, position.y + rect.3);
    let lr = (position.x + rect.2, position.y + rect.1);

    vertices.push(TexturedVertex {
        position: ll,
        tex_coord: (tex_region[0] as f32, tex_region[3] as f32),
    });
    vertices.push(TexturedVertex {
        position: ul,
        tex_coord: (tex_region[0] as f32, tex_region[1] as f32),
    });
    vertices.push(TexturedVertex {
        position: lr,
        tex_coord: (tex_region[2] as f32, tex_region[3] as f32),
    });
    vertices.push(TexturedVertex {
        position: ul,
        tex_coord: (tex_region[0] as f32, tex_region[1] as f32),
    });
    vertices.push(TexturedVertex {
        position: ur,
        tex_coord: (tex_region[2] as f32, tex_region[1] as f32),
    });
    vertices.push(TexturedVertex {
        position: lr,
        tex_coord: (tex_region[2] as f32, tex_region[3] as f32),
    });

    Ok(())
}
