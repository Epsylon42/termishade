extern crate nalgebra_glm as glm;
use derive_interpolate::Interpolate;
use termion_target::TermionTarget;
use termishade::{
    blend, next::Extend, rasterizer::TriangleRasterizer, BaseRenderer,
    ColorDepthRenderer, DrawParams, Program, NalgebraRenderer
};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
type ErrString = JsValue;
#[cfg(not(feature = "wasm"))]
type ErrString = String;

struct CubeProgram;

pub struct Vertex {
    pub pos: glm::Vec3,
    pub norm: glm::Vec3,
}

pub struct Uniform {
    pub model: glm::Mat4,
    pub pv: glm::Mat4,
    pub light: glm::Vec3,
}

#[derive(Clone, Copy, Interpolate)]
struct Intermediate {
    normal: glm::Vec3,
    color: glm::Vec3,
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
        let normal = (model * v.norm.ext(0.0)).xyz().normalize();
        let color = v.norm.abs();
        (pos, Intermediate { normal, color })
    }

    fn fragment(&self, _: &glm::Vec4, int: &Self::Intermediate, uniform: &Uniform) -> glm::Vec4 {
        let Intermediate { normal, color, .. } = *int;
        let Uniform { light, .. } = *uniform;

        let brightness = light.normalize().dot(&normal);
        let light = color.abs() * brightness;
        (light.map(|a| a.max(0.0)) + glm::Vec3::repeat(0.05)).ext(1.0)
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Webrender {
    pub t: f32,
    pub model: Vec<Vertex>,
    pub uniform: Uniform,
    renderer: ColorDepthRenderer,
    target: TermionTarget,
    pub original_num_vertices: usize,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Webrender {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(width: usize, height: usize, rgb: bool, obj: &[u8]) -> Result<Webrender, ErrString> {
        let model: obj::Obj =
            obj::load_obj(std::io::Cursor::new(obj)).map_err(|e| ErrString::from(format!("Invalid object: {}", e)))?;
        let original_num_vertices = model.vertices.len();

        let mut model = model
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

        let target = TermionTarget::new_without_io(width, height)
            .reduced_palette(!rgb);
        let renderer = ColorDepthRenderer::new(width, height);

        Self::center_model(&mut model);
        let model_size = Self::model_size(&model);
        let camera_pos = glm::vec3(-5.0, 3.0, -4.0).normalize() * model_size * 2.0;

        let projection =
            glm::perspective::<f32>(width as f32 / height as f32, 3.14 / 3.0, 0.1, 10.0);
        let view = glm::look_at(&camera_pos, &glm::zero(), &glm::Vec3::y());

        let uniform = Uniform {
            model: glm::identity(),
            pv: projection * view,
            light: glm::vec3(-1.0, 1.0, 1.0)
        };

        Ok(Webrender {
            t: 0.0,
            model,
            uniform,
            renderer,
            target,
            original_num_vertices,
        })
    }

    fn center_model(model: &mut [Vertex]) {
        let center = model.iter()
            .map(|v| v.pos)
            .sum::<glm::Vec3>() / model.len() as f32;

        for v in model {
            v.pos -= center;
        }
    }

    fn model_size(model: &[Vertex]) -> f32 {
        model.iter()
            .map(|v| v.pos.magnitude())
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or_default()
    }

    pub fn step(&mut self, dt: f32) {
        self.t += dt;

        let rotation = glm::rotation(self.t, &glm::Vec3::y());
        self.uniform.model = rotation;
        self.uniform.light = (rotation.transpose() * glm::vec4(-1.0, 1.0, 1.0, 0.0)).xyz();
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
