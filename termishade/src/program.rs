use alga::linear::InnerSpace;

pub trait Program: Sync {
    type VertexIn: Sync;
    type VertexOut: InnerSpace;
    type ColorOut;
    type Uniform: Sync;
    type Intermediate: Send + Sync;

    fn vertex(&self, v: &Self::VertexIn, _: &Self::Uniform) -> (Self::VertexOut, Self::Intermediate);
    fn fragment(
        &self,
        pos: &Self::VertexOut,
        i: &Self::Intermediate,
        u: &Self::Uniform,
    ) -> Self::ColorOut;
}
