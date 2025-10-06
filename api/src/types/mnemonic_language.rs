use bip39::Language;
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// BIP39 mnemonic language types
/// DRY: Integer mapping defined once using num_enum derive macros
/// Wrapper around bip39::Language for consistent architecture with AlphabetType
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum MnemonicLanguage {
    English = 0,
    Spanish = 1,
    French = 2,
    Portuguese = 3,
    Japanese = 4,
    SimplifiedChinese = 5,
    TraditionalChinese = 6,
    Italian = 7,
    Korean = 8,
    Czech = 9,
}

impl From<MnemonicLanguage> for Language {
    fn from(lang: MnemonicLanguage) -> Self {
        match lang {
            MnemonicLanguage::English => Language::English,
            MnemonicLanguage::Spanish => Language::Spanish,
            MnemonicLanguage::French => Language::French,
            MnemonicLanguage::Portuguese => Language::Portuguese,
            MnemonicLanguage::Japanese => Language::Japanese,
            MnemonicLanguage::SimplifiedChinese => Language::SimplifiedChinese,
            MnemonicLanguage::TraditionalChinese => Language::TraditionalChinese,
            MnemonicLanguage::Italian => Language::Italian,
            MnemonicLanguage::Korean => Language::Korean,
            MnemonicLanguage::Czech => Language::Czech,
        }
    }
}

impl MnemonicLanguage {
    /// Returns the default language (English)
    pub fn default() -> Self {
        MnemonicLanguage::English
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_valid_indices() {
        // num_enum generates try_from() automatically
        assert_eq!(
            MnemonicLanguage::try_from(0).unwrap(),
            MnemonicLanguage::English
        );
        assert_eq!(
            MnemonicLanguage::try_from(1).unwrap(),
            MnemonicLanguage::Spanish
        );
        assert_eq!(
            MnemonicLanguage::try_from(2).unwrap(),
            MnemonicLanguage::French
        );
        assert_eq!(
            MnemonicLanguage::try_from(3).unwrap(),
            MnemonicLanguage::Portuguese
        );
        assert_eq!(
            MnemonicLanguage::try_from(4).unwrap(),
            MnemonicLanguage::Japanese
        );
        assert_eq!(
            MnemonicLanguage::try_from(5).unwrap(),
            MnemonicLanguage::SimplifiedChinese
        );
        assert_eq!(
            MnemonicLanguage::try_from(6).unwrap(),
            MnemonicLanguage::TraditionalChinese
        );
        assert_eq!(
            MnemonicLanguage::try_from(7).unwrap(),
            MnemonicLanguage::Italian
        );
        assert_eq!(
            MnemonicLanguage::try_from(8).unwrap(),
            MnemonicLanguage::Korean
        );
        assert_eq!(
            MnemonicLanguage::try_from(9).unwrap(),
            MnemonicLanguage::Czech
        );
    }

    #[test]
    fn test_try_from_invalid_indices() {
        // num_enum handles invalid indices automatically
        assert!(MnemonicLanguage::try_from(10).is_err());
        assert!(MnemonicLanguage::try_from(99).is_err());
        assert!(MnemonicLanguage::try_from(255).is_err());
    }

    #[test]
    fn test_into_u8() {
        // num_enum generates Into<u8> automatically
        assert_eq!(u8::from(MnemonicLanguage::English), 0);
        assert_eq!(u8::from(MnemonicLanguage::Spanish), 1);
        assert_eq!(u8::from(MnemonicLanguage::French), 2);
        assert_eq!(u8::from(MnemonicLanguage::Portuguese), 3);
        assert_eq!(u8::from(MnemonicLanguage::Japanese), 4);
        assert_eq!(u8::from(MnemonicLanguage::SimplifiedChinese), 5);
        assert_eq!(u8::from(MnemonicLanguage::TraditionalChinese), 6);
        assert_eq!(u8::from(MnemonicLanguage::Italian), 7);
        assert_eq!(u8::from(MnemonicLanguage::Korean), 8);
        assert_eq!(u8::from(MnemonicLanguage::Czech), 9);
    }

    #[test]
    fn test_round_trip_conversion() {
        // Test that converting to u8 and back gives the same value
        for i in 0..=9 {
            let language = MnemonicLanguage::try_from(i).unwrap();
            assert_eq!(u8::from(language), i);
        }
    }

    #[test]
    fn test_conversion_to_bip39_language() {
        // Test conversion to bip39::Language
        assert_eq!(Language::from(MnemonicLanguage::English), Language::English);
        assert_eq!(Language::from(MnemonicLanguage::Spanish), Language::Spanish);
        assert_eq!(Language::from(MnemonicLanguage::Czech), Language::Czech);
    }
}
