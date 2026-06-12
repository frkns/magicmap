use super::traits::Magic;

#[inline]
fn r#mod(a: u64, b: u64) -> u64 {
    // may change to faster modulo
    a % b
}

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
        r#mod(key, self.modulus as u64) as usize
    }

    fn max_size(&self) -> usize {
        self.max_size
    }
}
