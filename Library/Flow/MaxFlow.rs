pub mod maxflow {

    pub mod max_flow_trait {
        pub trait FlowCapacity:
            Copy + Ord
            + std::ops::Add<Output = Self>
            + std::ops::Sub<Output = Self>
            + std::fmt::Debug
        {
            fn zero() -> Self;
            fn inf() -> Self;
        }

        impl FlowCapacity for i64 {
            fn zero() -> i64 { 0_i64 }
            fn inf() -> i64 { std::i64::MAX }
        }
    }

    use max_flow_trait::*;

    #[derive(Clone)]
    pub struct Edge<Cap> {
        pub to: usize,
        pub inv: usize,
        pub cap: Cap
    }

    impl<Cap: FlowCapacity> Edge<Cap> {
        pub fn new(to: usize, inv: usize, cap: Cap) -> Self {
            Edge { to, inv, cap }
        }
        pub fn add(&mut self, cap: Cap) {
            self.cap = self.cap + cap;
        }
        pub fn sub(&mut self, cap: Cap) {
            self.cap = self.cap - cap;
        }
    }

    #[derive(Clone)]
    pub struct FlowGraph<Cap> {
        pub graph: Vec<Vec<Edge<Cap>>>
    }

    impl<Cap: FlowCapacity> FlowGraph<Cap> {
        pub fn new(n: usize) -> Self {
            Self { graph: vec![vec![]; n] }
        }
        pub fn add_edge(&mut self, from: usize, to: usize, cap: Cap) {
            assert!(from.max(to) < self.graph.len(), "VERTICES NUMBER PROCEEDING");
            assert!(cap >= Cap::zero(), "NEGATIVE CAPACITY");
            assert!(from != to, "SELF LOOP");
            let x = self.graph[from].len();
            let y = self.graph[to].len();
            self.graph[from].push(Edge::new(to, y, cap));
            self.graph[to].push(Edge::new(from, x, Cap::zero()));
        }
        pub fn flow(&mut self, source: usize, sink: usize) -> Cap {
            let n = self.graph.len();
            assert!(source.max(sink) < n, "VERTICES NUMBER PROCEEDING");
            assert!(source != sink, "SELF LOOP");
            let mut ret = Cap::zero();
            loop {
                let level = (|| -> Vec<usize> {
                    let mut level = vec![0; n];
                    level[source] = 1;
                    let mut dq = std::collections::VecDeque::new();
                    dq.push_back(source);
                    while let Some(u) = dq.pop_front() {
                        for e in self.graph[u].iter() {
                            let v = e.to;
                            if e.cap > Cap::zero() && level[v] == 0 {
                                level[v] = level[u] + 1;
                                if v == sink {
                                    return level;
                                }
                                dq.push_back(v);
                            }
                        }
                    }
                    level
                })();
                if level[sink] == 0 { break; }
                let mut it = vec![0; n];
                loop {
                    let f = self._dfs(sink, source, Cap::inf(), &mut it, &level);
                    if f == Cap::zero() { break; }
                    ret = ret + f;
                }
            }
            ret
        }
        fn _dfs(&mut self, u: usize, source: usize, cap: Cap, it: &mut Vec<usize>, level: &Vec<usize>) -> Cap {
            if u == source { return cap; }
            while let Some((v, inv)) = self.graph[u].get(it[u]).map(|p| (p.to, p.inv)) {
                if level[v] + 1 == level[u] && self.graph[v][inv].cap > Cap::zero() {
                    let cap = cap.min(self.graph[v][inv].cap);
                    let c = self._dfs(v, source, cap, it, level);
                    if c > Cap::zero() {
                        self.graph[u][it[u]].add(c);
                        self.graph[v][inv].sub(c);
                        return c;
                    }
                }
                it[u] += 1;
            }
            Cap::zero()
        }
    }
    impl<Cap: FlowCapacity> std::fmt::Debug for FlowGraph<Cap> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for u in 0..self.graph.len() {
                for e in &self.graph[u] {
                    let inv = &self.graph[e.to][e.inv];
                    writeln!(f, "{} -> {} (flow: {:?} / {:?})", u, e.to, inv.cap, inv.cap + e.cap).ok();
                }
            }
            write!(f, "")
        }
    }
}
