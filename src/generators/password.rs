use crate::cli::AlphabetType;
use super::alphabets::get_alphabet;

pub fn generate_password_response(length: Option<usize>, alphabet_type: Option<AlphabetType>, raw: bool) -> Result<String, Box<dyn std::error::Error>> {
    let alphabet_type = alphabet_type.unwrap_or(AlphabetType::FullWithSymbols);
    
    // Different minimum lengths based on alphabet for same entropy (~130 bits)
    let (min_length, default_length) = match alphabet_type {
        AlphabetType::NoLookAlike => (24, 24),  // 24 chars with 49-char alphabet ≈ 130 bits
        AlphabetType::FullWithSymbols => (21, 21),  // 21 chars with 73-char alphabet ≈ 130 bits
        _ => return Err("Password generation only supports NoLookAlike or FullWithSymbols alphabets".into()),
    };
    
    let length = length.unwrap_or(default_length);
    
    if length < min_length || length > 44 {
        return Err(format!("Password length must be between {} and 44 characters for the selected alphabet", min_length).into());
    }
    
    let alphabet = get_alphabet(&alphabet_type);
    let password = nanoid::format(nanoid::rngs::default, alphabet, length);
    
    Ok(if raw {
        password
    } else {
        format!("{}\n", password)
    })
}