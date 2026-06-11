pub trait Magic {
    fn hash(&self, key: u64) -> usize;
    fn max_size(&self) -> usize;

    // assumes distinct keys

    fn size_if_valid(
        &self,
        keys: &[u64],
        scratch_buf: &mut [usize],
        epoch: &mut usize,
        less_than: Option<usize>,
    ) -> Option<usize> {
        assert!(keys.len() <= self.max_size());
        assert!(scratch_buf.len() >= self.max_size());
        let less_than = less_than.unwrap_or(usize::MAX);

        *epoch += 1;
        let mark = *epoch;

        let mut max_index = 0;

        for &key in keys {
            let index = self.hash(key);

            if max_index >= less_than {
                return None;
            }
            if scratch_buf[index] == mark {
                return None;
            }
            max_index = max_index.max(index);
            scratch_buf[index] = mark;
        }

        Some(max_index + 1)
    }
}
