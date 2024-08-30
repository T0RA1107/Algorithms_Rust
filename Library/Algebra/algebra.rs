mod algebra {
    pub trait Monoid {
        fn one() -> Self;
        fn ope(&self, rhs: &Self) -> Self;
    }

    pub trait Act<X> {
        fn act(&self, rhs: X) -> X;
    }

    pub trait Group: Monoid {
        fn inv(self) -> Self;
    }

    pub trait AGroup:
        std::ops::Add<Output = Self>
        + std::ops::Sub<Output = Self>
        + std::ops::Neg<Output = Self>
        + std::iter::Sum
    where
        Self: std::marker::Sized,
    {
        fn zero() -> Self;
    }
}
