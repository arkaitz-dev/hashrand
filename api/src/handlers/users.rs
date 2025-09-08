//! User management API endpoints
//!
//! Provides REST API endpoints for user CRUD operations with automatic
//! database environment detection based on request host.

use serde_json;
use spin_sdk::http::{Method, Request, Response};
use std::collections::HashMap;

use crate::database::{User, UserOperations, initialize_database};

/// Handle user-related requests
///
/// Routes:
/// - GET /api/users - List all users
/// - GET /api/users/:id - Get specific user
/// - POST /api/users - Create new user
/// - DELETE /api/users/:id - Delete user
///
/// # Arguments
/// * `req` - HTTP request
/// * `path` - Request path
/// * `query_params` - Query parameters
///
/// # Returns
/// * `Result<Response, anyhow::Error>` - HTTP response or error
pub fn handle_users(
    req: Request,
    path: &str,
    query_params: HashMap<String, String>,
) -> anyhow::Result<Response> {
    // Initialize database tables if needed
    if let Err(e) = initialize_database() {
        eprintln!("Database initialization error: {:?}", e);
        return Ok(Response::new(
            500,
            r#"{"error": "Database initialization failed"}"#,
        ));
    }

    let method = req.method();
    let path_parts: Vec<&str> = path.split('/').filter(|&s| !s.is_empty()).collect();

    match (method, path_parts.as_slice()) {
        // GET /api/users - List all users
        (Method::Get, ["api", "users"]) => handle_list_users(query_params),

        // GET /api/users/:id - Get specific user
        (Method::Get, ["api", "users", id_str]) => handle_get_user(id_str),

        // POST /api/users - Create new user
        (Method::Post, ["api", "users"]) => handle_create_user(req),

        // DELETE /api/users/:id - Delete user
        (Method::Delete, ["api", "users", id_str]) => handle_delete_user(id_str),

        // Unsupported routes
        _ => Ok(Response::new(
            404,
            r#"{"error": "User endpoint not found"}"#,
        )),
    }
}

/// Handle GET /api/users - List users
fn handle_list_users(query_params: HashMap<String, String>) -> anyhow::Result<Response> {
    // Parse optional limit parameter
    let limit = query_params
        .get("limit")
        .and_then(|s| s.parse::<u32>().ok());

    match UserOperations::list_users(limit) {
        Ok(users) => {
            let response = serde_json::json!({
                "users": users,
                "count": users.len()
            });

            Ok(Response::new(200, response.to_string()))
        }
        Err(e) => {
            eprintln!("Database error listing users: {:?}", e);
            Ok(Response::new(500, r#"{"error": "Failed to list users"}"#))
        }
    }
}

/// Handle GET /api/users/:id - Get specific user
fn handle_get_user(id_str: &str) -> anyhow::Result<Response> {
    let user_id = match id_str.parse::<i64>() {
        Ok(id) => id,
        Err(_) => {
            return Ok(Response::new(400, r#"{"error": "Invalid user ID format"}"#));
        }
    };

    match UserOperations::get_user_by_id(user_id) {
        Ok(Some(user)) => Ok(Response::new(200, serde_json::to_string(&user)?)),
        Ok(None) => Ok(Response::new(404, r#"{"error": "User not found"}"#)),
        Err(e) => {
            eprintln!("Database error getting user: {:?}", e);
            Ok(Response::new(500, r#"{"error": "Failed to get user"}"#))
        }
    }
}

/// Handle POST /api/users - Create new user
fn handle_create_user(req: Request) -> anyhow::Result<Response> {
    // Parse request body
    let body_bytes = req.body();
    let body_str = std::str::from_utf8(body_bytes)?;

    // Deserialize user data
    let user_data: Result<User, _> = serde_json::from_str(body_str);
    let user = match user_data {
        Ok(u) => u,
        Err(_) => {
            return Ok(Response::new(400, r#"{"error": "Invalid JSON format"}"#));
        }
    };

    // Validate required fields
    if user.username.is_empty() || user.email.is_empty() {
        return Ok(Response::new(
            400,
            r#"{"error": "Username and email are required"}"#,
        ));
    }

    // Create user in database
    match UserOperations::create_user(&user) {
        Ok(user_id) => {
            // Return created user
            match UserOperations::get_user_by_id(user_id) {
                Ok(Some(created_user)) => {
                    Ok(Response::new(201, serde_json::to_string(&created_user)?))
                }
                _ => {
                    let response = serde_json::json!({"id": user_id, "message": "User created"});
                    Ok(Response::new(201, response.to_string()))
                }
            }
        }
        Err(e) => {
            eprintln!("Database error creating user: {:?}", e);

            // Handle unique constraint violations
            let error_msg = if e.to_string().contains("UNIQUE constraint failed") {
                r#"{"error": "Username or email already exists"}"#
            } else {
                r#"{"error": "Failed to create user"}"#
            };

            Ok(Response::new(400, error_msg))
        }
    }
}

/// Handle DELETE /api/users/:id - Delete user
fn handle_delete_user(id_str: &str) -> anyhow::Result<Response> {
    let user_id = match id_str.parse::<i64>() {
        Ok(id) => id,
        Err(_) => {
            return Ok(Response::new(400, r#"{"error": "Invalid user ID format"}"#));
        }
    };

    match UserOperations::delete_user(user_id) {
        Ok(true) => Ok(Response::new(
            200,
            r#"{"message": "User deleted successfully"}"#,
        )),
        Ok(false) => Ok(Response::new(404, r#"{"error": "User not found"}"#)),
        Err(e) => {
            eprintln!("Database error deleting user: {:?}", e);
            Ok(Response::new(500, r#"{"error": "Failed to delete user"}"#))
        }
    }
}
