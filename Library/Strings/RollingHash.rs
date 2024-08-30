pub struct RollingHash {
    pub base: usize,
    inv: usize,
    ary: Vec<usize>
}

impl RollingHash {
    const MOD: usize = (1 << 61) - 1;
    const MASK30: usize = (1 << 30) - 1;
    const MASK31: usize = (1 << 31) - 1;
    const MASK61: usize = Self::MOD;

    pub fn new(S: Vec<usize>) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let base = rng.gen_range(27..Self::MOD);
        let inv = Self::_power(base, Self::MOD - 2);
        let mut ary = vec![0; S.len() + 1];
        let mut b = base;
        for i in 0..S.len() {
            ary[i + 1] = (ary[i] + Self::_mul_mod(S[i], b)) % Self::MOD;
            b = Self::_mul_mod(b, base);
        }
        Self{ base, inv, ary }
    }

    pub fn from_string(S: String) -> Self {
        Self::from_bytes(S.bytes().collect())
    }

    pub fn from_bytes(S: Vec<u8>) -> Self {
        let S: Vec<usize> = S.iter().map(|&s| (s - b'a') as usize).collect();
        Self::new(S)
    }

    pub fn hash(&self, l: usize, r: usize) -> usize {
        let xr = self.ary[r];
        let xl = self.ary[l];
        let d = if xr >= xl { xr - xl } else { xr + Self::MOD - xl };
        Self::_mul_mod(d, Self::_power(self.inv, l))
    }

    fn _calc_mod(x: usize) -> usize {
        let xu = x >> 61;
        let xd = x & Self::MASK61;
        (xu + xd) % Self::MOD
    }

    fn _mul_mod(a: usize, b: usize) -> usize {
        let (au, bu) = (a >> 31, b >> 31);
        let (ad, bd) = (a & Self::MASK31, b & Self::MASK31);
        let mid = ad * bu + au * bd;
        let midu = mid >> 30;
        let midd = mid & Self::MASK30;
        Self::_calc_mod(au * bu * 2 + midu + (midd << 31) + ad * bd)
    }

    fn _power(a: usize, idx: usize) -> usize {
        let mut ret = 1;
        let mut idx = idx;
        let mut a = a;
        while idx > 0 {
            if idx & 1 == 1 {
                ret = Self::_mul_mod(ret, a);
            }
            idx >>= 1;
            a = Self::_mul_mod(a, a);
        }
        ret
    }
}
