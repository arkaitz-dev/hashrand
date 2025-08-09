pub fn parse_length(s: &str) -> Result<usize, String> {
    let length: usize = s.parse().map_err(|_| "Length must be a number")?;

    if !(2..=128).contains(&length) {
        return Err("Length must be between 2 and 128".to_string());
    }

    Ok(length)
}

#[cfg(unix)]
pub fn parse_mode(s: &str) -> Result<u32, String> {
    // Parse octal string (e.g., "644" or "0644")
    let s = s.trim_start_matches('0');
    u32::from_str_radix(s, 8)
        .map_err(|_| format!("Invalid mode '{}'. Use octal format like 644 or 755", s))
}