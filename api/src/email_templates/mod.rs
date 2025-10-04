pub mod magic_link;
pub mod shared_secret;

pub use magic_link::render_magic_link_email;
// Note: Shared secret email functions prepared for future use when email sending is implemented
// pub use shared_secret::{render_shared_secret_receiver_email, render_shared_secret_sender_email};
