
#[cfg(feature = "ascii")]
mod data_ascii;
#[cfg(feature = "ascii")]
use data_ascii::LAY_CHARS;

#[cfg(feature = "bmp")]
mod data_bmp;
#[cfg(feature = "bmp")]
use data_bmp::LAY_CHARS;

#[cfg(feature = "normalization")]
use unicode_normalization::{
    UnicodeNormalization,
};

/// try to return a lowercased diacritics-free version
/// of the character.
#[inline(always)]
pub fn lower_lay_char(c: char) -> char {
    if (c as usize) < LAY_CHARS.len() {
        unsafe {
            *LAY_CHARS.get_unchecked(c as usize)
        }
    } else {
        c
    }
}

/// replace every character with its lowercased diacritics-free equivalent
/// whenever possible.
/// By construct, the resulting string is guaranteed to have the same number
/// of characters as the input one (it may be smaller in bytes but not larger).
/// This function doesn't do any normalization. It's thus necessary to ensure
/// the string is already normalized.
pub fn lower_lay_string(s: &str) -> String {
    s.chars()
        .map(|c| lower_lay_char(c))
        .collect()
}

/// normalize the string then replace every character with its
/// lowercased diacritics-free equivalent whenever possible.
#[cfg(feature = "normalization")]
pub fn normalized_lower_lay_string(s: &str) -> String {
    s.nfc()
        .map(|c| lower_lay_char(c))
        .collect()
}


// To test, run
//     cargo test --features="bmp, normalization"
#[cfg(all(test, feature="normalization"))]
mod tests {
    use super::*;
    #[test]
    fn test_lower_lay_char() {
        let s = "Comunicações"; // normalized string (length=12)
        let chars: Vec<char> = s.chars().collect();
        assert_eq!(chars.len(), 12);
        assert_eq!(chars[0], 'C');
        assert_eq!(lower_lay_char(chars[0]), 'c');
        assert_eq!(chars[8], 'ç');
        assert_eq!(lower_lay_char(chars[8]), 'c');
    }
    #[test]
    fn test_normalized_lower_lay_string() {
        let s = "Comunicações"; // unnormalized string (length=14)
        assert_eq!(s.chars().count(), 14);
        let s = normalized_lower_lay_string(s);
        assert_eq!(s.chars().count(), 12);
        assert_eq!(s, "comunicacoes");
    }
}

