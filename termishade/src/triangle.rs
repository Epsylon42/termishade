extern crate nalgebra as na;

use termishade::{Program, Renderer};

struct TestProgram;

impl Program<f32> for TestProgram {
    type Vertex = na::Vector2<f32>;
    type Uniform = f32;
    type Intermediate = ();

    fn vertex(
        &self,
        v: &na::Vector2<f32>,
        _: &Self::Uniform,
    ) -> (na::Vector4<f32>, Self::Intermediate) {
        (na::Vector4::new(v.x, v.y, 0.0, 1.0), ())
    }

    fn fragment(&self, p: &na::Vector4<f32>, _: &(), u: &Self::Uniform) -> na::Vector4<f32> {
        let p = (p.xy() + na::Vector2::new(*u, *u)).map(f32::sin).abs();
        na::Vector4::new(p.x, p.y, 0.0, 1.0)
    }
}

fn main() {
    let (w, h) = termion::terminal_size().unwrap();
    let mut renderer = Renderer::new(w as usize, h as usize);

    let triangle = [
        na::Vector2::new(0.0, 1.0),
        na::Vector2::new(1.0, -1.0),
        na::Vector2::new(-1.0, -1.0),
    ];

    let start = std::time::Instant::now();
    loop {
        let now = std::time::Instant::now();
        renderer.clear(&na::Vector4::new(0.0, 0.0, 0.0, 1.0));
        renderer.draw(&TestProgram, &triangle, &(now - start).as_secs_f32());
        renderer.termion_render();
    }
}
