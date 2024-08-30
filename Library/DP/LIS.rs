pub fn LIS<X: Copy + Ord + std::fmt::Debug>(x: &Vec<X>) -> usize {
    let mut dp = vec![];
    for &a in x {
        let k = dp.binary_search_by_key(&(a, 0), |&d| (d, 1)).unwrap_err();
        if k == dp.len() {
            dp.push(a);
        } else {
            dp[k] = a;
        }
        println!("{:?}", dp);
    }
    dp.len()
}