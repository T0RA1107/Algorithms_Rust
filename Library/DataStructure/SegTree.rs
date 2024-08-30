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

pub mod seg_tree {
    // internal source is written as 1-indexed
    // and you can access as 0-indexed

    use crate::algebra::Monoid;

    #[derive(Clone)]
    pub struct SegTree<X> {
        size: usize,
        data: Vec<X>
    }

    impl<X> std::ops::Index<usize> for SegTree<X> {
        type Output = X;
        fn index(&self, i: usize) -> &Self::Output {
            &self.data[self.size / 2 + i]
        }
    }

    impl<X: Copy + Monoid> SegTree<X> {
        pub fn new(length: usize) -> Self {
            let mut M = 1;
            while M <= length {
                M <<= 1;
            }
            let size = M + M;
            let data = vec![X::one(); size];
            SegTree { size, data }
        }

        pub fn from(xs: Vec<X>) -> Self {
            let mut tree = Self::new(xs.len());
            for (i, &x) in xs.iter().enumerate() {
                tree.data[tree.size / 2 + i] = x;
            }
            for i in (1..tree.size / 2).rev() {
                tree.data[i] = tree.data[i << 1].ope(&tree.data[i << 1 | 1])
            }
            tree
        }

        pub fn update(&mut self, i: usize, x: X) {
            let mut idx = self.size / 2 + i;
            self.data[idx] = x;
            while idx > 1 {
                idx >>= 1;
                self.data[idx] = self.data[idx << 1].ope(&self.data[idx << 1 | 1]);
            }
        }

        pub fn fold(&self, l: usize, r: usize) -> X {
            let mut idx_l = self.size / 2 + l;
            let mut idx_r = self.size / 2 + r;
            let mut vl = X::one();
            let mut vr = X::one();
            while idx_l < idx_r {
                if idx_l & 1 == 1 {
                    vl = vl.ope(&self.data[idx_l]);
                    idx_l += 1;
                }
                if idx_r & 1 == 1 {
                    idx_r -= 1;
                    vr = self.data[idx_r].ope(&vr);
                }
                idx_l >>= 1; idx_r >>= 1;
            }
            vl.ope(&vr)
        }
    }
}
