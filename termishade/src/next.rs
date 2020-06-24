use na::Scalar;

pub trait Extend {
    type N: Clone;
    type Res;

    fn ext(&self, a: Self::N) -> Self::Res;
}

impl<N: Scalar + Clone> Extend for na::Vector2<N> {
    type N = N;
    type Res = na::Vector3<N>;

    fn ext(&self, a: N) -> Self::Res {
        na::Vector3::new(self.x.clone(), self.y.clone(), a)
    }
}

impl<N: Scalar + Clone> Extend for na::Vector3<N> {
    type N = N;
    type Res = na::Vector4<N>;

    fn ext(&self, a: N) -> Self::Res {
        na::Vector4::new(self.x.clone(), self.y.clone(), self.z.clone(), a)
    }
}
