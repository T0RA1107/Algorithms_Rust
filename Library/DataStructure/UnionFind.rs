pub struct UnionFind {
    _n: usize,
    par: Vec<i32>
}

impl UnionFind {
    pub fn new(_n: usize) -> Self {
        UnionFind{ _n, par: vec![-1; _n] }
    }

    pub fn find(&mut self, v: usize) -> usize {
        if self.par[v] < 0 { return v; }
        self.par[v] = self.find(self.par[v] as usize) as i32;
        return self.par[v] as usize;
    }

    pub fn union(&mut self, u: usize, v: usize) -> bool {
        let pu = self.find(u);
        let pv = self.find(v);
        if pu == pv { return false; }
        if self.par[pu] < self.par[pv] {
            self.par[pu] += self.par[pv];
            self.par[pv] = pu as i32;
        } else {
            self.par[pv] += self.par[pu];
            self.par[pu] = pv as i32;
        }
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
