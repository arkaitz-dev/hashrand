use crate::server::{validate_query_params, check_rate_limit};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::sync::RwLock;

#[test]
fn test_validate_query_params_valid() {
    // Test valid parameters within limits
    let prefix = Some("test".to_string());
    let suffix = Some("end".to_string());
    assert!(validate_query_params(&prefix, &suffix, 10).is_ok());
    
    // Test with None values
    assert!(validate_query_params(&None, &None, 10).is_ok());
    
    // Test with only prefix
    assert!(validate_query_params(&prefix, &None, 10).is_ok());
    
    // Test with only suffix
    assert!(validate_query_params(&None, &suffix, 10).is_ok());
}

#[test]
fn test_validate_query_params_prefix_too_long() {
    let prefix = Some("this_prefix_is_too_long_for_limit".to_string());
    let suffix = Some("ok".to_string());
    let result = validate_query_params(&prefix, &suffix, 10);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Prefix length"));
}

#[test]
fn test_validate_query_params_suffix_too_long() {
    let prefix = Some("ok".to_string());
    let suffix = Some("this_suffix_is_too_long_for_limit".to_string());
    let result = validate_query_params(&prefix, &suffix, 10);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Suffix length"));
}

#[test]
fn test_validate_query_params_both_too_long() {
    let prefix = Some("very_long_prefix".to_string());
    let suffix = Some("very_long_suffix".to_string());
    let result = validate_query_params(&prefix, &suffix, 5);
    assert!(result.is_err());
    // Should fail on prefix first
    assert!(result.unwrap_err().contains("Prefix length"));
}

#[tokio::test]
async fn test_rate_limiter() {
    let rate_limiter = Arc::new(RwLock::new(HashMap::new()));
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    
    // First request should succeed
    assert!(check_rate_limit(&rate_limiter, addr, 2).await);
    
    // Second request should succeed
    assert!(check_rate_limit(&rate_limiter, addr, 2).await);
    
    // Third request should fail (exceeded limit of 2 per second)
    assert!(!check_rate_limit(&rate_limiter, addr, 2).await);
}