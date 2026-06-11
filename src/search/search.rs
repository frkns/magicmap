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

    println!("number of keys: {}", n);

    let mut rng = rng::SplitMix64::new(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos() as u64,
    );
    let mut scratch_buf = vec![0usize; 1usize << max_bits];
    let mut epoch = 1usize;

    let mut best_size = usize::MAX;
    let mut best_cand = Option::<MulShift>::None;

    for bits in (min_bits..=max_bits).rev() {
        let last_level = bits == min_bits;
        let level_size = 1usize << bits;

        println!(
            "- search   size <= 2^{bits} = {level_size}{}",
            if last_level { "    last level" } else { "" }
        );

        loop {
            let cand = MulShift::new(rng.next_odd(), bits);

            if let Some(size) =
                cand.size_if_valid(&keys, &mut scratch_buf, &mut epoch, Some(best_size))
            {
                if size >= best_size {
                    continue;
                }

                best_size = size;
                let perfect = size == n;

                println!(
                    "| found:   {}size={size}   `index = (key * {:#018x}) >> {}`{}    searched {}",
                    if perfect { color::CYAN } else { color::GREEN },
                    cand.mul(),
                    cand.shift(),
                    color::RESET,
                    epoch - 1,
                );

                best_cand = Some(cand);

                if !last_level {
                    break;
                }
                if perfect {
                    println!("|          {}^^^^{}", color::CYAN, color::RESET);
                    println!(
                        "*          {}perfect hash found.{}",
                        color::CYAN,
                        color::RESET
                    );
                    break;
                }
            }
        }
    }

    Ok(())
}
