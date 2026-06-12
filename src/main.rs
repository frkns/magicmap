#![allow(unused)]
mod core;
mod search;
use core::modulus::Modulus;
use core::mul_shift::MulShift;
use search::Search;

fn main() -> Result<(), String> {
    let keys: Vec<u64> = core::input::read_keys()?;
    Modulus::run(&keys);
    MulShift::run(&keys);

    Ok(())
}
