pub mod modint {
    use std::marker::PhantomData;

    pub trait Modulo {
        fn modulo() -> u32;
    }

    pub struct ModInt<T>(u32, PhantomData<T>);

    impl<T: Modulo> Clone for ModInt<T> {
        #[inline]
        fn clone(&self) -> Self {
            ModInt::new_unchecked(self.0)
        }
    }

    impl<T: Modulo> Copy for ModInt<T> {}

    impl<T: Modulo> std::ops::Add for ModInt<T> {
        type Output = ModInt<T>;
        #[inline]
        fn add(self, rhs: ModInt<T>) -> Self::Output {
            let mut ret = self.0 + rhs.0;
            if ret >= T::modulo() {
                ret -= T::modulo();
            }
            ModInt::new_unchecked(ret)
        }
    }

    impl<T: Modulo> std::ops::AddAssign for ModInt<T> {
        #[inline]
        fn add_assign(&mut self, rhs: ModInt<T>) {
            *self = *self + rhs;
        }
    }

    impl<T: Modulo> std::ops::Sub for ModInt<T> {
        type Output = ModInt<T>;
        #[inline]
        fn sub(self, rhs: ModInt<T>) -> Self::Output {
            let mut ret = self.0 + T::modulo() - rhs.0;
            if ret >= T::modulo() {
                ret -= T::modulo();
            }
            ModInt::new_unchecked(ret)
        }
    }

    impl<T: Modulo> std::ops::SubAssign for ModInt<T> {
        #[inline]
        fn sub_assign(&mut self, rhs: ModInt<T>) {
            *self = *self - rhs;
        }
    }

    impl<T: Modulo> std::ops::Mul for ModInt<T> {
        type Output = ModInt<T>;
        #[inline]
        fn mul(self, rhs: ModInt<T>) -> Self::Output {
            let v = self.0 as u64 * rhs.0 as u64 % T::modulo() as u64;
            ModInt::new_unchecked(v as u32)
        }
    }

    impl<T: Modulo> std::ops::MulAssign for ModInt<T> {
        #[inline]
        fn mul_assign(&mut self, rhs: ModInt<T>) {
            *self = *self * rhs;
        }
    }

    impl<T: Modulo> std::ops::Div for ModInt<T> {
        type Output = ModInt<T>;
        #[inline]
        fn div(self, rhs: ModInt<T>) -> Self::Output {
            self * rhs.inv()
        }
    }

    impl<T: Modulo> std::ops::DivAssign for ModInt<T> {
        #[inline]
        fn div_assign(&mut self, rhs: ModInt<T>) {
            *self = *self / rhs
        }
    }

    impl<T: Modulo> std::ops::Neg for ModInt<T> {
        type Output = ModInt<T>;
        #[inline]
        fn neg(self) -> Self::Output {
            ModInt::new_unchecked(if self.0 == 0 {0} else {T::modulo() - self.0})
        }
    }

    impl<T> std::fmt::Display for ModInt<T> {
        #[inline]
        fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl<T> std::fmt::Debug for ModInt<T> {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl<T: Modulo> std::str::FromStr for ModInt<T> {
        type Err = std::num::ParseIntError;
        #[inline]
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let val = s.parse::<u32>()?;
            Ok(ModInt::new_unchecked(val))
        }
    }

    impl<T: Modulo> From<u32> for ModInt<T> {
        #[inline]
        fn from(val: u32) -> ModInt<T> {
            ModInt::new_unchecked(val as u32 % T::modulo())
        }
    }

    impl<T: Modulo> From<u64> for ModInt<T> {
        #[inline]
        fn from(val: u64) -> ModInt<T> {
            ModInt::new_unchecked((val % T::modulo() as u64) as u32)
        }
    }

    impl<T: Modulo> From<usize> for ModInt<T> {
        #[inline]
        fn from(val: usize) -> ModInt<T> {
            ModInt::new_unchecked((val % T::modulo() as usize) as u32)
        }
    }

    impl<T: Modulo> From<i32> for ModInt<T> {
        #[inline]
        fn from(val: i32) -> ModInt<T> {
            ModInt::new_unchecked((val % T::modulo() as i32) as u32)
        }
    }

    impl<T: Modulo> From<i64> for ModInt<T> {
        #[inline]
        fn from(val: i64) -> ModInt<T> {
            ModInt::new_unchecked((val % T::modulo() as i64) as u32)
        }
    }

    impl<T: Modulo> ModInt<T> {
        #[inline]
        fn new_unchecked(n: u32) -> ModInt<T> {
            ModInt(n, PhantomData)
        }
        #[inline]
        pub fn new(n: u32) -> ModInt<T> {
            ModInt::new_unchecked(n % T::modulo())
        }
        #[inline]
        pub fn zero() -> ModInt<T> {
            ModInt::new_unchecked(0)
        }
        #[inline]
        pub fn one() -> ModInt<T> {
            ModInt::new_unchecked(1)
        }
        #[inline]
        pub fn pow(self, mut n: usize) -> ModInt<T> {
            let mut ret = ModInt::one();
            let mut base = self;
            while n > 0 {
                if n & 1 == 1 {
                    ret *= base;
                }
                base *= base;
                n >>= 1;
            }
            ret
        }
        #[inline]
        pub fn inv(self) -> ModInt<T> {
            assert!(self.0 != 0);
            self.pow(T::modulo() as usize - 2)
        }
    }

    impl<T: Modulo> std::iter::Sum for ModInt<T> {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            let mut res = Self::zero();
            for i in iter {
                res += i;
            }
            res
        }
    }
    
    pub struct Enumeration<T> {
        MAX: usize,
        factorial: Vec<ModInt<T>>,
        factorial_inv: Vec<ModInt<T>>
    }

    impl<T: Modulo> Enumeration<T> {
        pub fn new(MAX: usize) -> Self {
            let mut factorial = vec![ModInt::one(); MAX + 1];
            let mut factorial_inv = vec![ModInt::one(); MAX + 1];
            for n in 2..=MAX {
                factorial[n] = factorial[n - 1] * ModInt::from(n);
                factorial_inv[n] = factorial[n].inv();
            }
            Enumeration { MAX, factorial, factorial_inv }
        }
        #[inline]
        pub fn factorial(&self, n: usize) -> ModInt<T> {
            assert!(n <= self.MAX);
            self.factorial[n]
        }
        #[inline]
        pub fn P(&self, n: usize, k: usize) -> ModInt<T> {
            assert!(n <= self.MAX);
            if n < k { return ModInt::zero(); }
            self.factorial[n] * self.factorial_inv[n - k]
        }
        #[inline]
        pub fn C(&self, n: usize, k: usize) -> ModInt<T> {
            assert!(n <= self.MAX);
            if n < k { return ModInt::zero(); }
            self.factorial[n] * self.factorial_inv[k] * self.factorial_inv[n - k]
        }
        #[inline]
        pub fn H(&self, n: usize, k: usize) -> ModInt<T> {
            assert!(n <= self.MAX);
            self.C(n + k - 1, k)
        }
    }

    pub struct MOD998244353;
    impl Modulo for MOD998244353 {
        #[inline]
        fn modulo() -> u32 {
            998_244_353
        }
    }

    pub struct MOD1000000007;
    impl Modulo for MOD1000000007 {
        #[inline]
        fn modulo() -> u32 {
            1_000_000_007
        }
    }
}
