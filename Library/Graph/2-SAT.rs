pub mod scc {
    // nord i belongs to the group cmp[i]
    // dag is the adjective list of cmp

    pub struct SCC {
        N: usize,
        E: Vec<(usize, usize)>,
        is_calculated: bool,
        cmp: Vec<usize>,
        dag: Vec<Vec<usize>>
    }

    impl SCC {
        pub fn new(N: usize) -> Self {
            SCC { N, E: vec![], is_calculated: false, cmp: vec![], dag: vec![] }
        }
        #[inline]
        pub fn add_edge(&mut self, u: usize, v: usize) {
            self.E.push((u, v));
            self.is_calculated = false;
        }
        pub fn calculate(&mut self) {
            let mut G = vec![vec![]; self.N];
            let mut G_rev = vec![vec![]; self.N];
            for &(u, v) in &self.E {
                G[u].push(v);
                G_rev[v].push(u);
            }
            let mut idx = vec![];
            let mut searched = vec![false; self.N];
            for u in 0..self.N {
                if !searched[u] {
                    self._dfs(u, &G, &mut searched, &mut idx);
                }
            }
            searched = vec![false; self.N];
            self.cmp = vec![self.N; self.N];
            let mut num = 0;
            for &u in idx.iter().rev() {
                if !searched[u] {
                    self._rdfs(u, &G_rev, &mut searched, num);
                    num += 1;
                }
            }
            self.dag = vec![vec![]; num];
            for u in 0..self.N {
                let u2 = self.cmp[u];
                for &v in &G[u] {
                    let v2 = self.cmp[v];
                    if u2 != v2 {
                        self.dag[u2].push(v2);
                    }
                }
            }
            for u in 0..num {
                self.dag[u].sort();
                self.dag[u].dedup();
            }
            self.is_calculated = true;
        }
        pub fn from(N: usize, E: Vec<(usize, usize)>) -> Self {
            let mut scc = SCC::new(N);
            scc.E = E;
            scc.calculate();
            scc
        }
        fn _dfs(
            &self,
            u: usize,
            G: &Vec<Vec<usize>>,
            searched: &mut Vec<bool>,
            idx: &mut Vec<usize>
        ) {
            searched[u] = true;
            for &v in &G[u] {
                if searched[v] { continue; }
                self._dfs(v, G, searched, idx);
            }
            idx.push(u);
        }
        fn _rdfs(
            &mut self,
            u: usize,
            G_rev: &Vec<Vec<usize>>,
            searched: &mut Vec<bool>,
            num: usize,
        ) {
            self.cmp[u] = num;
            searched[u] = true;
            for &v in &G_rev[u] {
                if searched[v] { continue; }
                self._rdfs(v, G_rev, searched, num);
            }
        }
        #[inline]
        pub fn cmp(&mut self) -> &Vec<usize> {
            if !self.is_calculated {
                self.calculate()
            }
            &self.cmp
        }
        #[inline]
        pub fn dag(&mut self) -> &Vec<Vec<usize>> {
            if !self.is_calculated {
                self.calculate()
            }
            &self.dag
        }
    }
}

pub mod two_sat {
    use crate::scc::SCC;
    pub struct TwoSAT {
        N: usize,
        scc: SCC,
        answer: Vec<bool>
    }
    impl TwoSAT {
        pub fn new(N: usize) -> Self {
            TwoSAT { N, scc: SCC::new(N + N), answer: vec![false; N] }
        }
        #[inline]
        pub fn add_clause(&mut self, x: usize, f: bool, y: usize, g: bool) {
            assert!(x < self.N && y < self.N);
            self.scc.add_edge(2 * x + !f as usize, 2 * y + g as usize);
            self.scc.add_edge(2 * y + !g as usize, 2 * x + f as usize);
        }
        pub fn satisfiable(&mut self) -> bool {
            let id = self.scc.cmp();
            for i in 0..self.N {
                if id[2 * i] == id[2 * i + 1] {
                    return false;
                }
                self.answer[i] = id[2 * i] < id[2 * i + 1];
            }
            true
        }
        pub fn answer(&self) -> &Vec<bool> {
            &self.answer
        }
    }
}
