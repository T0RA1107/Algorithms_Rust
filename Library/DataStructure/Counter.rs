#[derive(Debug, Clone)]
pub struct Counter<K>(std::collections::BTreeMap<K, usize>);

impl<K> Counter<K>
where
    K: Ord + Copy
{
    pub fn new() -> Self {
        Counter(std::collections::BTreeMap::<K, usize>::new())
    }
    pub fn increment(&mut self, item: K) {
        *self.0.entry(item).or_insert(0) += 1;
    }
    pub fn decrement(&mut self, item: K) {
        assert!(self.0.contains_key(&item), "Key cannot be found");
        *self.0.entry(item).or_insert(0) -= 1;
        if self.0[&item] == 0 {
            self.0.remove(&item);
        }
    }
    pub fn count(&mut self, item: K) -> usize {
        match self.0.get(&item) {
            Some(&x) => x,
            None => 0,
        }
    }
    pub fn keys(&self) -> std::collections::btree_map::Keys<'_, K, usize> {
        self.0.keys()
    }
    pub fn values(&self) -> std::collections::btree_map::Values<'_, K, usize> {
        self.0.values()
    }
    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, K, usize> {
        self.0.iter()
    }
}
