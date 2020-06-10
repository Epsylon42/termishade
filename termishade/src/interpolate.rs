use alga::general::ComplexField;
use alga::linear::InnerSpace;

fn triangle_area<V: InnerSpace + Copy>(ps: [V; 3]) -> V::RealField
where
    V::RealField: From<f32>,
{
    let [a, b, c] = ps;
    let ab = b - a;
    let ac = b - c;
    let angle = ab.angle(&ac);
    return ab.norm() * ac.norm() * angle.sin() / V::RealField::from(2.0);
}

pub trait Interpolate3<V: InnerSpace>: Sized {
    fn to_barycentric(ps: [V; 3], p: V) -> [V::RealField; 3];
    fn interpolate(ps: [V; 3], p: V, v: [Self; 3]) -> Self;
}

impl<T, V> Interpolate3<V> for T
where
    V: InnerSpace + Copy,
    T: std::ops::Add<T, Output = T> + std::ops::Mul<V::RealField, Output = T>,
    V::RealField: From<f32>,
{
    fn to_barycentric(ps: [V; 3], p: V) -> [V::RealField; 3] {
        let [a, b, c] = ps;
        let full_area = triangle_area([a, b, c]);
        let a_area = triangle_area([b, c, p]) / full_area;
        let b_area = triangle_area([a, c, p]) / full_area;
        let c_area = triangle_area([a, b, p]) / full_area;
        [a_area, b_area, c_area]
    }

    fn interpolate(ps: [V; 3], p: V, v: [Self; 3]) -> Self {
        let [a, b, c] = v;
        let [a_area, b_area, c_area] = Self::to_barycentric(ps, p);
        a * a_area + b * b_area + c * c_area
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Flat<T>(pub T);

impl <T> From<T> for Flat<T> {
    fn from(t: T) -> Self {
        Flat(t)
    }
}

impl <T, V> Interpolate3<V> for Flat<T>
where
    V: InnerSpace + Copy,
    V::RealField: From<f32>
{
    fn to_barycentric(_: [V; 3], _: V) -> [V::RealField; 3] {
        [1.0.into(), 0.0.into(), 0.0.into()]
    }

    fn interpolate(_: [V; 3], _: V, [v, _, _]: [Self; 3]) -> Self {
        v
    }
}
