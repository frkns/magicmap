use std::collections::HashSet;
use std::io::{self, Read};

fn parse_key(s: &str) -> Result<u64, String> {
    if let Ok(x) = s.parse::<u64>() {
        return Ok(x);
    }

    let x = s
        .parse::<i64>()
        .map_err(|e| format!("invalid u64 or i64 integer: {e}"))?;

    Ok(x as u64)
}

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

        let key = parse_key(line)?;

        if !seen.insert(key) {
            return Err(format!("line {line_no}, {line}: duplicate key"));
        }
    }

    Ok(seen.into_iter().collect())
}
