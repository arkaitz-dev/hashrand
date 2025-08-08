use crate::cli::HashRequest;
use super::alphabets::get_alphabet;

pub fn generate_hash_from_request(request: &HashRequest) -> Result<String, Box<dyn std::error::Error>> {
    let alphabet = get_alphabet(&request.alphabet);
    
    // Generate the random hash
    let hash = nanoid::format(nanoid::rngs::default, alphabet, request.length);
    
    // Build the full name with optional prefix and suffix
    let full_name = format!(
        "{}{}{}",
        request.prefix.as_deref().unwrap_or(""),
        hash,
        request.suffix.as_deref().unwrap_or("")
    );
    
    // Format the output
    Ok(if request.raw {
        full_name
    } else {
        format!("{}\n", full_name)
    })
}