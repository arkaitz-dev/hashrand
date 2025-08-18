pub mod generate;
pub mod password;
pub mod api_key;
pub mod version;

pub use generate::handle_generate;
pub use password::handle_password;
pub use api_key::handle_api_key;
pub use version::handle_version;