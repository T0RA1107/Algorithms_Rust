#[derive(Debug, Clone, Copy)]
pub struct Point(i64, i64);

impl PartialEq for Point {
    fn eq(&self, rhs: &Point) -> bool {
        self.0 == rhs.0 && self.1 == rhs.1
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
        (self.0, self.1).cmp(&(rhs.0, rhs.1))
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
impl std::ops::Mul<Point> for i64 {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        Point(self * rhs.0, self * rhs.1)
    }
}
impl std::ops::Mul<i64> for Point {
    type Output = Point;
    fn mul(self, rhs: i64) -> Point {
        Point(rhs * self.0, rhs * self.1)
    }
}
// inner-product
impl std::ops::Mul<Point> for Point {
    type Output = i64;
    fn mul(self, rhs: Point) -> i64 {
        self.0 * rhs.0 + self.1 * rhs.1
    }
}
impl std::ops::Div<i64> for Point {
    type Output = Point;
    fn div(self, rhs: i64) -> Point {
        Point(self.0 / rhs, self.1 / rhs)
    }
}

impl Point {
    pub const PI: f64 = std::f64::consts::PI;
    pub fn new(x: i64, y: i64) -> Self {
        Point(x, y)
    }
    pub fn from(p: (i64, i64)) -> Self {
        Point(p.0, p.1)
    }
    pub fn zero() -> Self {
        Point(0, 0)
    }
    pub fn norm(&self) -> f64 {
        (self.norm2() as f64).sqrt()
    }
    pub fn norm2(&self) -> i64 {
        *self * *self
    }
    pub fn cross(&self, rhs: &Point) -> i64 {
        self.0 * rhs.1 - self.1 * rhs.0
    }
    pub fn distance(&self, rhs: &Point) -> f64 {
        (*self - *rhs).norm()
    }
    pub fn distance2(&self, rhs: &Point) -> i64 {
        (*self - *rhs).norm2()
    }
}
