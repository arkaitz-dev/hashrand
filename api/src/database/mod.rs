/// Database module for HashRand Spin application
/// 
/// This module handles SQLite database operations with environment-aware
/// database selection (development vs production) based on request host.

pub mod connection;
pub mod models;
pub mod operations;

pub use connection::{get_database_connection, initialize_database, DatabaseEnvironment};
pub use models::User;
pub use operations::UserOperations;