use crate::cli::AlphabetType;
use super::alphabets::get_alphabet;

pub fn generate_api_key_response(raw: bool) -> Result<String, Box<dyn std::error::Error>> {
    let alphabet = get_alphabet(&AlphabetType::Full);
    let hash = nanoid::format(nanoid::rngs::default, alphabet, 44);
    let api_key = format!("ak_{}", hash);
    
    Ok(if raw {
        api_key
    } else {
        format!("{}\n", api_key)
    })
}