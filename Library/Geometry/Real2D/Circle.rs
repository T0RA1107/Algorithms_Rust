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

#[derive(Debug, Clone, Copy)]
pub struct Line(pub Point, pub Point);

impl std::cmp::PartialEq for Line {
    fn eq(&self, rhs: &Line) -> bool {
        let a = Point::zero();
        let b = Point(1.0, 0.0);
        let c = Point(0.0, 1.0);
        for p in &[a, b, c] {
            if (self.distance_from(p) - rhs.distance_from(p)).abs() > EPS {
                return false;
            }
        }
        true
    }
}
impl std::cmp::Eq for Line {}

impl Line {
    pub fn new(x: Point, y: Point) -> Self {
        assert!(x != y, "cannot define Line");
        Line(x, y)
    }
    pub fn dir(&self) -> Point {
        let v = self.1 - self.0;
        v / v.norm()
    }
    pub fn extend(&self, k: f64) -> Line {
        let g = (self.0 + self.1) / 2.0;
        Line::new(g + (self.0 - g) * k / 2.0, g + (self.1 - g) * k / 2.0)
    }
    pub fn distance_from(&self, p: &Point) -> f64 {
        let u = *p - self.0;
        self.dir().cross(&u).abs()
    }
    pub fn project(&self, p: &Point) -> Point {
        self.0 + ((*p - self.0) * self.dir()) * self.dir()
    }
    pub fn ccw(&self, p: &Point) -> i32 {
        let u = self.1 - self.0;
        let v = *p - self.0;
        let c = u.cross(&v);
        if c > EPS {
            return 1;
        } else if c < -EPS {
            return -1;
        }
        if u * v < -EPS {
            return -2;
        } else if u.norm() < v.norm() {
            return 2;
        }
        0
    }
    pub fn reflect(&self, p: &Point) -> Point {
        let x = self.project(p);
        *p + 2.0 * (x - *p)
    }
    pub fn parallel(&self, rhs: &Line) -> bool {
        self.dir().cross(&rhs.dir()).abs() < EPS
    }
    pub fn vertical(&self, rhs: &Line) -> bool {
        (self.dir() * rhs.dir()).abs() < EPS
    }
    pub fn is_intersect(&self, rhs: &Line) -> bool {
        self.ccw(&rhs.0) * self.ccw(&rhs.1) <= 0 && rhs.ccw(&self.0) * rhs.ccw(&self.1) <= 0
    }
    pub fn intersection(&self, rhs: &Line) -> Point {
        assert!(self.is_intersect(rhs));
        let d1 = (self.1 - self.0).cross(&(rhs.1 - rhs.0));
        let d2 = (self.1 - self.0).cross(&(self.1 - rhs.0));
        if d1.abs() < EPS && d2.abs() < EPS {
            return rhs.0;
        }
        rhs.0 + (rhs.1 - rhs.0) * (d2 / d1)
    }
    pub fn LineIntersection(&self, rhs: &Line) -> Point{
        assert!(!self.parallel(&rhs));
        let d1 = (self.1 - self.0).cross(&(rhs.1 - rhs.0));
        let d2 = (self.1 - self.0).cross(&(self.1 - rhs.0));
        if d1.abs() < EPS && d2.abs() < EPS {
            return rhs.0;
        }
        rhs.0 + (rhs.1 - rhs.0) * (d2 / d1)
    }
    pub fn distanceBetweenSegmentAndPoint(&self, p: &Point) -> f64 {
        if (self.1 - self.0) * (*p - self.0) <= EPS {
            return (*p - self.0).norm();
        }
        if (self.0 - self.1) * (*p - self.1) <= EPS {
            return (*p - self.1).norm();
        }
        self.distance_from(&p)
    }
    pub fn distanceBetweenSegments(&self, rhs: &Line) -> f64 {
        if self.is_intersect(&rhs) {
            return 0.0;
        }
        let mut ret = self.distanceBetweenSegmentAndPoint(&rhs.0);
        ret = ret.min(self.distanceBetweenSegmentAndPoint(&rhs.1));
        ret = ret.min(rhs.distanceBetweenSegmentAndPoint(&self.0));
        ret = ret.min(rhs.distanceBetweenSegmentAndPoint(&self.1));
        ret
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    center: Point,
    radius: f64
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Self {
        assert!(radius >= 0.0);
        Circle { center, radius }
    }
    pub fn area(&self) -> f64 {
        self.radius.powi(2) * std::f64::consts::PI
    }
    pub fn is_intersect(&self, c: &Circle) -> i32 {
        let d = self.center.distance(&c.center);
        if d > self.radius + c.radius {
            return 4;
        }
        if d == self.radius + c.radius {
            return 3;
        }
        if (self.radius - c.radius).abs() < d - EPS && d < self.radius + c.radius - EPS {
            return 2;
        }
        if d == (self.radius - c.radius).abs() {
            return 1;
        }
        0
    }
    pub fn contains(&self, p: &Point) -> bool {
        self.center.distance(&p) <= self.radius + EPS
    }
    pub fn inner_center(a: Point, b: Point, c: Point) -> Circle {
        let da = b.distance(&c);
        let db = c.distance(&a);
        let dc = a.distance(&b);
        let center = (da * a + db * b + dc * c) / (da + db + dc);
        let radius = Line(b, c).distance_from(&center);
        Circle { center, radius }
    }
    pub fn circumcenter(a: Point, b: Point, c: Point) -> Circle {
        let ab = (a + b) / 2.0;
        let ac = (a + c) / 2.0;
        let atob = b - a;
        let atoc = c - a;
        let line1 = Line(ab, ab + Point(atob.1, -atob.0));
        let line2 = Line(ac, ac + Point(atoc.1, -atoc.0));
        let center = line1.LineIntersection(&line2);
        let radius = a.distance(&center);
        Circle { center, radius }
    }
}