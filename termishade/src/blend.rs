pub trait Blender<T> {
    fn blend(&self, dst: &T, src: T) -> T;
}

pub struct Replace;

impl <T> Blender<T> for Replace {
    fn blend(&self, _: &T, src: T) -> T {
        src
    }
}

pub struct AlphaBlend;

impl Blender<na::Vector4<f32>> for AlphaBlend {
    fn blend(&self, dst: &na::Vector4<f32>, src: na::Vector4<f32>) -> na::Vector4<f32> {
        let color = dst.xyz() * (1.0 - src.w) + src.xyz() * src.w;
        na::Vector4::new(color.x, color.y, color.z, 1.0)
    }
}
