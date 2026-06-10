pub trait Magic {
    fn hash(&self, key: u64) -> usize;
    fn max_size(&self) -> usize;

    // assumes distinct keys
    fn size_if_valid(&self, keys: &[u64]) -> Option<usize> {
        assert!(keys.len() <= self.max_size());

        let mut used = vec![false; self.max_size()];

        for &key in keys {
            let index = self.hash(key);

            if used[index] {
                return None;
            }
            used[index] = true;
        }

        for i in (0..self.max_size()).rev() {
            if used[i] {
                return Some(i + 1);
            }
        }

        Some(0)
    }
}
