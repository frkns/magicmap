#![allow(unused)]
mod input;
mod rng;

fn main() -> Result<(), String> {
    let keys: Vec<u64> = input::read_keys()?;
    dbg!(keys);

    Ok(())
}
