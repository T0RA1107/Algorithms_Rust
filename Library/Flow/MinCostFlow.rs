pub mod min_cost_flow {

    pub mod min_cost_flow_trait {
        pub trait FlowCapacity<Cost>:
            Copy + Ord
            + std::ops::Add<Output = Self>
            + std::ops::Sub<Output = Self>
            + std::ops::Mul<Cost, Output = Cost>
            + std::fmt::Debug
        {
            fn zero() -> Self;
            fn inf() -> Self;
        }

        pub trait FlowCost:
            Copy + Ord
            + std::ops::Add<Output = Self>
            + std::ops::Sub<Output = Self>
            + std::ops::Mul<Output = Self>
            + std::ops::Neg<Output = Self>
            + std::fmt::Debug
        {
            fn zero() -> Self;
            fn inf() -> Self;
        }

        impl FlowCapacity<i64> for i64 {
            fn zero() -> i64 { 0_i64 }
            fn inf() -> i64 { std::i64::MAX }
        }

        impl FlowCost for i64 {
            fn zero() -> i64 { 0_i64 }
            fn inf() -> i64 { std::i64::MAX }
        }
    }

    use min_cost_flow_trait::*;

    #[derive(Clone)]
    pub struct Edge<Cap, Cost> {
        pub to: usize,
        pub inv: usize,
        pub cap: Cap,
        pub cost: Cost,
        pub is_inv: bool
    }

    impl<Cap: FlowCapacity<Cost>, Cost: FlowCost> Edge<Cap, Cost> {
        pub fn new(to: usize, inv: usize, cap: Cap, cost: Cost, is_inv: bool) -> Self {
            Edge { to, inv, cap, cost, is_inv }
        }
        pub fn add(&mut self, cap: Cap) {
            self.cap = self.cap + cap;
        }
        pub fn sub(&mut self, cap: Cap) {
            self.cap = self.cap - cap;
        }
    }

    #[derive(Clone)]
    pub struct FlowGraph<Cap, Cost> {
        pub graph: Vec<Vec<Edge<Cap, Cost>>>
    }

    impl<Cap: FlowCapacity<Cost>, Cost: FlowCost> FlowGraph<Cap, Cost> {
        pub fn new(n: usize) -> Self {
            Self { graph: vec![vec![]; n] }
        }
        pub fn add_edge(&mut self, from: usize, to: usize, cap: Cap, cost: Cost) {
            assert!(from.max(to) < self.graph.len(), "VERTICES NUMBER PROCEEDING");
            assert!(cap >= Cap::zero(), "NEGATIVE CAPACITY");
            assert!(from != to, "SELF LOOP");
            let x = self.graph[from].len();
            let y = self.graph[to].len();
            self.graph[from].push(Edge::new(to, y, cap, cost, false));
            self.graph[to].push(Edge::new(from, x, Cap::zero(), -cost, true));
        }
        pub fn init(&mut self) {
            for u in 0..self.graph.len() {
                for i in 0..self.graph[u].len() {
                    if self.graph[u][i].is_inv { continue; }
                    let (to, inv) = (self.graph[u][i].to, self.graph[u][i].inv);
                    let c = self.graph[u][i].cap;
                    let rc = self.graph[to][inv].cap;
                    self.graph[u][i].cap = c + rc;
                    self.graph[to][inv].cap = Cap::zero();
                }
            }
        }
        pub fn min_cost(&mut self, source: usize, sink: usize, flow: Cap) -> Option<Cost> {
            let n = self.graph.len();
            assert!(source.max(sink) < n, "VERTICES NUMBER PROCEEDING");
            assert!(source != sink, "SELF LOOP");
            let mut ret = Cost::zero();
            let mut flow = flow;
            let mut potential = vec![Cost::zero(); n];
            let mut prevv = vec![!0; n];
            let mut preve = vec![!0; n];
            while flow > Cap::zero() {
                let mut min_cost = vec![Cost::inf(); n];
                let mut hq = std::collections::BinaryHeap::new();
                hq.push((std::cmp::Reverse(Cost::zero()), source));
                min_cost[source] = Cost::zero();
                while let Some((std::cmp::Reverse(d), u)) = hq.pop() {
                    if min_cost[u] < d { continue; }
                    for (i, e) in self.graph[u].iter().enumerate() {
                        let next_cost = min_cost[u] + e.cost + potential[u] - potential[e.to];
                        if e.cap > Cap::zero() && min_cost[e.to] > next_cost {
                            min_cost[e.to] = next_cost;
                            prevv[e.to] = u;
                            preve[e.to] = i;
                            hq.push((std::cmp::Reverse(min_cost[e.to]), e.to));
                        }
                    }
                }
                if min_cost[sink] == Cost::inf() {
                    // flow > max_flow
                    return None;
                }
                for u in 0..n {
                    potential[u] = potential[u] + min_cost[u];
                }
                let mut add_flow = flow;
                let mut now = sink;
                while now != source {
                    add_flow = add_flow.min(self.graph[prevv[now]][preve[now]].cap);
                    now = prevv[now];
                }
                flow = flow - add_flow;
                ret = ret + add_flow * potential[sink];
                let mut now = sink;
                while now != source {
                    let (to, inv) = (self.graph[prevv[now]][preve[now]].to, self.graph[prevv[now]][preve[now]].inv);
                    let rc = self.graph[to][inv].cap;
                    self.graph[prevv[now]][preve[now]].cap = self.graph[prevv[now]][preve[now]].cap - add_flow;
                    self.graph[to][inv].cap = rc + add_flow;
                    now = prevv[now];
                }
            }
            Some(ret)
        }
    }
    impl<Cap: FlowCapacity<Cost>, Cost: FlowCost> std::fmt::Debug for FlowGraph<Cap, Cost> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for u in 0..self.graph.len() {
                for e in &self.graph[u] {
                    let inv = &self.graph[e.to][e.inv];
                    writeln!(f, "{} -> {} (flow: {:?} / {:?}, cost: {:?})", u, e.to, inv.cap, inv.cap + e.cap, inv.cap * e.cost).ok();
                }
            }
            write!(f, "")
        }
    }
}
