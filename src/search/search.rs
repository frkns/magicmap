use crate::core;
use core::color;
use core::modulus::Modulus;
use core::mul_shift::MulShift;
use core::rng;
use core::traits::Magic;
use std::time::{SystemTime, UNIX_EPOCH};

fn ceil_log2(x: usize) -> usize {
    usize::BITS as usize - (x - 1).leading_zeros() as usize
}

pub fn run() -> Result<(), String> {
    let keys: Vec<u64> = core::input::read_keys()?;
    let n = keys.len();

    let approx_start_size = n.max(
        n.checked_mul(n)
            .ok_or_else(|| format!("length {n}^2 overflows usize"))?
            / 10,
    );

    let min_bits = ceil_log2(n);
    let max_bits = ceil_log2(approx_start_size);

    println!("- recieved {} keys", n);
    println!(
        "+ starting search on size <= {} (estimated feasible upper bound)",
        1usize << max_bits
    );

    let mut rng = rng::SplitMix64::new(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos() as u64,
    );
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
