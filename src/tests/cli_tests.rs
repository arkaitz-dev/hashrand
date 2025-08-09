use crate::cli::*;
use clap::Parser;

#[test]
fn test_parse_length_valid() {
    assert_eq!(parse_length("2").unwrap(), 2);
    assert_eq!(parse_length("64").unwrap(), 64);
    assert_eq!(parse_length("128").unwrap(), 128);
}

#[test]
fn test_parse_length_too_small() {
    assert!(parse_length("0").is_err());
    assert!(parse_length("1").is_err());
}

#[test]
fn test_parse_length_too_large() {
    assert!(parse_length("129").is_err());
    assert!(parse_length("200").is_err());
}

#[test]
fn test_parse_length_invalid_input() {
    assert!(parse_length("abc").is_err());
    assert!(parse_length("").is_err());
    assert!(parse_length("-5").is_err());
}

#[cfg(unix)]
#[test]
fn test_parse_mode_valid() {
    assert_eq!(parse_mode("644").unwrap(), 0o644);
    assert_eq!(parse_mode("755").unwrap(), 0o755);
    assert_eq!(parse_mode("0600").unwrap(), 0o600);
    assert_eq!(parse_mode("777").unwrap(), 0o777);
}

#[cfg(unix)]
#[test]
fn test_parse_mode_invalid() {
    assert!(parse_mode("888").is_err());
    assert!(parse_mode("abc").is_err());
    assert!(parse_mode("").is_err());
}

