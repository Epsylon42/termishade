use crate::Interpolate3;

pub fn map<T, U>(arr: [T; 3], mut f: impl FnMut(T) -> U) -> [U; 3] {
    let [a, b, c] = arr;
    [f(a), f(b), f(c)]
}

pub fn flatten_coord([width, _]: [usize; 2], [x, y]: [usize; 2]) -> usize {
    y * width + x
}

pub fn to_screenspace([width, height]: [usize; 2], p: na::Vector2<f32>) -> na::Vector2<f32> {
    let mut v = p / 2.0 + na::Vector2::new(0.5, 0.5);
    v.x *= width as f32;
    v.y *= height as f32;
    v
}

pub fn to_normspace([width, height]: [usize; 2], [x, y]: [usize; 2]) -> na::Vector2<f32> {
    let mut v = na::Vector2::new(x as f32, y as f32);
    v.x /= width as f32;
    v.y /= height as f32;
    v -= na::Vector2::new(0.5, 0.5);
    v * 2.0
}

pub fn bounding_box(
    [width, height]: [usize; 2],
    ps: &[na::Vector2<f32>; 3],
) -> Option<[[usize; 2]; 2]> {
    if ps
        .into_iter()
        .all(|p| p.x < 0.0 || p.x > (width - 1) as f32 || p.y < 0.0 || p.y > (height - 1) as f32)
    {
        return None;
    }

    let [a, b, c] = ps;

    let sx = (a.x).min(b.x).min(c.x).floor().max(0.0) as usize;
    let sy = (a.y).min(b.y).min(c.y).floor().max(0.0) as usize;
    let ex = (a.x).max(b.x).max(c.x).ceil().min((width - 1) as f32) as usize;
    let ey = (a.y).max(b.y).max(c.y).ceil().min((height - 1) as f32) as usize;

    Some([[sx, sy], [ex, ey]])
}

pub fn is_point_inside_triangle(ps: &[na::Vector2<f32>; 3], p: na::Vector2<f32>) -> bool {
    na::Vector2::<f32>::to_barycentric(*ps, p)
        .iter()
        .sum::<f32>()
        <= 1.01
}
