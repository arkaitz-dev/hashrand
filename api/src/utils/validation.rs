/// Input validation utilities for API endpoints
use anyhow::Result;

/// Validate email format more strictly than basic checks
pub fn validate_email(email: &str) -> Result<()> {
    // Basic checks
    if email.is_empty() {
        return Err(anyhow::anyhow!("Email cannot be empty"));
    }

    if email.len() > 254 {
        return Err(anyhow::anyhow!("Email too long (max 254 characters)"));
    }

    // Must contain exactly one @ symbol
    let at_count = email.matches('@').count();
    if at_count != 1 {
        return Err(anyhow::anyhow!("Email must contain exactly one @ symbol"));
    }

    let parts: Vec<&str> = email.split('@').collect();
    let (local, domain) = (parts[0], parts[1]);

    // Local part validation
    if local.is_empty() || local.len() > 64 {
        return Err(anyhow::anyhow!("Email local part invalid length"));
    }

    // Domain part validation
    if domain.is_empty() || domain.len() > 253 {
        return Err(anyhow::anyhow!("Email domain invalid length"));
    }

    // Domain must contain at least one dot
    if !domain.contains('.') {
        return Err(anyhow::anyhow!(
            "Email domain must contain at least one dot"
        ));
    }

    // Check for dangerous characters that could be used for XSS
    let dangerous_chars = ['<', '>', '"', '\'', '&', '\n', '\r', '\t'];
    if email.chars().any(|c| dangerous_chars.contains(&c)) {
        return Err(anyhow::anyhow!("Email contains invalid characters"));
    }

    Ok(())
}

/// Validate length parameter with strict bounds
pub fn validate_length(length: usize, min: usize, max: usize) -> Result<()> {
    if length < min || length > max {
        return Err(anyhow::anyhow!(
            "Length must be between {} and {}",
            min,
            max
        ));
    }
    Ok(())
}

/// Validate prefix/suffix with 4-byte limit and safe characters  
pub fn validate_prefix_suffix(text: &str, field_name: &str) -> Result<()> {
    if text.len() > 4 {
        return Err(anyhow::anyhow!("{} must be 4 bytes or less", field_name));
    }

    // Check for control characters or dangerous sequences
    if text.chars().any(|c| c.is_control()) {
        return Err(anyhow::anyhow!(
            "{} contains control characters",
            field_name
        ));
    }

    // Check for common injection patterns
    let dangerous_patterns = ["--", "__", "/*", "*/", "<", ">"];
    for pattern in &dangerous_patterns {
        if text.contains(pattern) {
            return Err(anyhow::anyhow!(
                "{} contains dangerous pattern: {}",
                field_name,
                pattern
            ));
        }
    }

    Ok(())
}

/// Validate seed string format (Base58)
pub fn validate_seed_string(seed_str: &str) -> Result<()> {
    if seed_str.is_empty() {
        return Err(anyhow::anyhow!("Seed cannot be empty"));
    }

    // Base58 has specific character set
    const BASE58_CHARS: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

    for c in seed_str.chars() {
        if !BASE58_CHARS.contains(c) {
            return Err(anyhow::anyhow!(
                "Seed contains invalid Base58 character: {}",
                c
            ));
        }
    }

    // Reasonable length limits for Base58 seed
    if seed_str.len() < 32 || seed_str.len() > 64 {
        return Err(anyhow::anyhow!(
            "Seed length invalid (expected 32-64 characters)"
        ));
    }

    Ok(())
}
