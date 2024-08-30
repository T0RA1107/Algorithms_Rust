fn RLE<T: Clone + Copy + Eq>(vector: &[T]) -> Vec<(T, usize)> {
    let mut ret = vec![(vector[0], 0)];
    for &v in vector {
        let (x, cnt) = ret.pop().unwrap();
        if x == v {
            ret.push((x, cnt + 1));
        } else {
            ret.push((x, cnt));
            ret.push((v, 1));
        }
    }
    ret
}
