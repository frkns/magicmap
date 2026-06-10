use super::traits::Magic;

pub struct MulShift {
    mul: u64,
    bits: usize,
    shift: usize,
    max_size: usize,
}

impl MulShift {
    pub fn new(mul: u64, bits: usize) -> Self {
        Self {
            mul,
            bits,
            shift: 64 - bits,
            max_size: 1 << bits,
        }
    }
}

impl Magic for MulShift {
    fn hash(&self, key: u64) -> usize {
        (key.wrapping_mul(self.mul) >> self.shift) as usize
    }

    fn max_size(&self) -> usize {
        self.max_size
    }
}

