use super::traits::Magic;

pub struct Modulus {
    modulus: usize,
    max_size: usize,
}

impl Modulus {
    pub fn new(modulus: usize) -> Self {
        Self {
            modulus,
            max_size: modulus as usize,
        }
    }
}

impl Magic for Modulus {
    fn hash(&self, key: u64) -> usize {
        (key % self.modulus as u64) as usize
    }

    fn max_size(&self) -> usize {
        self.max_size
    }
}
