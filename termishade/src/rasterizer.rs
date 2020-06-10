use crate::Interpolate3;
use crate::util::*;

pub trait Rasterizer<V> {
    fn rasterize(&self, vertices: &[V; 3], size: [usize; 2]) -> Vec<[usize; 2]>;
}

pub struct TriangleRasterizer;

impl Rasterizer<na::Vector2<f32>> for TriangleRasterizer {
    fn rasterize(&self, vertices: &[na::Vector2<f32>; 3], size: [usize; 2]) -> Vec<[usize; 2]> {

        let [[sx, sy], [ex, ey]] = bounding_box(size, vertices);


        iproduct!(sx..=ex, sy..=ey)
            .map(|(x, y)| [x, y])
            .filter(|[x, y]| is_point_inside_triangle(vertices, na::Vector2::new(*x as f32, *y as f32)))
            .collect()
    }
}
