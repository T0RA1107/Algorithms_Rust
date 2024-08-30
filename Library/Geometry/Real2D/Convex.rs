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

#[derive(Debug, Clone)]
pub struct Polygon(Vec<Point>); // Points must be sorted by counter-clockwise order

impl std::ops::Index<usize> for Polygon {
    type Output = Point;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl Polygon {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn contains(&self, p: &Point, on_edge: bool) -> bool {
        let n = self.len();
        for i in 0..n {
            let u = self[i];
            let v = self[(i + 1) % n];
            let edge = Line::new(u, v);
            match edge.ccw(p) {
                0 => {
                    if !on_edge {
                        return false;
                    }
                },
                1 => { continue },
                _ => { return false; }
            }
        }
        true
    }
    pub fn area(&self) -> f64 {
        (1..self.len() - 1)
            .map(|i| {
                (self[i] - self[0]).cross(&(self[i + 1] - self[i])).abs()
            }).sum::<f64>() / 2.0
    }
}

#[derive(Debug, Clone)]
pub struct Convex(Polygon);

impl Convex {
    pub fn new(points: &Vec<Point>) -> Self {
        let mut p = points.clone();
        let n = p.len();
        p.sort();
        let mut G1 = vec![p[0], p[1]];
        let mut G2 = vec![p[0], p[1]];
        for i in 2..n {
            while G1.len() >= 2 && (G1[G1.len() - 1] - G1[G1.len() - 2]).cross(&(p[i] - G1[G1.len() - 1])) <= EPS {
                G1.pop();
            }
            while G2.len() >= 2 && (G2[G2.len() - 1] - G2[G2.len() - 2]).cross(&(p[i] - G2[G2.len() - 1])) >= -EPS {
                G2.pop();
            }
            G1.push(p[i]);
            G2.push(p[i]);
        }
        let mut ch = vec![];
        for i in 0..G1.len() {
            ch.push(G1[i]);
        }
        for i in (1..G2.len() - 1).rev() {
            ch.push(G2[i]);
        }
        Convex(Polygon(ch))
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn perimeter(&self) -> f64 {
        let mut l = 0.0;
        let n = self.len();
        for i in 0..n {
            l += self.0[i].distance(&self.0[(i + 1) % n]);
        }
        l
    }
    pub fn diameter(&self) -> f64 {
        let n = self.len();
        let (mut si, mut sj) = (0, 0);
        for t in 0..n {
            if self.0[t].1 < self.0[si].1 {
                si = t;
            }
            if self.0[t].1 > self.0[sj].1 {
                sj = t;
            }
        }
        let mut d: f64 = 0.0;
        let (mut i, mut j) = (si, sj);
        while {
            d = d.max(self.0[i].distance(&self.0[j]));
            let di = self.0[(i + 1) % n] - self.0[i];
            let dj = self.0[(j + 1) % n] - self.0[j];
            if di.cross(&dj) >= 0.0 {
                j = (j + 1) % n;
            } else {
                i = (i + 1) % n;
            }
            true
        } {
            if i == si && j == sj { break; }
        }
        d
    }
    pub fn area(&self) -> f64 {
        self.0.area()
    }
    pub fn convex_cut(&self, line: &Line) -> Convex {
        let n = self.len();
        let mut ret = vec![];
        for i in 0..n {
            let now = self.0[i]; let nxt = self.0[(i + 1) % n];
            if line.ccw(&now) != -1 {
                ret.push(now);
            }
            if line.ccw(&now) * line.ccw(&nxt) < 0 {
                ret.push(line.intersection(&Line(now, nxt)));
            }
        }
        Convex(Polygon(ret))
    }
}