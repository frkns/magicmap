use std::collections::HashSet;
use std::io::{self, Read};

pub fn read_keys() -> Result<Vec<u64>, String> {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .map_err(|e| format!("failed to read stdin: {e}"))?;

    let mut seen = HashSet::<u64>::new();

    for (i, line) in input.lines().enumerate() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let line_no = i + 1;

        let key = line
            .parse::<u64>()
            .map_err(|e| format!("line {line_no}, {line}: {e}"))?;

        if !seen.insert(key) {
            return Err(format!("line {line_no}, {line}: duplicate key"));
        }
    }

    Ok(seen.into_iter().collect())
}
