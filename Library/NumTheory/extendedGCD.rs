pub fn gcd_ex(x: usize, y: usize) -> (usize, usize, usize) {
    if y == 0 {
        (1, 0, x)
    } else {
        let (p, q, g) = gcd_ex(y, x % y);
        (q, p - q * (x / y), g)
    }
}
