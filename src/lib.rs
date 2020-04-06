
#[cfg(feature = "ascii")]
mod data_ascii;
#[cfg(feature = "ascii")]
use data_ascii::LAY_CHARS;

#[cfg(feature = "bmp")]
mod data_bmp;
#[cfg(feature = "bmp")]
use data_bmp::LAY_CHARS;

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
pub fn normalized_lower_lay_string(s: &str) -> String {
    s.nfc()
        .map(|c| lower_lay_char(c))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normalized_lower_lay_string() {
        let s = "Comunicações"; // unnormalized string
        let nor = s.nfc().collect::<String>();
        println!("{:?}", nor);
        assert_eq!(s.chars().count(), 15);
        let s = normalized_lower_lay_string(s);
        assert_eq!(s.chars().count(), 12);
        assert_eq!(s, "comunicacoes");
    }
}

