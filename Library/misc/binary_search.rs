pub trait BinarySearch<T> {
    fn lower_bound(&self, x: T) -> usize;
    fn upper_bound(&self, x: T) -> usize;
}

impl<T: Ord + Clone + Copy> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: T) -> usize {
        self.binary_search_by_key(&(x, 0), |&y| (y, 1)).unwrap_err()
    }
    fn upper_bound(&self, x: T) -> usize {
        self.binary_search_by_key(&(x, 1), |&y| (y, 0)).unwrap_err()
    }
}
