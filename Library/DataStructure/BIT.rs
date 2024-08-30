pub mod algebra {
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

pub mod bit {
    use crate::algebra::*;

    // ----- begin Algebra implement for i64 -----
    impl Monoid for i64 {
        fn one() -> Self {
            1
        }
        fn ope(&self, rhs: &Self) -> Self {
            self + rhs
        }
    }
    impl Group for i64 {
        fn inv(self) -> Self {
            -self
        }
    }
    impl AGroup for i64 {
        fn zero() -> Self {
            0
        }
    }
    // ----- end Algebra implement for i64 -----

    // ----- begin Binary Indexed Tree -----
    pub struct BIT<T> {
        n: usize,
        data: Vec<T>,
    }

    impl<T: Copy + AGroup> BIT<T> {
        pub fn new(n: usize) -> Self {
            BIT { n, data: vec![T::zero(); n + 1] }
        }

        // sum of [0, i)
        pub fn accum(&self, i: usize) -> T {
            let mut i = i;
            let mut ret = T::zero();
            while i > 0 {
                ret = ret + self.data[i];
                i -= i & i.wrapping_neg();
            }
            ret
        }

        // sum of [l, r)
        pub fn sum(&self, l: usize, r: usize) -> T {
            if r <= l { return T::zero(); }
            self.accum(r) - self.accum(l)
        }

        pub fn get(&self, i: usize) -> T {
            self.sum(i, i + 1)
        }


        pub fn update(&mut self, i: usize, x: T) {
            let mut i = i + 1;
            while i <= self.n {
                self.data[i] = self.data[i] + x;
                i += i & i.wrapping_neg();
            }
        }

        pub fn lower_bound(&mut self, mut w: T) -> usize
        where
            T: std::cmp::PartialOrd
        {
            let mut x: usize = 0;
            let mut k: usize = (self.n + 1).next_power_of_two() / 2;
            while k > 0 {
                if x + k < self.n && self.data[x + k] < w {
                    w = w - self.data[x + k];
                    x += k;
                }
                k /= 2;
            }
            x
        }
    }
    // ----- end Binary Indexed Tree -----

    // ----- begin Binary Indexed Tree 2D -----
    pub struct BIT2D<T> {
        h: usize,
        w: usize,
        data: Vec<Vec<T>>,
    }

    impl<T: Copy + AGroup> BIT2D<T> {
        pub fn new(h: usize, w: usize) -> Self {
            BIT2D { h, w, data: vec![vec![T::zero()]; h + 1] }
        }

        // sum of [0, i) x [0, j)
        pub fn accum(&mut self, i: usize, j: usize) -> T {
            let mut i = i;
            let mut ret = T::zero();
            while i > 0 {
                let mut j = j;
                while j > 0 {
                    ret = ret + self.data[i][j];
                    j -= j & j.wrapping_neg();
                }
                i -= i & i.wrapping_neg();
            }
            ret
        }

        // sum of [li, ri) x [lj, rj)
        pub fn sum(&mut self, li: usize, ri: usize, lj: usize, rj: usize) -> T {
            if ri <= li || rj <= lj { return T::zero(); }
            self.accum(ri, rj) - self.accum(ri, lj) - self.accum(li, rj) + self.accum(li, lj)
        }

        pub fn update(&mut self, i: usize, j: usize, x: T) {
            let mut i = i + 1;
            while i <= self.h {
                let mut j = j + 1;
                while j <= self.w {
                    self.data[i][j] = self.data[i][j] + x;
                    j += j & j.wrapping_neg();
                }
                i += i & i.wrapping_neg();
            }
        }
    }
    // ----- end Binary Indexed Tree 2D -----
}
