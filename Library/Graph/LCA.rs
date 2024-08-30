pub struct LCA {
    depth: Vec<usize>,
    table: Vec<Vec<usize>>,
}

impl LCA {
    pub fn new(N: usize, root: usize, graph: Vec<Vec<usize>>, MAX: usize) -> Self {
        let mut par = vec![root; N];
        let mut depth = vec![0; N];
        LCA::_dfs(root, &graph, &mut par, &mut depth);
        let mut table = vec![vec![0; N]; MAX];
        LCA::_doubling(&par, &mut table);
        LCA { depth, table }
    }
    fn _dfs(u: usize, G: &Vec<Vec<usize>>, par: &mut Vec<usize>, depth: &mut Vec<usize>) {
        for &v in &G[u] {
            if v == par[u] { continue; }
            par[v] = u;
            depth[v] = depth[u] + 1;
            LCA::_dfs(v, G, par, depth);
        }
    }
    fn _doubling(par: &Vec<usize>, table: &mut Vec<Vec<usize>>) {
        let MAX = table.len();
        let N = par.len();
        for (u, &v) in par.iter().enumerate() {
            table[0][u] = v;
        }
        for i in 1..MAX {
            for j in 0..N {
                table[i][j] = table[i - 1][table[i - 1][j]];
            }
        }
    }
    pub fn lca(&self, u: usize, v: usize) -> usize {
        let mut u = u; let mut v = v;
        if self.depth[u] > self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }
        let dd = self.depth[v] - self.depth[u];
        let MAX = self.table.len();
        for i in 0..MAX {
            if dd >> i == 0 { break; }
            else if dd >> i & 1 == 1 {
                v = self.table[i][v];
            }
        }
        if u == v { u }
        else {
            for i in (0..MAX).rev() {
                if self.table[i][u] != self.table[i][v] {
                    u = self.table[i][u];
                    v = self.table[i][v];
                }
            }
            self.table[0][u]
        }
    }
    pub fn distance(&self, u: usize, v: usize) -> usize {
        let l = self.lca(u, v);
        self.depth[u] + self.depth[v] - 2 * self.depth[l]
    }
}