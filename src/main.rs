#![allow(unused)]
mod core;
mod search;
use core::color;

fn main() -> Result<(), String> {
    search::run()?;
    Ok(())
}
