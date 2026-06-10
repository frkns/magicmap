pub trait Magic {
    fn hash(&self, key: u64) -> usize;
    fn max_size(&self) -> usize;

    // assumes distinct keys

    fn size_if_valid<const LESS_THAN: usize>(
        &self,
        keys: &[u64],
        scratch_buf: &mut [usize],
        epoch: &mut usize,
    ) -> Option<usize> {

        assert!(keys.len() <= self.max_size());

        let mut used = vec![false; self.max_size()];
        let mut max_index = 0;

        for &key in keys {
            let index = self.hash(key);

            if used[index] {
                return None;
            }
            max_index = max_index.max(index);
            used[index] = true;
        }

        Some(max_index)
    }
}
