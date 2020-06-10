use crate::{Program, Rasterizer, Interpolate3, Blender};
use crate::util::*;

pub struct DrawParams<'a, P, R, B> {
    pub program: &'a P,
    pub rasterizer: &'a R,
    pub blender: &'a B,
    pub depth_test_enabled: bool,
}

pub trait Renderer
{
    type Color;
    type Vertex;

    fn size(&self) -> [usize; 2];

    fn color_buffer(&mut self) -> &mut [Self::Color];
    fn depth_buffer(&mut self) -> &mut [f32];

    fn clear_color(&mut self, color: &Self::Color)
        where Self::Color: Clone
    {
        for c in self.color_buffer() {
            *c = color.clone();
        }
    }

    fn clear_depth(&mut self, depth: f32)
    {
        for d in self.depth_buffer() {
            *d = depth;
        }
    }

    fn draw<P, R, B>(&mut self, params: DrawParams<P, R, B>, vertices: &[P::VertexIn], uniform: &P::Uniform)
        where P: Program<VertexOut=na::Vector4<f32>, ColorOut=Self::Color>,
              R: Rasterizer<na::Vector2<f32>>,
              B: Blender<Self::Color>,
              P::Intermediate: Interpolate3<na::Vector3<f32>> + Copy
    {
        let transformed = vertices
            .into_iter()
            .map(|v| params.program.vertex(v, uniform))
            .collect::<Vec<_>>();

        let size = self.size();
        for chunk in transformed.chunks(3) {
            let vertices = [chunk[0].0, chunk[1].0, chunk[2].0];
            let vertices = map(vertices, |v| v.xyz() / v.w);

            let intermediate = [chunk[0].1, chunk[1].1, chunk[2].1];

            let screenspace_vertices = map(vertices, |v| to_screenspace(size, v.xy()));
            let points = params.rasterizer.rasterize(&screenspace_vertices, size);

            for point in points {
                let idx = flatten_coord(size, point);
                let point = to_normspace(size, point);

                let z = <_>::interpolate(
                    map(vertices, |v| v.xy()),
                    point,
                    map(vertices, |v| v.z)
                );

                if params.depth_test_enabled {
                    let depth = self.depth_buffer();
                    if depth[idx] < z {
                        continue;
                    }
                    depth[idx] = z;
                }

                let point = na::Vector4::new(point.x, point.y, z, 1.0);
                let intermediate = <_>::interpolate(
                    vertices,
                    point.xyz(),
                    intermediate
                );
                let src = params.program.fragment(&point, &intermediate, uniform);
                let color = self.color_buffer();
                color[idx] = params.blender.blend(&color[idx], src);
            }
        }
    }
}

pub struct TestRenderer {
    width: usize,
    height: usize,
    color: Vec<na::Vector4<f32>>,
    depth: Vec<f32>,
}

impl TestRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            color: vec![na::Vector4::zeros(); width * height],
            depth: vec![0.0; width * height],
        }
    }
}

impl Renderer for TestRenderer {
    type Color = na::Vector4<f32>;
    type Vertex = na::Vector4<f32>;

    fn size(&self) -> [usize; 2] {
        [self.width, self.height]
    }

    fn color_buffer(&mut self) -> &mut [Self::Color] {
        &mut self.color
    }

    fn depth_buffer(&mut self) -> &mut [f32] {
        &mut self.depth
    }
}
