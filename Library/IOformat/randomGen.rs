use rand::Rng;

fn rand_vec(n: usize, min: usize, max: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(min..max)).collect()
}

fn rand_vec(n: usize, min: i64, max: i64) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(min..max)).collect()
}
