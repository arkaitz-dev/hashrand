// Submodules
mod argon2;
mod chacha;
mod user_id;

// Re-export public API from submodules

// ChaCha20 encryption/decryption
pub use chacha::{decrypt_magic_link, encrypt_magic_link, generate_chacha_nonce_and_key};

// User ID derivation and username encoding
pub use user_id::{
    derive_user_id, derive_user_id_with_context, email_to_username, user_id_to_username,
};
