pub struct PotentialUnionFind {
    _n: usize,
    par: Vec<i32>,
    potential: Vec<i64>
}

impl PotentialUnionFind {
    pub fn new(_n: usize) -> Self {
        PotentialUnionFind{ _n, par: vec![-1; _n], potential: vec![0; _n] }
    }

    pub fn find(&mut self, v: usize) -> usize {
        if self.par[v] < 0 { return v; }
        let r = self.find(self.par[v] as usize);
        self.potential[v] += self.potential[self.par[v] as usize];
        self.par[v] = r as i32;
        return self.par[v] as usize;
    }

    pub fn potential(&mut self, v: usize) -> i64 {
        self.find(v);
        self.potential[v]
    }

    pub fn union(&mut self, u: usize, v: usize, w: i64) -> bool {
        let mut w = w + self.potential(u) - self.potential(v);
        let mut pu = self.find(u);
        let mut pv = self.find(v);
        if pu == pv { return false; }
        if self.par[pu] > self.par[pv] {
            std::mem::swap(&mut pu, &mut pv);
            w = -w;
        }
        self.par[pu] += self.par[pv];
        self.par[pv] = pu as i32;
        self.potential[pv] = w;
        return true;
    }


    pub fn same(&mut self, u: usize, v: usize) -> bool {
        self.find(u) == self.find(v)
    }

    pub fn size(&mut self, v: usize) -> usize {
        let p = self.find(v);
        return -self.par[p] as usize;
    }
}
