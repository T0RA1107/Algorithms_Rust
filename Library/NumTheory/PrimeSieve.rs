pub struct PrimeSieve {
    min_prime_factor: Vec<usize>,
    primes: Vec<usize>
}

impl PrimeSieve {
    pub fn new(MAX: usize) -> Self {
        let mut min_prime_factor: Vec<usize> = (0..MAX).collect();
        for n in 2..MAX {
            if n * n > MAX { break; }
            if min_prime_factor[n] != n { continue; }
            let mut now = n * n;
            while now < MAX {
                if min_prime_factor[now] == now {
                    min_prime_factor[now] = n;
                }
                now += n;
            }
        }
        let primes: Vec<usize> = (0..MAX).filter(|&i| min_prime_factor[i] == i).collect();
        PrimeSieve { min_prime_factor, primes }
    }
    pub fn factorization(&self, n: usize) -> Vec<(usize, usize)> {
        let mut factors = vec![];
        let mut now = n;
        if n < self.min_prime_factor.len() {
            // O(log n)
            let mut pre = self.min_prime_factor[n];
            let mut cnt = 0;
            while now != 1 {
                now /= self.min_prime_factor[now];
                cnt += 1;
                if self.min_prime_factor[now] != pre {
                    factors.push((pre, cnt));
                    pre = self.min_prime_factor[now];
                    cnt = 0;
                }
            }
        } else {
            // O(√n)
            for &p in &self.primes {
                if now % p != 0 { continue; }
                let mut cnt = 0;
                while now % p == 0 {
                    now /= p;
                    cnt += 1;
                }
                factors.push((p, cnt));
            }
        }
        factors
    }
    pub fn is_prime(&self, n: usize) -> bool {
        if n < self.min_prime_factor.len() {
            self.min_prime_factor[n] == n
        } else {
            assert!(n < self.min_prime_factor.len().pow(2_u32), "n should be smaller than {}", self.min_prime_factor.len().pow(2_u32));
            for &p in &self.primes {
                if n % p == 0 {
                    return false;
                }
            }
            true
        }
    }
}
