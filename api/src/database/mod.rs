/// Database module for HashRand Spin application
///
/// This module handles SQLite database operations for magic link authentication.
pub mod connection;
pub mod operations;

pub use connection::{get_database_connection, initialize_database};
