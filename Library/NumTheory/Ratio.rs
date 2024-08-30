// n / m = Ratio(n, m)
// 1 / 0 as infinity
// - n / m = (-n) / m | n > 0, m > 0 

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ratio(i64, i64);

impl Ratio {
    pub fn new(a: i64, b: i64) -> Self {
        Ratio(a, b).normalize()
    }

    fn normalize(&mut self) -> Self {
        let g = Self::gcd(self.0.abs(), self.1.abs());
        self.0 /= g;
        self.1 /= g;
        if self.1 < 0 {
            self.0 *= -1;
            self.1 *= -1;
        }
        *self
    }

    pub fn from(x: i64) -> Self {
        Ratio(x, 1)
    }
    
    pub fn inv(&self) -> Self {
        if self.0 > 0 {
            Ratio(self.1, self.0)
        } else if self.0 < 0 {
            Ratio(-self.1, -self.0)
        } else {
            Ratio(1, 0)
        }
    }

    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    fn lcm(a: i64, b: i64) -> i64 {
        a / Self::gcd(a, b) * b
    }
}

impl std::ops::Add for Ratio {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let bottom = Self::lcm(self.1, rhs.1);
        let top = self.0 * bottom / self.1 + rhs.0 * bottom / rhs.1;
        Ratio(top, bottom)
    }
}

impl std::ops::AddAssign for Ratio {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl std::ops::Neg for Ratio {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Ratio(-self.0, self.1)
    }
}

impl std::ops::Sub for Ratio {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl std::ops::SubAssign for Ratio {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl std::ops::Mul for Ratio {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Ratio::new(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl std::ops::MulAssign for Ratio {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::Div for Ratio {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl std::ops::DivAssign for Ratio {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    } 
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        let left = self.0 * rhs.1;
        let right = rhs.0 * self.1;
        left.partial_cmp(&right)
    }
}

impl Ord for Ratio {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&rhs).unwrap()
    }
}