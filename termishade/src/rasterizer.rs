use crate::Interpolate3;

pub trait Rasterizer<V> {
    fn rasterize(&self, vertices: &[V; 3], size: [usize; 2]) -> Vec<[usize; 2]>;
}

pub struct TriangleRasterizer;

impl Rasterizer<na::Vector2<f32>> for TriangleRasterizer {
    fn rasterize(&self, vertices: &[na::Vector2<f32>; 3], size: [usize; 2]) -> Vec<[usize; 2]> {

        let [[sx, sy], [ex, ey]] = bounding_box(size, vertices);

        (sx..=ex)
            .flat_map(|x| (sy..=ey).map(move |y| [x, y]))
            .filter(|[x, y]| is_point_inside_triangle(vertices, na::Vector2::new(*x as f32, *y as f32)))
            .collect()
    }
}


pub fn bounding_box([width, height]: [usize; 2], ps: &[na::Vector2<f32>; 3]) -> [[usize; 2]; 2] {
    let [a, b, c] = ps;

    let sx = (a.x).min(b.x).min(c.x).floor().max(0.0) as usize;
    let sy = (a.y).min(b.y).min(c.y).floor().max(0.0) as usize;
    let ex = (a.x).max(b.x).max(c.x).ceil().min((width - 1) as f32) as usize;
    let ey = (a.y).max(b.y).max(c.y).ceil().min((height - 1) as f32) as usize;

    [[sx, sy], [ex, ey]]
}

pub fn is_point_inside_triangle(ps: &[na::Vector2<f32>; 3], p: na::Vector2<f32>) -> bool {
    na::Vector2::<f32>::to_barycentric(*ps, p)
        .iter()
        .sum::<f32>()
        <= 1.01
}
