use crate::core;
use core::color;
use core::magic::Magic;
use core::modulus::Modulus;
use core::mul_shift::MulShift;
use core::rng;
use std::time::{SystemTime, UNIX_EPOCH};

fn make_rng() -> rng::SplitMix64 {
    rng::SplitMix64::new(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos() as u64,
    )
}

fn ceil_log2(x: usize) -> usize {
    usize::BITS as usize - (x - 1).leading_zeros() as usize
}

pub trait Search: Magic {
    fn run(keys: &[u64]) -> Result<(), String>;
}

impl Search for Modulus {
    fn run(keys: &[u64]) -> Result<(), String> {
        let n = keys.len();
        let max_key = *keys.iter().max().unwrap() as usize;

        // probe to beyond the birthday frontier (n^2), where valid moduli are dense.
        // scratch grows to the largest probed modulus, so this also caps memory.
        let limit = n
            .saturating_mul(n)
            .saturating_mul(3)
            .max(n.saturating_mul(30_000))
            .min(max_key.saturating_add(1))
            .min(1 << 31);

        let span = limit - n + 1;
        let bits = (usize::BITS - (span - 1).leading_zeros()).max(1);

        println!("> recieved {n} keys");
        println!("+ sweeping modulus over [{n}, {limit}]");

        let mut scratch_buf = vec![0usize; n];
        let mut epoch = 1usize;
        let mut tested = 0usize;
        let (mut best_size, mut best_modulo) = (usize::MAX, 0usize);

        // bit-reversed order
        let mut state = 0usize;
        loop {
            let modulo = n + state;
            tested += 1;

            if scratch_buf.len() < modulo {
                scratch_buf.resize((scratch_buf.len() * 2).max(modulo), 0);
            }

            if let Some(size) = Modulus::new(modulo).size_if_valid(
                keys,
                &mut scratch_buf,
                &mut epoch,
                Some(best_size),
            ) {
                (best_size, best_modulo) = (size, modulo);
                let perfect = size == n;
                println!(
                    "| found:   {}size={size}  `key % {modulo}`{}    searched {tested}",
                    if perfect { color::CYAN } else { color::GREEN },
                    color::RESET,
                );
                if perfect {
                    break;
                }
            }

            loop {
                let mut bit = 1 << (bits - 1);
                while state & bit != 0 {
                    state ^= bit;
                    bit >>= 1;
                }
                state |= bit;
                if state == 0 {
                    if best_modulo == 0 {
                        println!("- exhausted [{n}, {limit}], nothing found (try mul-shift)\n");
                    } else {
                        println!("- exhausted [{n}, {limit}], best is optimal\n");
                    }
                    return Ok(());
                }
                if state < span {
                    break;
                }
            }
        }

        Ok(())
    }
}

impl Search for MulShift {
    fn run(keys: &[u64]) -> Result<(), String> {
        let n = keys.len();

        let approx_start_size = n.max(
            n.checked_mul(n)
                .ok_or_else(|| format!("length {n}^2 overflows usize"))?
                / 10,
        );

        let min_bits = ceil_log2(n);
        let max_bits = ceil_log2(approx_start_size);

        println!("> recieved {} keys", n);
        println!(
            "+ starting search on size <= {} (estimated feasible upper bound)",
            1usize << max_bits
        );

        let mut rng = make_rng();
        let mut scratch_buf = vec![0usize; 1usize << max_bits];
        let mut epoch = 1usize;
        let mut attempts = 0usize;

        let mut best_bits = max_bits;
        let mut best_size = usize::MAX;
        let mut best_mul = 0u64;

        loop {
            attempts += 1;
            let mul = rng.next_odd();

            let mut bits = best_bits;
            let mut improved = false;

            while let Some(s) =
                MulShift::new(mul, bits).size_if_valid(&keys, &mut scratch_buf, &mut epoch, None)
            {
                if s < best_size {
                    best_size = s;
                    best_bits = bits;
                    best_mul = mul;
                    improved = true;
                }
                if bits == min_bits {
                    break;
                }
                bits -= 1;
            }

            if !improved {
                continue;
            }

            let perfect = best_size == n;
            println!(
                "| found:   {}bits={best_bits}  size={best_size}  `(key * {best_mul:#018x}) >> {}`{}    searched {attempts}",
                if perfect { color::CYAN } else { color::GREEN },
                64 - best_bits,
                color::RESET,
            );
            if perfect {
                break;
            }
        }

        Ok(())
    }
}
