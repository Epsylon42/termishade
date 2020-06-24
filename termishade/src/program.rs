use alga::linear::InnerSpace;

pub trait Program: Sync {
    type VertexIn;
    type VertexOut: InnerSpace;
    type ColorOut;
    type Uniform;
    type Intermediate;

    fn vertex(
        &self,
        v: &Self::VertexIn,
        _: &Self::Uniform,
    ) -> (Self::VertexOut, Self::Intermediate);
    fn fragment(
        &self,
        pos: &Self::VertexOut,
        i: &Self::Intermediate,
        u: &Self::Uniform,
    ) -> Self::ColorOut;
}