#[test]
fn test_api_key_default_length() {
    // Test that api-key option uses default 44 characters
    let args_vec = vec!["hashrand", "--api-key"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert!(args.api_key);
    assert!(!args.password);
    assert_eq!(args.length, 21); // Default before modification in main (becomes 44)
    // Validation should pass after length is changed to 44 in main
}

#[test]
fn test_password_default_length() {
    // Test that password option uses 21 characters by default
    let args_vec = vec!["hashrand", "--password"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert!(args.password);
    assert!(!args.api_key);
    assert_eq!(args.length, 21); // Default password length
}

#[test]
fn test_api_key_with_custom_length() {
    // Test that api-key option can have valid custom length (44-64)
    let args_vec = vec!["hashrand", "--api-key", "60"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert!(args.api_key);
    assert_eq!(args.length, 60);
    assert!(args.validate().is_ok());
    
    // Test invalid lengths
    let args_vec = vec!["hashrand", "--api-key", "30"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert!(args.validate().is_err()); // Too short
    
    let args_vec = vec!["hashrand", "--api-key", "65"];  
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert!(args.validate().is_err()); // Too long
}

#[test]
fn test_password_with_custom_length() {
    // Test that password option respects custom length within valid range
    let args_vec = vec!["hashrand", "--password", "24"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert!(args.password);
    assert_eq!(args.length, 24);
}

#[test] 
fn test_password_length_too_short() {
    // Test that password length below 21 is invalid (will be checked in main)
    let args_vec = vec!["hashrand", "--password", "15"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert!(args.password);
    assert_eq!(args.length, 15); // Parse succeeds, validation happens in main
}

#[test]
fn test_password_length_too_long() {
    // Test that password length above 44 is invalid (will be checked in main)
    let args_vec = vec!["hashrand", "--password", "50"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert!(args.password);
    assert_eq!(args.length, 50); // Parse succeeds, validation happens in main
}

#[test]
fn test_password_length_at_boundaries() {
    // Test boundary values for password length
    let test_cases = vec![
        ("21", 21), // minimum
        ("44", 44), // maximum
    ];
    
    for (length_str, expected) in test_cases {
        let args_vec = vec!["hashrand", "--password", length_str];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert!(args.password);
        assert_eq!(args.length, expected);
    }
}

#[test]
fn test_api_key_password_conflict() {
    // Test that api-key and password cannot be used together
    let args_vec = vec!["hashrand", "--api-key", "--password"];
    let result = Args::try_parse_from(&args_vec);
    assert!(result.is_err());
}

#[test]
fn test_api_key_conflicts_with_alphabets() {
    // Test that api-key conflicts with other alphabet options
    let test_cases = vec![
        vec!["hashrand", "--api-key", "--full"],
        vec!["hashrand", "--api-key", "--no-look-alike"],
        vec!["hashrand", "--api-key", "--full-with-symbols"],
    ];
    
    for args_vec in test_cases {
        let result = Args::try_parse_from(&args_vec);
        assert!(result.is_err());
    }
}

#[test]
fn test_api_key_length_boundaries() {
    // Test API key length boundaries (44 and 64 should pass)
    let args_vec = vec!["hashrand", "--api-key", "44"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert!(args.validate().is_ok());
    
    let args_vec = vec!["hashrand", "--api-key", "64"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert!(args.validate().is_ok());
    
    // Test just outside boundaries (43 and 65 should fail)
    let args_vec = vec!["hashrand", "--api-key", "43"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert!(args.validate().is_err());
    
    let args_vec = vec!["hashrand", "--api-key", "65"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert!(args.validate().is_err());
}

#[test]
fn test_password_conflicts_with_alphabets() {
    // Test that password conflicts with other alphabet options
    let test_cases = vec![
        vec!["hashrand", "--password", "--full"],
        vec!["hashrand", "--password", "--no-look-alike"],
        vec!["hashrand", "--password", "--full-with-symbols"],
    ];
    
    for args_vec in test_cases {
        let result = Args::try_parse_from(&args_vec);
        assert!(result.is_err());
    }
}

#[test]
fn test_api_key_conflicts_with_all_options() {
    // Test that api-key conflicts with ALL other options except --raw
    let test_cases = vec![
        vec!["hashrand", "--api-key", "--check"],
        vec!["hashrand", "--api-key", "--mkdir"],
        vec!["hashrand", "--api-key", "--touch"],
        vec!["hashrand", "--api-key", "--prefix", "test"],
        vec!["hashrand", "--api-key", "--suffix", "test"],
        vec!["hashrand", "--api-key", "--path", "/tmp"],
    ];
    
    for args_vec in test_cases {
        let result = Args::try_parse_from(&args_vec);
        assert!(result.is_err());
    }
}

#[test]
fn test_password_conflicts_with_non_length_options() {
    // Test that password conflicts with all options except length and --raw
    let test_cases = vec![
        vec!["hashrand", "--password", "--check"],
        vec!["hashrand", "--password", "--mkdir"],
        vec!["hashrand", "--password", "--touch"],
        vec!["hashrand", "--password", "--prefix", "test"],
        vec!["hashrand", "--password", "--suffix", "test"],
        vec!["hashrand", "--password", "--path", "/tmp"],
    ];
    
    for args_vec in test_cases {
        let result = Args::try_parse_from(&args_vec);
        assert!(result.is_err());
    }
}

#[test]
fn test_raw_with_api_key() {
    // Test that --raw works with --api-key
    let args_vec = vec!["hashrand", "--api-key", "--raw"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert!(args.api_key);
    assert!(args.raw);
}

#[test]
fn test_raw_with_password() {
    // Test that --raw works with --password
    let args_vec = vec!["hashrand", "--password", "--raw"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert!(args.password);
    assert!(args.raw);
}

#[test]
fn test_raw_with_password_and_length() {
    // Test that --raw works with --password and custom length
    let args_vec = vec!["hashrand", "--password", "20", "--raw"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert!(args.password);
    assert!(args.raw);
    assert_eq!(args.length, 20);
}

#[test]
fn test_raw_short_flag() {
    // Test that -r short flag works
    let args_vec = vec!["hashrand", "-r"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert!(args.raw);
}

#[test]
fn test_raw_with_various_options() {
    // Test that --raw works with various alphabet options
    let test_cases = vec![
        vec!["hashrand", "--raw", "--full"],
        vec!["hashrand", "--raw", "--no-look-alike"],
        vec!["hashrand", "--raw", "--full-with-symbols"],
        vec!["hashrand", "-r", "--check"],
    ];
    
    for args_vec in test_cases {
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert!(args.raw);
    }
}

#[test]
fn test_api_key_has_prefix() {
    // Test that API keys have the 'ak_' prefix
    // Note: This test only verifies argument parsing, not the actual prefix logic
    let args_vec = vec!["hashrand", "--api-key"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert!(args.api_key);
    // The actual prefix logic is tested through integration tests
}

#[test]
fn test_serve_option_parsing() {
    // Test that --serve option parses correctly
    let args_vec = vec!["hashrand", "--serve", "8080"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert_eq!(args.serve, Some(8080));
    assert!(!args.listen_all_ips);
    
    // Test short form
    let args_vec = vec!["hashrand", "-s", "3000"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert_eq!(args.serve, Some(3000));
    assert!(!args.listen_all_ips);
    
    // Test with --listen-all-ips flag
    let args_vec = vec!["hashrand", "--serve", "8080", "--listen-all-ips"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert_eq!(args.serve, Some(8080));
    assert!(args.listen_all_ips);
    
    // Test that --listen-all-ips requires --serve
    let args_vec = vec!["hashrand", "--listen-all-ips"];
    let result = Args::try_parse_from(&args_vec);
    assert!(result.is_err());
}

#[test]
fn test_max_param_length_option_parsing() {
    // Test default value
    let args_vec = vec!["hashrand", "--serve", "8080"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert_eq!(args.max_param_length, 32);
    
    // Test custom value
    let args_vec = vec!["hashrand", "--serve", "8080", "--max-param-length", "64"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert_eq!(args.max_param_length, 64);
    
    // Test that --max-param-length requires --serve
    let args_vec = vec!["hashrand", "--max-param-length", "64"];
    let result = Args::try_parse_from(&args_vec);
    assert!(result.is_err());
}

#[test]
fn test_rate_limiting_options() {
    // Test enable-rate-limiting flag
    let args_vec = vec!["hashrand", "--serve", "8080", "--enable-rate-limiting"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert!(args.enable_rate_limiting);
    assert_eq!(args.rate_limit, 100); // Default
    
    // Test custom rate limit
    let args_vec = vec!["hashrand", "--serve", "8080", "--enable-rate-limiting", "--rate-limit", "50"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert!(args.enable_rate_limiting);
    assert_eq!(args.rate_limit, 50);
    
    // Test that --rate-limit requires --enable-rate-limiting
    let args_vec = vec!["hashrand", "--serve", "8080", "--rate-limit", "50"];
    let result = Args::try_parse_from(&args_vec);
    assert!(result.is_err());
}

#[test]
fn test_cors_options() {
    // Test enable-cors flag
    let args_vec = vec!["hashrand", "--serve", "8080", "--enable-cors"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert!(args.enable_cors);
    
    // Test that --enable-cors requires --serve
    let args_vec = vec!["hashrand", "--enable-cors"];
    let result = Args::try_parse_from(&args_vec);
    assert!(result.is_err());
}

#[test]
fn test_body_size_options() {
    // Test default max-body-size
    let args_vec = vec!["hashrand", "--serve", "8080"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert_eq!(args.max_body_size, 1024);
    
    // Test custom max-body-size
    let args_vec = vec!["hashrand", "--serve", "8080", "--max-body-size", "2048"];
    let args = Args::try_parse_from(&args_vec).unwrap();
    assert_eq!(args.max_body_size, 2048);
    
    // Test that --max-body-size requires --serve
    let args_vec = vec!["hashrand", "--max-body-size", "2048"];
    let result = Args::try_parse_from(&args_vec);
    assert!(result.is_err());
}