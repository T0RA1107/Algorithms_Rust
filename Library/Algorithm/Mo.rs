pub trait Mo {
    type Output: Default;

    // add i-th element
    fn add(&mut self, i: usize);

    // remove i-th element
    fn remove(&mut self, i: usize);

    // answer to the query in the current interval [l, r)
    fn query(&self) -> Self::Output;

    // answer to the all queries by Mo's algorithm
    fn mo(&mut self, N: usize, queries: &[(usize, usize)]) -> Vec<Self::Output> {
        let Q = queries.len();
        let B = ((N as f64 / (Q as f64).sqrt()) as usize).max(1);
        let mut left = Vec::with_capacity(Q);
        let mut right = Vec::with_capacity(Q);
        for &(l, r) in queries {
            left.push(l);
            right.push(r);
        }
        let mut order = (0..Q).collect::<Vec<_>>();
        order.sort_by(|&i, &j| {
            if left[i] / B == left[j] / B {
                right[i].cmp(&right[j])
            } else {
                left[i].cmp(&left[j])
            }
        });

        let mut ret = Vec::new();
        let mut l = 0;
        let mut r = 0;
        ret.resize_with(Q, Default::default);
        for i in order {
            while l > left[i] {
                l -= 1;
                self.add(l);
            }
            while r < right[i] {
                self.add(r);
                r += 1;
            }
            while l < left[i] {
                self.remove(l);
                l += 1;
            }
            while r > right[i] {
                r -= 1;
                self.remove(r);
            }
            ret[i] = self.query();
        }
        ret
    }
}
