use itertools::Itertools;

pub struct Diameter {
    pub D: usize,
    pub left: usize,
    pub right: usize,
    pub from_left: Vec<usize>,
    pub from_right: Vec<usize>
}
impl Diameter {
    pub fn new(G: &Vec<Vec<usize>>) -> Self {
        let n = G.len();
        let mut dist = vec![0; n];
        Diameter::_dfs(0, 0, &G, &mut dist);
        let left = dist.iter().position_max().unwrap();
        let mut from_left = vec![0; n];
        Diameter::_dfs(left, left, &G, &mut from_left);
        let right = from_left.iter().position_max().unwrap();
        let mut from_right = vec![0; n];
        Diameter::_dfs(right, right, &G, &mut from_right);
        Self {
            D: from_left[right], left, right, from_left, from_right
        }
    }
    fn _dfs(u: usize, par: usize, G: &Vec<Vec<usize>>, dist: &mut Vec<usize>) {
        for &v in &G[u] {
            if v == par { continue; }
            dist[v] = dist[u] + 1;
            Diameter::_dfs(v, u, G, dist);
        }
    }
}
