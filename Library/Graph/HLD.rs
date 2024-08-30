pub mod directed_tree {
    #[derive(Debug, Clone)]
    pub struct DirectedTree<T> {
        n: usize,
        root: usize,
        children: Vec<Vec<(usize, T)>>,
        parents: Vec<Option<(usize, T)>>
    }

    impl<T: Copy> DirectedTree<T> {
        pub fn new(root: usize, n: usize, E: &Vec<(usize, usize, T)>) -> Self {
            let mut G = vec![vec![]; n];
            for &(u, v, d) in E {
                G[u].push((v, d));
                G[v].push((u, d));
            }
            let mut children = vec![vec![]; n];
            let mut parents = vec![None; n];
            DirectedTree::_dfs(root, &G, &mut children, &mut parents);
            DirectedTree { n, root, children, parents }
        }
        fn _dfs(u: usize,
                G: &Vec<Vec<(usize, T)>>,
                children: &mut Vec<Vec<(usize, T)>>,
                parents: &mut Vec<Option<(usize, T)>>)
        {
            for &(v, d) in &G[u] {
                if let Some((p, _)) = parents[u] {
                    if v == p { continue; }
                }
                parents[v] = Some((u, d));
                children[u].push((v, d));
                DirectedTree::_dfs(v, G, children, parents);
            }
        }
        pub fn root(&self) -> usize { self.root }
        pub fn len(&self) -> usize { self.n }
        pub fn parent(&self, v: usize) -> Option<(usize, T)> { self.parents[v] }
        pub fn children(&self, v: usize) -> std::slice::Iter<(usize, T)> { self.children[v].iter() }
        pub fn into_norm(&self) -> DirectedTreeNorm {
            let mut children = vec![];
            let mut parents = vec![];
            for i in 0..self.n {
                children.push(self.children[i].iter().map(|&u| u.0).collect::<Vec<usize>>());
                if let Some(u) = self.parents[i] {
                    parents.push(Some(u.0))
                } else {
                    parents.push(None)
                }
            }
            DirectedTreeNorm {
                n: self.n,
                root: self.root,
                children,
                parents
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct DirectedTreeNorm {
        n: usize,
        root: usize,
        children: Vec<Vec<usize>>,
        parents: Vec<Option<usize>>
    }

    impl DirectedTreeNorm {
        pub fn new(root: usize, n: usize, E: &Vec<(usize, usize)>) -> Self {
            let mut G = vec![vec![]; n];
            for &(u, v) in E {
                G[u].push(v);
                G[v].push(u);
            }
            let mut children = vec![vec![]; n];
            let mut parents = vec![None; n];
            DirectedTreeNorm::_dfs(root, &G, &mut children, &mut parents);
            DirectedTreeNorm { n, root, children, parents }
        }
        fn _dfs(u: usize,
                G: &Vec<Vec<usize>>,
                children: &mut Vec<Vec<usize>>,
                parents: &mut Vec<Option<usize>>)
        {
            for &v in &G[u] {
                if Some(v) == parents[u] { continue; }
                parents[v] = Some(u);
                children[u].push(v);
                DirectedTreeNorm::_dfs(v, G, children, parents);
            }
        }
        pub fn from_P(n: usize, P: &Vec<usize>) -> Self {
            let mut parents = vec![None; n];
            let mut children = vec![vec![]; n];
            for (i, &p) in P.iter().enumerate() {
                parents[i + 1] = Some(p);
                children[p].push(i + 1);
            }
            DirectedTreeNorm { n, root: 0, children, parents }
        }
        pub fn root(&self) -> usize { self.root }
        pub fn len(&self) -> usize { self.n }
        pub fn parent(&self, v: usize) -> Option<usize> { self.parents[v] }
        pub fn children(&self, v: usize) -> std::slice::Iter<usize> { self.children[v].iter() }
    }
}

pub mod hld {
    use crate::directed_tree::*;

    pub struct HLD {
        pub seq: Vec<usize>,
        pub tree: DirectedTreeNorm,
        pub subtree_size: Vec<usize>,
        pub heavy: Vec<Option<usize>>,
        pub seq_head: Vec<usize>,
        pub t_in: Vec<usize>,
        pub t_out: Vec<usize>
    }

    impl HLD {
        pub fn new(root: usize, n: usize, E: &Vec<(usize, usize)>) -> Self {
            let tree = DirectedTreeNorm::new(root, n, E);
            HLD::from(&tree)
        }
        pub fn from(tree: &DirectedTreeNorm) -> Self {
            let mut hld = HLD {
                seq: vec![],
                tree: tree.clone(),
                subtree_size: vec![0; tree.len()],
                heavy: vec![None; tree.len()],
                seq_head: vec![!0; tree.len()],
                t_in: vec![0; tree.len()],
                t_out: vec![0; tree.len()]
            };
            hld._dfs_size(tree, tree.root());
            hld.seq_head[tree.root()] = tree.root();
            hld._dfs_hld(tree, tree.root(), &mut 0);
            hld
        }
        fn _dfs_size(&mut self, tree: &DirectedTreeNorm, u: usize) {
            self.subtree_size[u] = 1;
            for &v in tree.children(u) {
                self._dfs_size(tree, v);
                self.subtree_size[u] += self.subtree_size[v];
            }
            self.heavy[u] = tree.children(u).max_by_key(|&&i| self.subtree_size[i]).map(|&x| x);
        }
        fn _dfs_hld(&mut self, tree: &DirectedTreeNorm, u: usize, t: &mut usize) {
            self.t_in[u] = *t;
            self.seq.push(u);
            *t += 1;
            if let Some(h) = self.heavy[u] {
                self.seq_head[h] = self.seq_head[u];
                self._dfs_hld(tree, h, t);
                for &v in tree.children(u).filter(|&&v| v != h) {
                    self.seq_head[v] = v;
                    self._dfs_hld(tree, v, t);
                }
            }
            self.t_out[u] = *t;
        }
        pub fn sequence(&self) -> std::slice::Iter<usize> { self.seq.iter() }
        pub fn lca(&self, u: usize, v: usize) -> usize {
            let (mut u, mut v) = (u, v);
            while u != v {
                if self.t_in[u] > self.t_in[v] { std::mem::swap(&mut u, &mut v); }
                if self.seq_head[u] == self.seq_head[v] { break; }
                v = self.tree.parent(self.seq_head[v]).unwrap();
            }
            u
        }
    }
}
