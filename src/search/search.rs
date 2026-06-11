use crate::core;
use core::color;
use core::modulus::Modulus;
use core::mul_shift::MulShift;
use core::rng;
use core::traits::Magic;

const MAX_BITS: usize = 22;

fn ceil_log2(x: usize) -> usize {
    usize::BITS as usize - (x - 1).leading_zeros() as usize
}

pub fn run() -> Result<(), String> {
    let keys: Vec<u64> = core::input::read_keys()?;
    let n = keys.len();
    assert!(
        n <= (1usize << MAX_BITS),
        "number of keys exceeds hardcoded limit of 2^{MAX_BITS}"
    );
    println!("number of keys: {}", n);

    // let mut
    let min_bits = ceil_log2(n);

    let mut rng = rng::SplitMix64::new(0u64);
    let mut scratch_buf = vec![0usize; 1usize << MAX_BITS];
    let mut epoch = 1usize;

    let mut best_size = 1usize << MAX_BITS;
    let mut best_cand = Option::<MulShift>::None;

    for bits in (min_bits..MAX_BITS).rev() {
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
                    "| {}found:   size={size}   `index = (key * {}) >> {}`{}{}",
                    color::GREEN,
                    cand.mul(),
                    cand.shift(),
                    color::RESET,
                    if perfect { "    perfect!" } else { "" },
                );

                best_cand = Some(cand);

                if !last_level {
                    break;
                }
                if perfect {
                    println!("* Found a perfect hash! Stopping now.");
                    break;
                }
            }
        }
    }

    Ok(())
}
