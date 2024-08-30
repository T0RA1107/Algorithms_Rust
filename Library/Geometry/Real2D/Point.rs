pub const EPS: f64 = 1e-8;

#[derive(Debug, Clone, Copy)]
pub struct Point(f64, f64);

impl PartialEq for Point {
    fn eq(&self, rhs: &Point) -> bool {
        (self.0 - rhs.0).abs() < EPS && (self.1 - rhs.1).abs() < EPS
    }
    fn ne(&self, rhs: &Point) -> bool {
        !(self == rhs)
    }
}
impl Eq for Point {}
impl PartialOrd for Point {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}
impl Ord for Point {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        if self.0 != rhs.0 {
            self.0.partial_cmp(&rhs.0).unwrap()
        } else {
            self.1.partial_cmp(&rhs.1).unwrap()
        }
    }
}
impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::Neg for Point {
    type Output = Point;
    fn neg(self) -> Point {
        Point(-self.0, -self.1)
    }
}
impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        self + (-rhs)
    }
}
// scalar multiplication
impl std::ops::Mul<Point> for f64 {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        Point(self * rhs.0, self * rhs.1)
    }
}
impl std::ops::Mul<f64> for Point {
    type Output = Point;
    fn mul(self, rhs: f64) -> Point {
        Point(rhs * self.0, rhs * self.1)
    }
}
// inner-product
impl std::ops::Mul<Point> for Point {
    type Output = f64;
    fn mul(self, rhs: Point) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1
    }
}
impl std::ops::Div<f64> for Point {
    type Output = Point;
    fn div(self, rhs: f64) -> Point {
        Point(self.0 / rhs, self.1 / rhs)
    }
}

impl Point {
    pub const PI: f64 = std::f64::consts::PI;
    pub fn new(x: f64, y: f64) -> Self {
        Point(x, y)
    }
    pub fn from(p: (f64, f64)) -> Self {
        Point(p.0, p.1)
    }
    pub fn zero() -> Self {
        Point(0.0, 0.0)
    }
    pub fn norm(&self) -> f64 {
        (*self * *self).sqrt()
    }
    pub fn cross(&self, rhs: &Point) -> f64 {
        self.0 * rhs.1 - self.1 * rhs.0
    }
    pub fn arg(&self) -> f64 {
        self.1.atan2(self.0)
    }
    pub fn arg_delta(&self, rhs: &Point) -> f64 {
        let mut delta = self.arg() - rhs.arg();
        if delta < -EPS {
            delta += 2.0 * Point::PI;
        }
        delta
    }
    pub fn distance(&self, rhs: &Point) -> f64 {
        (*self - *rhs).norm()
    }
}