pub struct Trie {
    child: Vec<Vec<Option<usize>>>,  // (nodes, 26) -> None or node id
    is_end: Vec<bool>,
    common: Vec<usize>
}

impl Trie {
    pub fn new() -> Self {
        Self { child: vec![vec![None; 26]], is_end: vec![false], common: vec![0] }
    }

    pub fn from(S: Vec<String>) -> Self {
        let mut trie = Self::new();
        for s in S {
            trie.insert(s);
        }
        trie
    }

    pub fn insert(&mut self, word: String) {
        let mut node = 0;
        self.common[node] += 1;
        for s in word.bytes() {
            if self.child[node][(s - b'a') as usize] == None {
                self.child[node][(s - b'a') as usize] = Some(self.size());
                self.child.push(vec![None; 26]);
                self.is_end.push(false);
                self.common.push(0);
            }
            node = self.child[node][(s - b'a') as usize].unwrap();
            self.common[node] += 1;
        }
        self.is_end[node] = true;
    }

    pub fn search_trie(&self, word: String, prefix: bool) -> bool {
        let mut node = 0;
        for s in word.bytes() {
            if self.child[node][(s - b'a') as usize] == None {
                return false;
            }
            node = self.child[node][(s - b'a') as usize].unwrap();
        }
        if prefix { return true; }
        if self.is_end[node] { return true; }
        false
    }

    pub fn size(&self) -> usize {
        self.child.len()
    }
}
