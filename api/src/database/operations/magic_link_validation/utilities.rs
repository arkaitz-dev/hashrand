use super::super::magic_link_types::ValidationResult;

/// Copy bytes to fixed-size array (DRY utility)
pub fn copy_to_array<const N: usize>(dest: &mut [u8; N], src: &[u8]) {
    dest.copy_from_slice(src);
}

/// Extract UTF-8 string from byte slice (DRY utility)
///
/// # Arguments
/// * `bytes` - Byte slice to extract string from
/// * `field_name` - Name of the field for debugging
///
/// # Returns
/// * `Result<String, ValidationResult>` - Extracted string or validation error
pub fn extract_utf8_string(bytes: &[u8], field_name: &str) -> Result<String, ValidationResult> {
    match std::str::from_utf8(bytes) {
        Ok(s) => {
            println!("🔍 DEBUG EXTRACT: Extracted {}: '{}'", field_name, s);
            Ok(s.to_string())
        }
        Err(_) => {
            println!("❌ Database: Invalid UTF-8 in {} bytes", field_name);
            Err(create_validation_error())
        }
    }
}

/// Create validation error tuple (DRY utility)
pub fn create_validation_error() -> ValidationResult {
    (false, None, None, None, None)
}
