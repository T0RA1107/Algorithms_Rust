fn Z_algorithm(S: Vec<char>) -> Vec<usize> {
    let N = S.len();
    let mut Z = vec![0; N];
    let mut c = 0;
    for i in 1..N {
        let l = i - c;
        if i + Z[l] < c + Z[c] {
            Z[i] = Z[l];
        } else {
            let mut j = {
                if c + Z[c] > i {
                    c + Z[c] - i
                } else {
                    0
                }
            };
            while i + j < N && S[i] == S[i + j] {
                j += 1;
            }
            Z[i] = j;
            c = i;
        }
    }
    Z[0] = N;
    Z
}
