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

pub mod lazy_seg_tree {
    use crate::algebra::*;

    #[derive(Clone)]
    pub struct LazySegTree<X, M> {
        _n: usize,
        m: usize,
        data: Vec<X>,
        lazy: Vec<M>
    }

    impl<X: Copy + Monoid, M: Copy + Monoid + Act<X>> LazySegTree<X, M> {
        pub fn new(_n: usize) -> Self {
            let m = 1 << (32 - (_n as u32 - 1).leading_zeros()) as usize;
            LazySegTree {
                _n,
                m,
                data: vec![X::one(); m + m],
                lazy: vec![M::one(); m + m]
            }
        }

        pub fn build(&mut self, seq: Vec<X>) {
            for (i, &x) in seq.iter().enumerate() {
                self.data[i + self.m] = x;
            }
            for i in (1..self.m).rev() {
                self.data[i] = self.data[i << 1].ope(&self.data[i << 1 | 1]);
            }
        }

        fn _eval_at(&self, i: usize) -> X {
            self.lazy[i].act(self.data[i])
        }

        fn _propagate_at(&mut self, i: usize) {
            self.data[i] = self._eval_at(i);
            self.lazy[i << 1] = self.lazy[i << 1].ope(&self.lazy[i]);
            self.lazy[i << 1 | 1] = self.lazy[i << 1 | 1].ope(&self.lazy[i]);
            self.lazy[i] = M::one();
        }

        fn _propagate_above(&mut self, i: usize) {
            let h = (32 - (i as u32).leading_zeros()) as usize;
            for n in (1..h).rev() {
                self._propagate_at(i >> n);
            }
        }

        fn _recalc_above(&mut self, mut i: usize) {
            while i > 1 {
                i >>= 1;
                self.data[i] = self._eval_at(i << 1).ope(&self._eval_at(i << 1 | 1));
            }
        }

        pub fn set_val(&mut self, mut i: usize, x: X) {
            i += self.m;
            self._propagate_above(i);
            self.data[i] = x;
            self.lazy[i] = M::one();
            self._recalc_above(i);
        }

        fn _lsb(n: usize) -> usize {
            let _n = n as i32;
            (_n & -_n) as usize
        }

        pub fn range_update(&mut self, l: usize, r: usize, x: M) {
            let mut l = l + self.m; let mut r = r + self.m;
            let l0 = l / Self::_lsb(l);
            let r0 = r / Self::_lsb(r) - 1;
            self._propagate_above(l0); self._propagate_above(r0);
            while l < r {
                if l & 1 == 1 {
                    self.lazy[l] = self.lazy[l].ope(&x);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    self.lazy[r] = self.lazy[r].ope(&x);
                }
                l >>= 1; r >>= 1;
            }
            self._recalc_above(l0); self._recalc_above(r0);
        }

        pub fn fold(&mut self, l: usize, r: usize) -> X {
            let mut l = l + self.m; let mut r = r + self.m;
            let l0 = l / Self::_lsb(l);
            let r0 = r / Self::_lsb(r) - 1;
            self._propagate_above(l0); self._propagate_above(r0);
            let mut vl = X::one();
            let mut vr = X::one();
            while l < r {
                if l & 1 == 1 {
                    vl = vl.ope(&self._eval_at(l));
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    vr = (self._eval_at(r)).ope(&vr);
                }
                l >>= 1; r >>= 1;
            }
            vl.ope(&vr)
        }

        pub fn get(&mut self, i: usize) -> X {
            self.fold(i, i + 1)
        }
    }
}

