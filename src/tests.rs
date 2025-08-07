#[cfg(test)]
pub mod tests {
    use crate::cli::*;
    use crate::server::*;
    use crate::{check_name_exists, generate_unique_name};
    use clap::Parser;
    use std::collections::HashMap;
    use std::fs;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;
    use tempfile::tempdir;
    use tokio::sync::RwLock;

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

    #[test]
    fn test_check_name_exists_no_match() {
        let dir = tempdir().unwrap();
        let name = "test123";
        assert!(!check_name_exists(name, dir.path(), None));
    }

    #[test]
    fn test_check_name_exists_exact_match() {
        let dir = tempdir().unwrap();
        let name = "test123";
        let file_path = dir.path().join(name);
        fs::write(&file_path, "content").unwrap();
        assert!(check_name_exists(name, dir.path(), None));
    }

    #[test]
    fn test_check_name_exists_in_subdirectory() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        let name = "test123";
        let file_path = subdir.join(name);
        fs::write(&file_path, "content").unwrap();
        assert!(check_name_exists(name, dir.path(), None));
    }

    #[test]
    fn test_check_name_exists_with_depth_limit() {
        let dir = tempdir().unwrap();
        
        // Create nested directories beyond the depth limit
        let mut current_path = dir.path().to_path_buf();
        for i in 0..5 {
            current_path = current_path.join(format!("level{}", i));
            fs::create_dir(&current_path).unwrap();
        }
        
        // Create a file at depth 5
        let name = "deep_file";
        fs::write(current_path.join(name), "content").unwrap();
        
        // Should not find file with depth limit of 2
        assert!(!check_name_exists(name, dir.path(), Some(2)));
        
        // Should find file with no depth limit
        assert!(check_name_exists(name, dir.path(), None));
    }

    #[test]
    fn test_generate_unique_name_no_prefix_suffix() {
        let dir = tempdir().unwrap();
        let alphabet: Vec<char> = vec!['a', 'b'];
        let length = 3;

        // Create files with all possible 3-character combinations except one
        let combinations = ["aaa", "aab", "aba", "abb", "baa", "bab", "bba"];
        for combo in &combinations {
            fs::write(dir.path().join(combo), "content").unwrap();
        }

        // The only remaining combination is "bbb"
        let unique_hash = generate_unique_name(&alphabet, length, None, None, dir.path());
        assert_eq!(unique_hash, "bbb");
    }

    #[test]
    fn test_generate_unique_name_with_prefix_suffix() {
        let dir = tempdir().unwrap();
        let alphabet: Vec<char> = vec!['a', 'b'];
        let length = 2;
        let prefix = Some("pre_");
        let suffix = Some("_suf");

        // Create files blocking most combinations
        let combinations = ["pre_aa_suf", "pre_ab_suf", "pre_ba_suf"];
        for combo in &combinations {
            fs::write(dir.path().join(combo), "content").unwrap();
        }

        // The only remaining combination is "bb"
        let unique_hash = generate_unique_name(&alphabet, length, prefix, suffix, dir.path());
        assert_eq!(unique_hash, "bb");
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
    fn test_api_key_fixed_length() {
        // Test that api-key option uses fixed 44 characters
        let args_vec = vec!["hashrand", "--api-key"];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert!(args.api_key);
        assert!(!args.password);
        assert_eq!(args.length, 21); // Original default before modification in main
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
    fn test_api_key_cannot_have_custom_length() {
        // Test that api-key option cannot have custom length
        let args_vec = vec!["hashrand", "--api-key", "20"];
        let result = Args::try_parse_from(&args_vec);
        assert!(result.is_err());
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
    fn test_alphabet_type_selection() {
        assert!(matches!(
            get_alphabet(&AlphabetType::Base58),
            &[_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _]
        ));
        assert_eq!(get_alphabet(&AlphabetType::Base58).len(), 58);
        assert_eq!(get_alphabet(&AlphabetType::NoLookAlike).len(), 57);
        assert_eq!(get_alphabet(&AlphabetType::Full).len(), 62);
        assert_eq!(get_alphabet(&AlphabetType::FullWithSymbols).len(), 73);
    }

    #[test]
    fn test_generate_hash_from_request() {
        let request = HashRequest {
            length: 10,
            alphabet: AlphabetType::Base58,
            raw: true,
            check: false,
            prefix: Some("test_".to_string()),
            suffix: Some("_end".to_string()),
        };
        
        let result = generate_hash_from_request(&request).unwrap();
        assert!(result.starts_with("test_"));
        assert!(result.ends_with("_end"));
        assert_eq!(result.len(), 10 + 5 + 4); // length + prefix + suffix
        assert!(!result.contains('\n')); // raw output
    }

    #[test]
    fn test_generate_hash_from_request_with_newline() {
        let request = HashRequest {
            length: 8,
            alphabet: AlphabetType::Full,
            raw: false,
            check: false,
            prefix: None,
            suffix: None,
        };
        
        let result = generate_hash_from_request(&request).unwrap();
        assert_eq!(result.len(), 9); // 8 chars + newline
        assert!(result.ends_with('\n'));
    }

    #[test]
    fn test_generate_api_key_response() {
        let result = generate_api_key_response(true).unwrap();
        assert!(result.starts_with("ak_"));
        assert_eq!(result.len(), 47); // ak_ + 44 chars
        assert!(!result.contains('\n')); // raw output
        
        let result = generate_api_key_response(false).unwrap();
        assert!(result.starts_with("ak_"));
        assert!(result.ends_with('\n'));
        assert_eq!(result.len(), 48); // ak_ + 44 chars + newline
    }

    #[test]
    fn test_generate_password_response() {
        // Default length
        let result = generate_password_response(None, true).unwrap();
        assert_eq!(result.len(), 21);
        assert!(!result.contains('\n'));
        
        // Custom length within range
        let result = generate_password_response(Some(30), false).unwrap();
        assert_eq!(result.len(), 31); // 30 chars + newline
        assert!(result.ends_with('\n'));
        
        // Length too short
        let result = generate_password_response(Some(15), true);
        assert!(result.is_err());
        
        // Length too long
        let result = generate_password_response(Some(50), true);
        assert!(result.is_err());
        
        // Boundary values
        let result = generate_password_response(Some(21), true).unwrap();
        assert_eq!(result.len(), 21);
        
        let result = generate_password_response(Some(44), true).unwrap();
        assert_eq!(result.len(), 44);
    }

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
}