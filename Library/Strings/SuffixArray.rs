pub fn IDCompress<T: Clone + Ord>(xs: &Vec<T>) -> Vec<usize> {
    let xset: std::collections::BTreeSet<&T> = xs.iter().collect();
    let xmap: std::collections::BTreeMap<&T, usize> =
        xset.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    xs.iter().map(|x| xmap[x]).collect()
}

pub struct SuffixArray {
    pub S: Vec<usize>,
    pub SA: Vec<usize>,
    pub ISA: Vec<usize>,
    pub LCP: Vec<usize>
}

impl SuffixArray {
    pub fn new(S: &Vec<char>) -> Self {
        let S = IDCompress(&Self::into_num(S));
        let N = S.len();
        let mut sa = SuffixArray {
            S, SA: (0..N).collect(), ISA: vec![!0; N], LCP: vec![0; N]
        };
        sa.make_property();
        sa
    }
    fn into_num(S: &Vec<char>) -> Vec<usize> {
        if b'a' <= S[0] as u8 && S[0] as u8 <= b'z' {
            return S.iter().map(|&s| (s as u8 - b'a') as usize).collect();
        } else {
            return S.iter().map(|&s| (s as u8 - b'A') as usize).collect();
        }
    }
    fn count_sort(&self, key: &Vec<usize>) -> Vec<usize> {
        let mut cnt = vec![0; self.S.len() + 1];
        for &x in key {
            cnt[x + 1] += 1;
        }
        for x in 0..self.S.len() {
            cnt[x + 1] += cnt[x];
        }
        let mut I = vec![0; self.S.len()];
        for i in 0..self.S.len() {
            I[cnt[key[i]]] = i;
            cnt[key[i]] += 1;
        }
        I
    }
    fn make_property(&mut self) {
        let N = self.S.len();
        let mut rank = self.S.clone();
        let mut k = 1;
        while k < N {
            let mut key1 = vec![0; N];
            for i in 0..N - k {
                key1[i] = 1 + rank[i + k];
            }

            // SAの更新
            let I = self.count_sort(&key1);
            let key2 = I.iter().map(|&i| rank[i]).collect::<Vec<_>>();
            let J = self.count_sort(&key2);
            self.SA = J.iter().map(|&j| I[j]).collect();

            // rankの更新
            let key = (0..N).map(|i| (key2[J[i]], key1[self.SA[i]])).collect::<Vec<_>>();
            rank[self.SA[0]] = 0;
            for i in 1..N {
                rank[self.SA[i]] = rank[self.SA[i - 1]];
                if key[i - 1] < key[i] { rank[self.SA[i]] += 1; }
            }
            k <<= 1;
        }

        for i in 0..N {
            self.ISA[self.SA[i]] = i;
        }
        let mut h = 0;
        for i in 0..N {
            if h > 0 { h -= 1; }
            if self.ISA[i] == 0 { continue; }
            let j = self.SA[self.ISA[i] - 1];
            while j + h < N && i + h < N {
                if self.S[j + h] != self.S[i + h] { break; }
                h += 1;
            }
            self.LCP[self.ISA[i] - 1] = h;
        }
    }
}

