#[derive(Debug)]
pub struct Cumsum(Vec<i64>);

impl Cumsum {
    // 0-indexed interface
    pub fn new(xs: &Vec<i64>) -> Self {
        let mut arr = vec![0; xs.len() + 1];
        for (i, &x) in xs.iter().enumerate() {
            arr[i + 1] = arr[i] + x;
        }
        Self(arr)
    }
    // sum of [0, idx)
    pub fn sum_up(&self, idx: usize) -> i64 {
        self.0[idx]
    }
    // sum of [i, j)
    pub fn sum(&self, i: usize, j: usize) -> i64 {
        self.0[j] - self.0[i]
    }
}
