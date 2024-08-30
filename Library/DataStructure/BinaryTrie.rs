pub struct BinaryTrie {
    child: Vec<Vec<Option<usize>>>,
    is_end: Vec<bool>,
    common: Vec<usize>,
    MAX: usize
}

impl BinaryTrie {
    pub fn new(MAX: usize) -> Self {
        Self { child: vec![vec![None; 2]], is_end: vec![false], common: vec![0], MAX }
    }

    pub fn insert(&mut self, x: usize) {
        let mut node = 0;
        self.common[node] += 1;
        for i in (0..self.MAX).rev() {
            let j = x >> i & 1;
            if self.child[node][j] == None {
                self.child[node][j] = Some(self.size());
                self.child.push(vec![None; 26]);
                self.is_end.push(false);
                self.common.push(0);
            }
            node = self.child[node][j].unwrap();
            self.common[node] += 1;
        }
        self.is_end[node] = true;
    }

    pub fn count(&self, x: usize, xor: usize) -> usize {
        let x = x ^ xor;
        let mut cur = 0;
        for i in (0..self.MAX).rev() {
            let j = x >> i & 1;
            if self.child[cur][j] == None {
                return 0;
            }
            cur = self.child[cur][j].unwrap();
        }
        self.common[cur]
    }

    pub fn max(&self, xor: usize) -> usize {
        let mut cur = 0;
        let mut res = 0;
        for i in (0..self.MAX).rev() {
            let mut j = xor >> i & 1;
            if self.child[cur][j ^ 1] != None {
                j ^= 1;
            }
            cur = self.child[cur][j].unwrap();
            res |= j << i;
        }
        res ^ xor
    }

    pub fn min(&self, xor: usize) -> usize {
        let mut cur = 0;
        let mut res = 0;
        for i in (0..self.MAX).rev() {
            let mut j = xor >> i & 1;
            if self.child[cur][j] == None {
                j ^= 1;
            }
            cur = self.child[cur][j].unwrap();
            res |= j << i;
        }
        res ^ xor
    }

    pub fn size(&self) -> usize {
        self.child.len()
    }
}
