extern crate nalgebra_glm as glm;
use derive_interpolate::Interpolate;
use termion_target::TermionTarget;
use termishade::{
    blend, next::Extend, rasterizer::TriangleRasterizer, BaseRenderer,
    ColorDepthRenderer, DrawParams, Program, NalgebraRenderer
};

use wasm_bindgen::prelude::*;

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

    fn vertex(&self, v: &Vertex, uniform: &Uniform) -> (glm::Vec4, Self::Intermediate) {
        let Uniform { model, pv, .. } = *uniform;

        let pos = pv * model * v.pos.ext(1.0);
        (pos, Intermediate { normal: v.norm })
    }

    fn fragment(&self, _: &glm::Vec4, int: &Self::Intermediate, uniform: &Uniform) -> glm::Vec4 {
        let Intermediate { normal } = *int;
        let Uniform { model, light, .. } = *uniform;

        let normal = (model * normal.ext(0.0)).xyz().normalize();
        let brightness = light.normalize().dot(&normal);
        let light = normal.abs() * brightness;
        (light.map(|a| a.max(0.0)) + glm::Vec3::repeat(0.05)).ext(1.0)
    }
}

#[wasm_bindgen]
pub struct Webrender {
    t: f32,
    model: Vec<Vertex>,
    uniform: Uniform,
    renderer: ColorDepthRenderer,
    target: TermionTarget,
}

#[wasm_bindgen]
impl Webrender {
    pub fn new(width: usize, height: usize, obj: &str) -> Result<Webrender, JsValue> {
        let model: obj::Obj =
            obj::load_obj(std::io::Cursor::new(obj)).map_err(|_| JsValue::from_str("Invalid object"))?;

        let model = model
            .indices
            .iter()
            .map(|&i| {
                let v = &model.vertices[i as usize];
                Vertex {
                    pos: v.position.into(),
                    norm: v.normal.into(),
                }
            })
            .collect::<Vec<_>>();

        let target = TermionTarget::new_without_io(width, height);
        let renderer = ColorDepthRenderer::new(width, height);

        let projection =
            glm::perspective::<f32>(width as f32 / height as f32, 3.14 / 3.0, 0.1, 10.0);
        let view = glm::look_at(&glm::vec3(-5.0, 3.0, -4.0), &glm::zero(), &glm::Vec3::y());

        let uniform = Uniform {
            model: glm::identity(),
            pv: projection * view,
            light: glm::vec3(-1.0, 1.0, 1.0),
        };

        Ok(Webrender {
            t: 0.0,
            model,
            uniform,
            renderer,
            target,
        })
    }

    pub fn step(&mut self, dt: f32) {
        self.t += dt;
        self.uniform.model = glm::rotation(self.t, &glm::Vec3::y())
    }

    pub fn render(&mut self) -> String {
        self.renderer.clear_color(&glm::vec4(0.0, 0.0, 0.0, 1.0));
        self.renderer.clear_depth(1.0);
        self.renderer.draw(
            DrawParams {
                program: &CubeProgram,
                rasterizer: &TriangleRasterizer,
                blender: &blend::Replace,
                depth_test_enabled: true,
            },
            &self.model,
            &self.uniform,
        );

        self.target.draw_to_string(self.renderer.color_buffer())
    }
}
