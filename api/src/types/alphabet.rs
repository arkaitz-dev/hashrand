/// Available alphabet types for hash generation
#[derive(Debug, Clone, PartialEq)]
pub enum AlphabetType {
    Base58,          // 58 characters - Bitcoin alphabet (default)
    NoLookAlike,     // 49 characters - Maximum readability
    Full,            // 62 characters - Full alphanumeric
    FullWithSymbols, // 73 characters - Maximum entropy
    Numeric,         // 10 characters - Only digits 0-9
}

impl AlphabetType {
    /// Returns the alphabet character string
    pub fn chars(&self) -> &'static str {
        match self {
            AlphabetType::Base58 => "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz",
            AlphabetType::NoLookAlike => "346789ABCDEFGHJKLMNPQRTUVWXYabcdefghijkmnpqrtwxyz",
            AlphabetType::Full => "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
            AlphabetType::FullWithSymbols => {
                "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_*^@#+!?$%"
            }
            AlphabetType::Numeric => "0123456789",
        }
    }

    /// Returns the alphabet as a character vector for nanoid
    pub fn as_chars(&self) -> Vec<char> {
        self.chars().chars().collect()
    }

    /// Parses string to AlphabetType
    pub fn from_str(s: &str) -> Self {
        match s {
            "base58" => AlphabetType::Base58,
            "no-look-alike" => AlphabetType::NoLookAlike,
            "full" => AlphabetType::Full,
            "full-with-symbols" => AlphabetType::FullWithSymbols,
            "numeric" => AlphabetType::Numeric,
            _ => AlphabetType::Base58, // Default
        }
    }

    /// Returns the recommended minimum length for this alphabet
    #[allow(dead_code)]
    pub fn min_length(&self) -> usize {
        match self {
            AlphabetType::Base58 => 21,
            AlphabetType::NoLookAlike => 24,
            AlphabetType::Full => 21,
            AlphabetType::FullWithSymbols => 21,
            AlphabetType::Numeric => 35, // Higher due to lower entropy (10 vs 58+ chars)
        }
    }
}
