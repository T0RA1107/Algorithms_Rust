pub struct LowLink {
    pub aps: std::collections::BTreeSet<usize>,
    pub bridges: std::collections::BTreeSet<(usize, usize)>,
    pub ord: Vec<usize>,
    pub low: Vec<usize>,
    pub par: Vec<Option<usize>>
}

impl LowLink {
    pub fn new(G: &Vec<Vec<usize>>) -> Self {
        let mut lowlink = LowLink {
            aps: std::collections::BTreeSet::new(),
            bridges: std::collections::BTreeSet::new(),
            ord: vec![0; G.len()],
            low: vec![0; G.len()],
            par: vec![None; G.len()]
        };
        let mut t = 0;
        let mut visited = vec![false; G.len()];
        for i in 0..G.len() {
            if visited[i] { continue; }
            visited[i] = true;
            lowlink._dfs(G, i, &mut t, &mut visited);
        }
        lowlink
    }
    fn _dfs(
        &mut self,
        G: &Vec<Vec<usize>>,
        u: usize,
        t: &mut usize,
        visited: &mut Vec<bool>,
    ) {
        visited[u] = true;
        self.ord[u] = *t;
        self.low[u] = self.ord[u];
        *t += 1;
        let mut is_aps = false;
        let mut cnt = 0;
        for &v in &G[u] {
            if !visited[v] {
                self.par[v] = Some(u);
                cnt += 1;
                self._dfs(G, v, t, visited);
                self.low[u] = self.low[u].min(self.low[v]);
                if self.par[u] != None && self.ord[u] <= self.low[v] {
                    is_aps = true;
                }
                if self.ord[u] < self.low[v] {
                    self.bridges.insert((u.min(v), v.max(u)));
                }
            } else if Some(v) != self.par[u] {
                self.low[u] = self.low[u].min(self.ord[v]);
            }
        }
        if self.par[u] == None && cnt >= 2 { is_aps = true; }
        if is_aps {
            self.aps.insert(u);
        }
    }
}
