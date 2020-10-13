// Copyright (c) 2015, 2018, 2020 Brandon Thomas <bt@brand.io>

#![deny(dead_code)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![deny(unused_extern_crates)]
#![deny(unused_imports)]
#![deny(unused_qualifications)]

//! **Arpabet (_A1 R P AH0 B EH2 T_)**, a library for speech synthesis that
//! leverages Carnegie Mellon University's _[CMUdict](http://www.speech.cs.cmu.edu/cgi-bin/cmudict)_.
//! This is a simple library to enable the building of concatenative speech
//! synthesis engines.
//!
//! Usage:
//!
//! ```
//! extern crate arpabet;
//! use arpabet::load_cmudict;
//!
//! let arpabet = load_cmudict();
//!
//! assert_eq!(arpabet.get_polyphone_str("test"),
//!   Some(vec!["T".into(), "EH1".into(), "S".into(), "T".into()]));
//! ```

extern crate arpabet_cmudict;
extern crate arpabet_parser;
extern crate arpabet_types;

// We simply re-export the symbols in the shape of the original arpabet crate
// as it was before its decomposition into several crates.
pub use arpabet_cmudict::load_cmudict;
pub use arpabet_parser::load_from_file;
pub use arpabet_parser::load_from_str;
pub use arpabet_types::Arpabet;
pub use arpabet_types::Phoneme;
pub use arpabet_types::Polyphone;
pub use arpabet_types::Word;
pub use arpabet_types::constants::ALL_CONSONANTS;
pub use arpabet_types::constants::ALL_PUNCTUATION;
pub use arpabet_types::constants::ALL_VOWELS;
pub use arpabet_types::constants::PHONEME_MAP;
pub use arpabet_types::constants;
pub use arpabet_types::error::ArpabetError;
pub use arpabet_types::error;
pub use arpabet_types::extensions;
pub use arpabet_types::phoneme;

// Integration tests.
#[cfg(test)]
mod tests {
  use crate as arpabet;

  #[test]
  fn integration_test_load_cmudict() {
    let cmudict = arpabet::load_cmudict();

    assert_eq!(cmudict.get_polyphone_str("game"),
      Some(vec!["G", "EY1", "M"]));

    assert_eq!(cmudict.get_polyphone_str("boy"),
      Some(vec!["B", "OY1"]));

    assert_eq!(cmudict.get_polyphone_str("advance"),
      Some(vec!["AH0", "D", "V", "AE1", "N", "S"]));

    assert_eq!(cmudict.get_polyphone_str("sp"), None);

    assert_eq!(cmudict.get_polyphone_str("ZZZZZ"), None);
  }

  #[test]
  fn test_symbols_reexported() {
    // We're just testing that the symbols are exported.

    // Typedefs
    let _a : arpabet::Polyphone = vec![];
    let _b : arpabet::Word = "".to_string();

    // Structs
    let _c = arpabet::extensions::SentenceToken::Phoneme(
      arpabet::Phoneme::Consonant(
        arpabet::phoneme::Consonant::B));

    let _d = arpabet::extensions::SentenceToken::Punctuation(
      arpabet::extensions::Punctuation::Comma);

    let _e = arpabet::Phoneme::Vowel(
      arpabet::phoneme::Vowel::AA(arpabet::phoneme::VowelStress::PrimaryStress));

    // Constants
    assert_eq!(arpabet::ALL_CONSONANTS.len(), 31);
    assert_eq!(arpabet::ALL_VOWELS.len(), 76);
    assert_eq!(arpabet::ALL_PUNCTUATION.len(), 10);
    assert_eq!(arpabet::PHONEME_MAP.len(), 107);

    // Core structs + Errors
    assert_eq!(arpabet::Arpabet::new().len(), 0);
    assert_eq!(&arpabet::ArpabetError::EmptyFile.to_string(), "The file was empty.");
  }
}
