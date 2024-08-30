pub fn gcd_ex(x: i64, y: i64) -> (i64, i64, i64) {
    if y == 0 {
        (1, 0, x)
    } else {
        let (p, q, g) = gcd_ex(y, x % y);
        (q, p - q * (x / y), g)
    }
}

/// for all i, x = r[i] (mod. m[i]) => x = y (mod. z)
/// (r[i], m[i]) -> Some(y, z)
pub fn crt(rm: Vec<(i64, i64)>) -> Option<(i64, i64)> {
    let mut r0 = 0;
    let mut m0 = 1;
    for &(r, m) in rm.iter() {
        let (p, _, g) = gcd_ex(m0, m);
        if (r - r0) % g != 0 {
            return None;
        }
        let tmp = (r - r0) / g * p % (m / g);
        r0 += m0 * tmp;
        m0 *= m / g;
    }
    if r0 < 0 {
        let x = (-r0 + m0 - 1) / m0;
        r0 += m0 * x;
    }
    Some((r0, m0))
}