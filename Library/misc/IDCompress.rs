pub fn IDCompress<T: Clone + Ord>(xs: &Vec<T>) -> Vec<usize> {
    let xset: std::collections::BTreeSet<&T> = xs.iter().collect();
    let xmap: std::collections::BTreeMap<&T, usize> =
        xset.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    xs.iter().map(|x| xmap[x]).collect()
}

pub fn IDCompress<T: Clone + Ord>(xs: &Vec<T>) -> std::collections::BTreeMap<&T, usize> {
    let xset: std::collections::BTreeSet<&T> = xs.iter().collect();
    let xmap: std::collections::BTreeMap<&T, usize> =
        xset.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    xmap
}
