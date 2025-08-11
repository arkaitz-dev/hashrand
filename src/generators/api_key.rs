use crate::cli::AlphabetType;
use super::alphabets::get_alphabet;

pub fn generate_api_key_response(length: usize, alphabet_type: Option<AlphabetType>, raw: bool) -> Result<String, Box<dyn std::error::Error>> {
    let alphabet_type = alphabet_type.unwrap_or(AlphabetType::Full);
    
    // Different minimum lengths based on alphabet for same entropy (~262 bits)
    let (min_length, default_length) = match alphabet_type {
        AlphabetType::NoLookAlike => (47, 47),  // 47 chars with 49-char alphabet ≈ 262 bits
        AlphabetType::Full => (44, 44),  // 44 chars with 62-char alphabet ≈ 262 bits
        _ => return Err("API key generation only supports NoLookAlike or Full alphabets".into()),
    };
    
    // Use provided length or default for selected alphabet
    let actual_length = if length == 44 && alphabet_type == AlphabetType::NoLookAlike {
        default_length // Auto-adjust default length for nolookalike
    } else {
        length
    };
    
    // Validate length range for selected alphabet
    if actual_length < min_length || actual_length > 64 {
        return Err(format!("API key length must be between {} and 64 characters for the selected alphabet", min_length).into());
    }
    
    let alphabet = get_alphabet(&alphabet_type);
    let hash = nanoid::format(nanoid::rngs::default, alphabet, actual_length);
    let api_key = format!("ak_{}", hash);
    
    Ok(if raw {
        api_key
    } else {
        format!("{}\n", api_key)
    })
}