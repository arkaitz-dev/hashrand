use crate::cli::AlphabetType;
use super::alphabets::get_alphabet;

pub fn generate_api_key_response(length: usize, raw: bool) -> Result<String, Box<dyn std::error::Error>> {
    // Validate length is between 44 and 64 characters
    if length < 44 || length > 64 {
        return Err("API key length must be between 44 and 64 characters".into());
    }
    
    let alphabet = get_alphabet(&AlphabetType::Full);
    let hash = nanoid::format(nanoid::rngs::default, alphabet, length);
    let api_key = format!("ak_{}", hash);
    
    Ok(if raw {
        api_key
    } else {
        format!("{}\n", api_key)
    })
}