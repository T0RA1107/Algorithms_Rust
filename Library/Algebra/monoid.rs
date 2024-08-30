// Monoid M(X, *, id)
// requirements
// 1: a, b, c in M -> a * (b * c) = (a * b) * c
// 2: a in M -> a * id = id * a

pub trait Monoid {
    fn one() -> Self;
    fn ope(&self, rhs: &Self) -> Self;
}

pub trait Act<X> {
    fn act(&self, rhs: X) -> X;
}

macro_rules! monoid {
    (
        $type:ty where [ $( $params:tt )* ];
        one = $one:expr;
        ope($self:ident, $rhs:ident) = $code:block
        $(;)*
    ) => {
        impl<$($params)*> Monoid for $type {
            fn one() -> Self { $one }
            fn ope(&$self, $rhs: &Self) -> Self { $code }
        }
    };
    (
        $type:ty;
        one = $one:expr;
        ope($self:ident, $rhs:ident) = $code:block
        $(;)*
    ) => {
        monoid! {
            $type where [];
            one = $one;
            ope($self, $rhs) = $code
        }
    }
}

// --- begin MinMonoid ---
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MinMonoid(usize);

impl Monoid for MinMonoid {
    fn one() -> Self {
        MinMonoid(usize::MAX)
    }

    fn ope(&self, rhs: &Self) -> Self {
        MinMonoid(self.0.min(rhs.0))
    }
}

impl std::fmt::Display for MinMonoid {
    fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for MinMonoid {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s.parse::<usize>()?;
        Ok(MinMonoid(val))
    }
}
// --- end MinMonoid ---

// --- begin MaxMonoid ---
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MaxMonoid(usize);

impl Monoid for MaxMonoid {
    fn one() -> Self {
        MaxMonoid(usize::MIN)
    }

    fn ope(&self, rhs: &Self) -> Self {
        MaxMonoid(self.0.min(rhs.0))
    }
}

impl std::fmt::Display for MaxMonoid {
    fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for MaxMonoid {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s.parse::<usize>()?;
        Ok(MaxMonoid(val))
    }
}
// --- end MaxMonoid ---

// ---begin SumMonoid ---
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SumMonoid(usize);

impl Monoid for SumMonoid {
    fn one() -> Self {
        SumMonoid(0)
    }

    fn ope(&self, rhs: &Self) -> Self {
        SumMonoid(self.0 + rhs.0)
    }
}

impl std::fmt::Display for SumMonoid {
    fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for SumMonoid {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s.parse::<usize>()?;
        Ok(SumMonoid(val))
    }
}
// --- end SumMonoid ---
