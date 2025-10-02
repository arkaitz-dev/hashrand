/// Convert Base58 username to user_id bytes (DRY utility)
///
/// # Arguments
/// * `username` - Base58 encoded username
///
/// # Returns
/// * `Result<[u8; 16], String>` - 16-byte user_id or error
pub fn username_to_user_id(username: &str) -> Result<[u8; 16], String> {
    let user_id_bytes = bs58::decode(username)
        .into_vec()
        .map_err(|_| "Invalid username format")?;

    if user_id_bytes.len() != 16 {
        return Err("Invalid username length".to_string());
    }

    let mut user_id = [0u8; 16];
    user_id.copy_from_slice(&user_id_bytes);

    Ok(user_id)
}
