#[derive(Debug, Clone)]
pub struct MultiSet<K>(std::collections::BTreeMap<K, usize>);

impl<K> MultiSet<K>
where 
    K: Ord + Copy
{
    pub fn new() -> Self {
        MultiSet(std::collections::BTreeMap::<K, usize>::new())
    }
    pub fn add(&mut self, item: K) {
        *self.0.entry(item).or_insert(0) += 1;
    }
    pub fn remove(&mut self, item: K) {
        *self.0.entry(item).or_insert(0) -= 1;
        if self.0[&item] == 0 {
            self.0.remove(&item);
        }
    }
    pub fn remove_all(&mut self, item: K) {
        self.0.remove(&item);
    }
    pub fn keys(&self) -> std::collections::btree_map::Keys<'_, K, usize> {
        self.0.keys()
    }
    pub fn min(&self) -> Option<&K> {
        self.keys().next()
    }
    pub fn max(&self) -> Option<&K> {
        self.keys().last()
    }
    pub fn lower_bound(&self, min: K) -> Option<(&K, &usize)> {
        self.0.range(min..).next()
    }
}