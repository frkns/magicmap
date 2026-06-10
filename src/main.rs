#![allow(unused)]
mod core;
mod search;

fn main() -> Result<(), String> {
    let keys: Vec<u64> = core::input::read_keys()?;
    dbg!(keys);

    Ok(())
}
