extern crate nalgebra_glm as glm;
use termion_output::TermionOutput;
use termishade::{
    rasterizer::TriangleRasterizer,
    renderer::{DrawParams, Renderer, TestRenderer},
    Program,
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

impl Program for CubeProgram {
    type VertexIn = Vertex;
    type VertexOut = glm::Vec4;
    type ColorOut = glm::Vec4;
    type Uniform = Uniform;
    type Intermediate = glm::Vec3;

    fn vertex(&self, v: &Vertex, Uniform { model, pv, .. }: &Uniform) -> (glm::Vec4, glm::Vec3) {
        let pos = *pv * *model * glm::vec4(v.pos.x, v.pos.y, v.pos.z, 1.0);
        let norm = *model * glm::vec4(v.norm.x, v.norm.y, v.norm.z, 0.0);
        (pos, v.norm.xyz())
    }

    fn fragment(
        &self,
        p: &glm::Vec4,
        normal: &glm::Vec3,
        Uniform { model, light, .. }: &Uniform,
    ) -> glm::Vec4 {
        let brightness = light.normalize().dot(&normal.normalize());
        let light = normal.abs();
        glm::vec4(light.x, light.y, light.z, 1.0).map(|a| a.max(0.0))
    }
}

fn main() {
    let input = std::fs::read_to_string(
        std::env::args()
            .nth(1)
            .unwrap_or_else(|| String::from("cube.obj")),
    )
    .unwrap();
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

    let output = TermionOutput::new().unwrap();
    let [w, h] = output.size();
    let mut renderer = TestRenderer::new(w, h);

    let view = glm::look_at(&glm::vec3(-5.0, 3.0, -4.0), &glm::zero(), &glm::Vec3::y());

    let projection = glm::perspective::<f32>(w as f32 / h as f32, 3.14 / 3.0, 0.1, 10.0);

    let start = std::time::Instant::now();
    loop {
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
                depth_test_enabled: true,
            },
            &cube,
            &uni,
        );

        output.render(renderer.color_buffer());
    }
}
