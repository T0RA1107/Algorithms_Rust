// Group G(X, *, id)
// requirements
// 1: a, b, c in G -> a * (b * c) = (a * b) * c
// 2: a in G -> a * id = id * a
// 3: for some b in G, for all a in G -> a * b = b * a = id

pub trait Monoid {
    fn one() -> Self;
    fn ope(&self, rhs: &Self) -> Self;
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

macro_rules! agroup {
    (
        $type:ty where [ $( $params:tt )* ];
        zero = $zero:expr;
        add($self:ident, $rhs:ident) = $code:block;
        neg($self_neg:ident) = $code_neg:block
        $(;)*
    ) => {
        impl<$($params)*> std::ops::Add for $type {
            type Output = Self;
            fn add($self, $rhs: Self) -> Self { $code }
        }
        impl<$($params)*> std::ops::Neg for $type {
            type Output = Self;
            fn neg($self_neg) -> Self { $code_neg }
        }
        impl<$($params)*> std::ops::Sub for $type {
            type Output = Self;
            fn sub($self, rhs: Self) -> Self { ($self) + (-rhs) }
        }
        impl<$($params)*> std::ops::AddAssign for $type where Self: Clone {
            fn add_assign(&mut $self, $rhs: Self) {
                *$self = (*$self).clone() + $rhs;
            }
        }
        impl<$($params)*> std::ops::SubAssign for $type where Self: Clone {
            fn sub__assign(&mut $self, $rhs: Self) {
                *$self = (*$self).clone() - $rhs;
            }
        }
        impl<$($params)*> std::iter::Sum for $type {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::zero(), std::ops::Add::add)
            }
        }
        impl std::fmt::Display for $type {
            fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl<$($params)*> AGroup for $type {
            fn zero() -> Self { $zero }
        }
        impl std::str::FromStr for $type {
            type Err = std::num::ParseIntError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let val = s.parse()?;
                Ok(val)
            }
        }
    };
    (
        $type:ty ;
        zero = $zero:expr ;
        add($self:ident, $y:ident) = $code:block ;
        neg($self_neg:ident) = $code_neg:block
        $(;)*
    ) => {
        agroup! { $type where []; zero = $zero; add($self, $y) = $code; neg($self_neg) = $code_neg; }
    };
}
