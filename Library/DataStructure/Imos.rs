pub struct Imos(Vec<i64>, bool);

impl Imos {
    pub fn new(N: usize) -> Self {
        Self(vec![0_i64; N + 1], false)
    }
    // add x to [l, r)
    pub fn add(&mut self, l: usize, r: usize, x: i64) {
        assert!(!self.1, "already built");
        self.0[l] += x;
        self.0[r] -= x;
    }
    pub fn build(&mut self) {
        assert!(!self.1, "already built");
        for i in 1..self.0.len() {
            self.0[i] += self.0[i - 1];
        }
        self.1 = true;
    }
    pub fn get(&self, idx: usize) -> i64 {
        assert!(self.1, "before build");
        self.0[idx - 1]
    }
}