use crate::cli::AlphabetType;
use super::alphabets::get_alphabet;

pub fn generate_password_response(length: Option<usize>, raw: bool) -> Result<String, Box<dyn std::error::Error>> {
    let length = length.unwrap_or(21);
    
    if length < 21 || length > 44 {
        return Err("Password length must be between 21 and 44 characters".into());
    }
    
    let alphabet = get_alphabet(&AlphabetType::FullWithSymbols);
    let password = nanoid::format(nanoid::rngs::default, alphabet, length);
    
    Ok(if raw {
        password
    } else {
        format!("{}\n", password)
    })
}