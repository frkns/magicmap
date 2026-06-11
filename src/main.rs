#![allow(unused)]
mod core;
mod search;

fn main() -> Result<(), String> {
    search::run()?;
    Ok(())
}
