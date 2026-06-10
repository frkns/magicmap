use crate::core;

fn test() -> Result<(), String> {
    let keys: Vec<u64> = core::input::read_keys()?;
    Ok(())
}
