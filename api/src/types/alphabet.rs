use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Available alphabet types for hash generation
/// DRY: Integer mapping defined once using num_enum derive macros
#[derive(Debug, Clone, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum AlphabetType {
    Base58 = 0,          // 58 characters - Bitcoin alphabet (default)
    NoLookAlike = 1,     // 49 characters - Maximum readability
    Full = 2,            // 62 characters - Full alphanumeric
    FullWithSymbols = 3, // 73 characters - Maximum entropy
    Numeric = 4,         // 10 characters - Only digits 0-9
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_valid_indices() {
        // num_enum generates try_from() automatically
        assert_eq!(AlphabetType::try_from(0).unwrap(), AlphabetType::Base58);
        assert_eq!(
            AlphabetType::try_from(1).unwrap(),
            AlphabetType::NoLookAlike
        );
        assert_eq!(AlphabetType::try_from(2).unwrap(), AlphabetType::Full);
        assert_eq!(
            AlphabetType::try_from(3).unwrap(),
            AlphabetType::FullWithSymbols
        );
        assert_eq!(AlphabetType::try_from(4).unwrap(), AlphabetType::Numeric);
    }

    #[test]
    fn test_try_from_invalid_indices() {
        // num_enum handles invalid indices automatically
        assert!(AlphabetType::try_from(5).is_err());
        assert!(AlphabetType::try_from(10).is_err());
        assert!(AlphabetType::try_from(255).is_err());
    }

    #[test]
    fn test_into_u8() {
        // num_enum generates Into<u8> automatically
        assert_eq!(u8::from(AlphabetType::Base58), 0);
        assert_eq!(u8::from(AlphabetType::NoLookAlike), 1);
        assert_eq!(u8::from(AlphabetType::Full), 2);
        assert_eq!(u8::from(AlphabetType::FullWithSymbols), 3);
        assert_eq!(u8::from(AlphabetType::Numeric), 4);
    }

    #[test]
    fn test_round_trip_conversion() {
        // Test that converting to u8 and back gives the same value
        for i in 0..=4 {
            let alphabet = AlphabetType::try_from(i).unwrap();
            assert_eq!(u8::from(alphabet), i);
        }
    }
}
