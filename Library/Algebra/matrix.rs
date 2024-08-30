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

use algebra::{Monoid, AGroup};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<K> {
    data: Vec<Vec<K>>
}

#[macro_export]
macro_rules! mat {
    ( $( $( $x:expr ),* );* ) => {
        Matrix::new( vec![ $( vec![ $( $x ),* ] ),* ] )
    }
}

impl<K> Matrix<K>
where
    K: Clone + Copy
{
    pub fn new(data: Vec<Vec<K>>) -> Self {
        Matrix { data }
    }
    pub fn size(&self) -> (usize, usize) {
        (self.data.len(), self.data[0].len())
    }
    pub fn map<F>(&self, f: F) -> Matrix<K>
    where
        F: Fn(&K) -> K,
    {
        let data = self.data
                       .iter()
                       .map(|row| row.iter().map(&f).collect())
                       .collect();
        Matrix::new(data)
    }
    pub fn t(&self) -> Matrix<K> {
        let (h, w) = self.size();
        let mut data = vec![vec![]; w];
        for j in 0..w {
            for i in 0..h {
                data[j].push(self.data[i][j]);
            }
        }
        Matrix { data }
    }
}

// implement for AGroup
impl<K: Copy + AGroup> Matrix<K> {
    pub fn zero(h: usize, w: usize) -> Matrix<K> {
        Matrix::new(vec![vec![K::zero(); w]; h])
    }
}
impl<K: Copy + AGroup> std::ops::Add for &Matrix<K> {
    type Output = Matrix<K>;
    fn add(self, other: Self) -> Self::Output {
        let (h1, w1) = self.size();
        let (h2, w2) = other.size();
        assert!(h1 == h2);
        assert!(w1 == w2);
        let data = (0..h1).map(|i| (0..w1).map(|j| self.data[i][j] + other.data[i][j]).collect())
                          .collect();
        Matrix::new(data)
    }
}
impl<K: Copy + AGroup> std::ops::Neg for &Matrix<K> {
    type Output = Matrix<K>;
    fn neg(self) -> Self::Output {
        self.map(|&x| -x)
    }
}

// implement for Monoid
impl<K: Copy + AGroup + Monoid> Matrix<K> {
    pub fn one(n: usize) -> Matrix<K> {
        let mut e = vec![vec![K::zero(); n]; n];
        for i in 0..n {
            e[i][i] = K::one();
        }
        Matrix::new(e)
    }
}
impl<K: Copy + AGroup + Monoid> std::ops::Mul<&Matrix<K>> for &Matrix<K> {
    type Output = Matrix<K>;
    fn mul(self, other: &Matrix<K>) -> Matrix<K> {
        let (h1, w1) = self.size();
        let (h2, w2) = other.size();
        assert!(w1 == h2);
        let data = (0..h1).map(|i| {
            (0..w2)
                .map(|j| (0..w1).map(|k| self.data[i][k].ope(&other.data[k][j])).sum())
                .collect()
        }).collect();
        Matrix::new(data)
    }
}

impl<K: Copy + AGroup + Monoid> Matrix<K> {
    pub fn pow(&self, n: usize) -> Matrix<K> {
        if n == 0 {
            Matrix::one(self.data.len())
        } else if n == 1 {
            self.clone()
        } else if n % 2 == 0 {
            let m = self * self;
            m.pow(n / 2)
        } else {
            let m = self * self;
            &m.pow(n / 2) * self
        }
    }
}
