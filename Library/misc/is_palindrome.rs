fn is_palindrome<T: Clone + Eq>(S: &[T]) -> bool {
    for i in 0..S.len() {
        if S[i] != S[S.len() - i - 1] {
            return false;
        }
    }
    true
}
