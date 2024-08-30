const INF: usize = 1 << 60;

pub struct MergeSortTree {
    pub size: usize,
    pub n: usize,
    pub data: Vec<Vec<usize>>,
    pub cum: Vec<Vec<usize>>
}

impl MergeSortTree {
    pub fn new(A: Vec<usize>) -> Self {
        let size = A.len().next_power_of_two();
        let n = size.trailing_zeros() as usize;
        let mut data = vec![vec![INF; size]; n + 1];
        let mut cum = vec![vec![0; size + 1]; n + 1];
        for (i, &a) in A.iter().enumerate() {
            data[n][i] = a;
            cum[n][i + 1] = cum[n][i] + a;
        }
        for d in (1..=n).rev() {
            let m = 1 << n - d + 1;
            for j in 0..1 << d - 1 {
                let mut now = j * m;
                let mut l = j * m;
                let mut r = j * m + m / 2;
                while now < (j + 1) * m {
                    let nxt = {
                        if l == j * m + m / 2 {
                            r += 1;
                            data[d][r - 1]
                        } else if r == (j + 1) * m {
                            l += 1;
                            data[d][l - 1]
                        } else if data[d][l] <= data[d][r] {
                            l += 1;
                            data[d][l - 1]
                        } else {
                            r += 1;
                            data[d][r - 1]
                        }
                    };
                    data[d - 1][now] = nxt;
                    cum[d - 1][now + 1] = cum[d - 1][now] + data[d - 1][now];
                    now += 1;
                }
            }
        }
        Self { size, n, data, cum }
    }
    // sum of the value y <= x in [l, r)
    pub fn fold(&self, l: usize, r: usize, x: usize) -> usize {
        let mut idx_l = l;
        let mut idx_r = r;
        let mut res = 0;
        for d in (0..=self.n).rev() {
            let m = 1 << self.n - d;
            if idx_l < idx_r {
                if idx_l & 1 == 1 {
                    // [idx_l << n - d + 1, (idx_l + 1) << n - d + 1)
                    let i = self.data[d][idx_l * m..(idx_l + 1) * m].upper_bound(x);
                    res += self.cum[d][idx_l * m + i] - self.cum[d][idx_l * m];
                    idx_l += 1;
                }
                if idx_r & 1 == 1 {
                    idx_r -= 1;
                    // [idx_r << n - d + 1, (idx_r + 1) << n - d + 1)
                    let i = self.data[d][idx_r * m..(idx_r + 1) * m].upper_bound(x);
                    res += self.cum[d][idx_r * m + i] - self.cum[d][idx_r * m];
                }
                idx_l >>= 1; idx_r >>= 1;
            } else {
                break;
            }
        }
        res
    }
}
