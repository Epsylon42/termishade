extern crate nalgebra_glm as glm;
use derive_interpolate::Interpolate;
use termion_target::{Key, TermionTarget};
use termishade::{
    blend, next::Extend, rasterizer::TriangleRasterizer, target::RenderTarget, BaseRenderer,
    ColorDepthRenderer, DrawParams, NalgebraParRenderer, Program,
};

struct CubeProgram;

struct Vertex {
    pos: glm::Vec3,
    norm: glm::Vec3,
}

struct Uniform {
    model: glm::Mat4,
    pv: glm::Mat4,
    light: glm::Vec3,
}

#[derive(Clone, Copy, Interpolate)]
struct Intermediate {
    normal: glm::Vec3,
}

impl Program for CubeProgram {
    type VertexIn = Vertex;
    type VertexOut = glm::Vec4;
    type ColorOut = glm::Vec4;
    type Uniform = Uniform;
    type Intermediate = Intermediate;

    fn vertex(
        &self,
        v: &Vertex,
        uniform: &Uniform,
    ) -> (glm::Vec4, Self::Intermediate) {
        let Uniform { model, pv, .. } = *uniform;

        let pos = pv * model * v.pos.ext(1.0);
        (pos, Intermediate { normal: v.norm })
    }

    fn fragment(
        &self,
        _: &glm::Vec4,
        int: &Self::Intermediate,
        uniform: &Uniform,
    ) -> glm::Vec4 {
        let Intermediate { normal } = *int;
        let Uniform { model, light, .. } = *uniform;

        let normal = (model * normal.ext(0.0)).xyz().normalize();
        let brightness = light.normalize().dot(&normal);
        let light = normal.abs() * brightness;
        (light.map(|a| a.max(0.0)) + glm::Vec3::repeat(0.05)).ext(1.0)
    }
}

fn main() {
    let input = std::fs::read_to_string(
        std::env::args()
            .nth(1)
            .unwrap_or_else(|| String::from("cube.obj")),
    )
    .unwrap();

    let multisampling_level = std::env::args()
        .nth(2)
        .unwrap_or_else(|| String::from("1"))
        .parse()
        .expect("invalid multisampling level");

    let cube: obj::Obj = obj::load_obj(std::io::Cursor::new(input)).unwrap();

    let cube = cube
        .indices
        .iter()
        .map(|&i| {
            let v = &cube.vertices[i as usize];
            Vertex {
                pos: v.position.into(),
                norm: v.normal.into(),
            }
        })
        .collect::<Vec<_>>();

    let mut target = TermionTarget::new().unwrap();
    let [w, h] = target.size_multisampled(multisampling_level);
    let mut renderer = ColorDepthRenderer::new(w, h);

    let mut view = glm::look_at(&glm::vec3(-5.0, 3.0, -4.0), &glm::zero(), &glm::Vec3::y());

    let projection = glm::perspective::<f32>(w as f32 / h as f32, 3.14 / 3.0, 0.1, 10.0);

    let start = std::time::Instant::now();
    'main: loop {
        let now = std::time::Instant::now();
        let model: glm::Mat4 = glm::rotation((now - start).as_secs_f32(), &glm::Vec3::y());

        let uni = Uniform {
            model,
            pv: projection * view,
            light: glm::vec3(-1.0, 1.0, 1.0),
        };

        renderer.clear_color(&glm::vec4(0.0, 0.0, 0.0, 1.0));
        renderer.clear_depth(1.0);
        renderer.draw(
            DrawParams {
                program: &CubeProgram,
                rasterizer: &TriangleRasterizer,
                blender: &blend::Replace,
                depth_test_enabled: true,
            },
            &cube,
            &uni,
        );

        target.draw_multisampled(renderer.color_buffer(), multisampling_level);

        while let Some(key) = target.get_key() {
            match key {
                Key::Esc | Key::Ctrl('c') => break 'main,
                Key::Left => {
                    view = glm::rotation(-0.01, &glm::Vec3::y()) * view;
                }
                Key::Right => {
                    view = glm::rotation(0.01, &glm::Vec3::y()) * view;
                }
                _ => {}
            }
        }
    }
}
