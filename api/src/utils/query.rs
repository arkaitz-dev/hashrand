use std::collections::HashMap;

/// Parses query parameters from a URL query string
/// 
/// # Arguments
/// * `query_string` - String with parameters in format "key1=value1&key2=value2"
/// 
/// # Example
/// ```
/// let params = parse_query_params("length=10&alphabet=base58&raw=true");
/// assert_eq!(params.get("length"), Some(&"10".to_string()));
/// ```
pub fn parse_query_params(query_string: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    
    if query_string.is_empty() {
        return params;
    }
    
    for pair in query_string.split('&') {
        let parts: Vec<&str> = pair.split('=').collect();
        if parts.len() == 2 {
            // Basic URL decoding - we don't have urlencoding available in Spin
            let key = decode_url_component(parts[0]);
            let value = decode_url_component(parts[1]);
            params.insert(key, value);
        }
    }
    
    params
}

/// Basic URL decoding for common cases
/// Only handles the most basic cases without external dependencies
fn decode_url_component(s: &str) -> String {
    s.replace("%20", " ")
     .replace("%21", "!")
     .replace("%22", "\"")
     .replace("%23", "#")
     .replace("%24", "$")
     .replace("%25", "%")
     .replace("%26", "&")
     .replace("%27", "'")
     .replace("%28", "(")
     .replace("%29", ")")
     .replace("%2A", "*")
     .replace("%2B", "+")
     .replace("%2C", ",")
     .replace("%2D", "-")
     .replace("%2E", ".")
     .replace("%2F", "/")
     .replace("%3A", ":")
     .replace("%3B", ";")
     .replace("%3C", "<")
     .replace("%3D", "=")
     .replace("%3E", ">")
     .replace("%3F", "?")
     .replace("%40", "@")
     .replace("%5B", "[")
     .replace("%5C", "\\")
     .replace("%5D", "]")
     .replace("%5E", "^")
     .replace("%5F", "_")
     .replace("%60", "`")
     .replace("%7B", "{")
     .replace("%7C", "|")
     .replace("%7D", "}")
     .replace("%7E", "~")
}