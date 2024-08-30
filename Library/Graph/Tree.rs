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
