pub mod FFT {
    use num::Complex;
    pub fn fast_fourier_transform(ary: &[Complex<f64>]) -> Vec<Complex<f64>> {
        let n = ary.len();
        assert!(n.count_ones() == 1, "the length of array should be power of two");
        let mut ret: Vec<_> = ary.to_vec();
        let bit = n.trailing_zeros() as usize;

        for si in (0..bit).rev() {
            let s = 1_usize << si;
            let zeta = Complex::from_polar(1.0, 2.0 * std::f64::consts::PI / (s << 1) as f64);
            for ii in 0..(n / (s << 1)) {
                let i = ii * (s << 1);
                let mut z_i = Complex::new(1.0, 0.0);
                for j in 0..s {
                    let t = ret[i + j] - ret[s + i + j];
                    ret[i + j] = ret[i + j] + ret[s + i + j];
                    ret[s + i + j] = t * z_i;
                    z_i *= zeta;
                }
            }
        }

        ret
    }

    pub fn inverse_fast_fourier_transform(ary: &[Complex<f64>]) -> Vec<Complex<f64>> {
        let n = ary.len();
        assert!(n.count_ones() == 1, "the length of array should be power of two");
        let mut ret: Vec<_> = ary.to_vec();
        let bit = n.trailing_zeros() as usize;

        for si in 0..bit {
            let s = 1_usize << si;
            let zeta = Complex::from_polar(1.0, -2.0 * std::f64::consts::PI / (s << 1) as f64);
            for ii in 0..(n / (s << 1)) {
                let i = ii * (s << 1);
                let mut z_i = Complex::new(1.0, 0.0);
                for j in 0..s {
                    let t = ret[s + i + j] * z_i;
                    ret[s + i + j] = ret[i + j] - t;
                    ret[i + j] = ret[i + j] + t;
                    z_i *= zeta;
                }
            }
        }

        let inv_n = Complex::new(1_f64 / n as f64, 0f64);
        ret.iter().map(|&x| x * inv_n).collect()
    }

    pub fn convolution(A: &[f64], B: &[f64]) -> Vec<f64> {
        let (n, m) = (A.len(), B.len());
        if A.len().min(B.len()) <= 60 {
            let (n, m, A, B) = if n < m { (m, n, B, A) } else { (n, m, A, B) };
            let mut ret = vec![0.0; n + m - 1];
            for i in 0..n {
                for j in 0..m {
                    ret[i + j] += A[i] * B[j];
                }
            }
            return ret;
        }
        let mut a: Vec<Complex<f64>> = A.iter().map(|&a| Complex::new(a as f64, 0.0)).collect();
        let mut b: Vec<Complex<f64>> = B.iter().map(|&a| Complex::new(a as f64, 0.0)).collect();
        let mut z = 1;
        while z < n + m - 1 { z <<= 1; }
        a.resize(z, Complex::new(0.0, 0.0)); b.resize(z, Complex::new(0.0, 0.0));
        let fa = fast_fourier_transform(&a);
        let fb = fast_fourier_transform(&b);
        let fc: Vec<Complex<f64>> = fa.iter().zip(&fb).map(|(&x, &y)| x * y).collect();
        let c = inverse_fast_fourier_transform(&fc);
        c.iter().take(n + m - 1).map(|&z| z.re).collect()
    }

    pub fn convolution_usize(A: &[usize], B: &[usize]) -> Vec<usize> {
        let a: Vec<f64> = A.iter().map(|&a| a as f64).collect();
        let b: Vec<f64> = B.iter().map(|&b| b as f64).collect();
        let c = convolution(&a, &b);
        c.iter().map(|&z| z.round() as usize).collect()
    }
}
