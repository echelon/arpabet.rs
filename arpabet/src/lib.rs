// Copyright (c) 2015, 2018, 2020 Brandon Thomas <bt@brand.io>

//#![deny(dead_code)]
// #![deny(missing_docs)]
// #![deny(unreachable_patterns)]
// #![deny(unused_extern_crates)]
// #![deny(unused_imports)]
// #![deny(unused_qualifications)]

//! **Arpabet (_A1 R P AH0 B EH2 T_)**, a library for speech synthesis that
//! leverages Carnegie Mellon University's _[CMUdict](http://www.speech.cs.cmu.edu/cgi-bin/cmudict)_.
//! This is a simple library to enable the building of concatenative speech
//! synthesis engines.
//!
//! Usage:
//!
//! TODO: FIX THIS
//! ```text
//! extern crate arpabet;
//! use arpabet::Arpabet;
//!
//! let arpabet = Arpabet::load_cmudict();
//!
//! assert_eq!(arpabet.get_polyphone_str("test"),
//!   Some(vec!["T".into(), "EH1".into(), "S".into(), "T".into()]));
//! ```

#[macro_use] extern crate lazy_static;
extern crate phf;
extern crate regex;
extern crate arpabet_types;

#[cfg(test)] extern crate chrono;
#[cfg(test)] #[macro_use] extern crate expectest;

pub use arpabet_types::constants;
pub use arpabet_types::error;
pub use arpabet_types::phoneme;
pub use arpabet_types::extensions;

use regex::Regex;
use std::collections::HashMap;
use std::collections::hash_map::Keys;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub use arpabet_types::Phoneme;
pub use arpabet_types::Word;
pub use arpabet_types::Polyphone;
pub use arpabet_types::Arpabet;

pub use error::ArpabetError;

pub use constants::ALL_CONSONANTS;
pub use constants::ALL_PUNCTUATION;
pub use constants::ALL_VOWELS;
pub use constants::PHONEME_MAP;

//pub const CMU_DICT_TEXT : &'static str = include_str!("../../cmudict/cmudict-0.7b");

// TODO
// lazy_static! {
//   // TODO: When static constexpr are added to Rust, evaluate this at compile time.
//   // Lazily cached copy of the entire CMU arpabet.
//   static ref CMU_DICT : Arpabet = Arpabet::load_from_str(CMU_DICT_TEXT)
//       .expect("CMU dictionary should lazily load.");
// }

// TODO
//include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

// TODO
// pub fn parse_keyword(keyword: &str) -> Option<Keyword> {
//   KEYWORDS.get(keyword).cloned()
// }


#[cfg(test)]
mod tests {
  use chrono::prelude::*;
  use expectest::prelude::*;

  use super::*;

  use phoneme::{
    Consonant,
    Vowel,
    VowelStress,
  };

  // #[test]
  // fn test_build() {
  //   assert_eq!(CMU_DICT_2.len(), 133_793);
  //   //assert_eq!(CMU_DICT_2.get("A"), Some(&["T"]));
  // }

  // #[test]
  // fn load_cmudict() {
  //   let arpabet = Arpabet::load_cmudict();

  //   assert_eq!(arpabet.get_polyphone_str("game"),
  //     Some(vec!["G", "EY1", "M"]));

  //   assert_eq!(arpabet.get_polyphone_str("boy"),
  //     Some(vec!["B", "OY1"]));

  //   assert_eq!(arpabet.get_polyphone_str("advance"),
  //     Some(vec!["AH0", "D", "V", "AE1", "N", "S"]));

  //   assert_eq!(arpabet.get_polyphone_str("sp"), None);

  //   assert_eq!(arpabet.get_polyphone_str("ZZZZZ"), None);
  // }

  // #[test]
  // fn cmudict_is_cached() {
  //   let _ = Arpabet::load_cmudict(); // pre-cache

  //   let start = Utc::now();

  //   for _ in 0 .. 1000 {
  //     // This should be cached...
  //     let arpabet = Arpabet::load_cmudict();

  //     assert_eq!(arpabet.get_polyphone_str("yep"),
  //       Some(vec!["Y", "EH1", "P"]));
  //   }

  //   let end = Utc::now();
  //   let duration = end.signed_duration_since(start);
  //   expect!(duration.num_milliseconds()).to(be_less_than(1_000));
  // }

  #[test]
  fn insert() {
    let mut arpa = Arpabet::new();
    arpa.insert("foo".to_string(), vec![
      Phoneme::Consonant(Consonant::F),
      Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress)),
    ]);

    assert_eq!(arpa.get_polyphone("foo"), Some(vec![
      Phoneme::Consonant(Consonant::F),
      Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress))],
    ));

    assert_eq!(arpa.get_polyphone("bar"), None);

    arpa.insert("foo".to_string(), vec![
      Phoneme::Consonant(Consonant::B),
      Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress)),
    ]);

    assert_eq!(arpa.get_polyphone("foo"), Some(vec![
      Phoneme::Consonant(Consonant::B),
      Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress))],
    ));
  }

  #[test]
  fn remove() {
    let mut arpa = Arpabet::new();
    arpa.insert("foo".to_string(), vec![
      Phoneme::Consonant(Consonant::F),
      Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress)),
    ]);

    arpa.insert("boo".to_string(), vec![
      Phoneme::Consonant(Consonant::B),
      Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress)),
    ]);

    assert_eq!(arpa.get_polyphone("foo"), Some(vec![
      Phoneme::Consonant(Consonant::F),
      Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress))],
    ));
    assert_eq!(arpa.get_polyphone("boo"), Some(vec![
      Phoneme::Consonant(Consonant::B),
      Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress))],
    ));
    assert_eq!(arpa.len(), 2);

    arpa.remove("boo");
    assert_eq!(arpa.get_polyphone("foo"), Some(vec![
      Phoneme::Consonant(Consonant::F),
      Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress))],
    ));
    assert_eq!(arpa.get_polyphone("boo"), None);
    assert_eq!(arpa.len(), 1);

    arpa.remove("foo");
    assert_eq!(arpa.get_polyphone("foo"), None);
    assert_eq!(arpa.get_polyphone("boo"), None);
    assert_eq!(arpa.len(), 0);
  }

  #[test]
  fn size() {
    let mut arpa = Arpabet::new();
    assert_eq!(arpa.len(), 0);

    arpa.insert("foo".to_string(), vec![
      Phoneme::Consonant(Consonant::F),
      Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress)),
    ]);

    assert_eq!(arpa.len(), 1);

    arpa.insert("boo".to_string(), vec![
      Phoneme::Consonant(Consonant::B),
      Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress)),
    ]);

    assert_eq!(arpa.len(), 2);

    arpa.remove("boo");
    assert_eq!(arpa.len(), 1);

    arpa.remove("foo");
    assert_eq!(arpa.len(), 0);
  }

  #[test]
  fn keys() {
    let mut arpa = Arpabet::new();
    arpa.insert("foo".to_string(), vec![
      Phoneme::Consonant(Consonant::F),
      Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress)),
    ]);
    arpa.insert("boo".to_string(), vec![
      Phoneme::Consonant(Consonant::B),
      Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress)),
    ]);

    let keys: Vec<String> = arpa.keys().cloned().collect();
    assert_eq!(keys.len(), 2);

    // NB: contains is meh, see: https://github.com/rust-lang/rust/issues/42671
    assert!(keys.iter().any(|x| x == "foo"));
    assert!(keys.iter().any(|x| x == "boo"));
  }

  #[test]
  fn get_polyphone() {
    let mut a = Arpabet::new();
    a.insert("foo".to_string(), vec![
      Phoneme::Consonant(Consonant::F),
      Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress)),
    ]);
    assert_eq!(a.get_polyphone("foo"), Some(vec![
      Phoneme::Consonant(Consonant::F),
      Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress))],
    ));
    assert_eq!(a.get_polyphone("bar"), None);
  }

  #[test]
  fn get_polyphone_str() {
    let mut a = Arpabet::new();
    a.insert("foo".to_string(), vec![
      Phoneme::Consonant(Consonant::F),
      Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress)),
    ]);
    assert_eq!(a.get_polyphone_str("foo"), Some(vec!["F", "UW1"]));
    assert_eq!(a.get_polyphone_str("bar"), None);
  }

  #[test]
  fn get_polyphone_ref() {
    let mut a = Arpabet::new();
    a.insert("foo".to_string(), vec![
      Phoneme::Consonant(Consonant::F),
      Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress)),
    ]);
    assert_eq!(a.get_polyphone_ref("foo"), Some(&vec![
      Phoneme::Consonant(Consonant::F),
      Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress))],
    ));
    assert_eq!(a.get_polyphone_ref("bar"), None);
  }

  #[test]
  fn combine() {
    let a = {
      let mut arpa = Arpabet::new();
      arpa.insert("foo".to_string(), vec![
        Phoneme::Consonant(Consonant::F),
        Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress)),
      ]);
      arpa.insert("bar".to_string(), vec![
        Phoneme::Consonant(Consonant::B),
        Phoneme::Vowel(Vowel::AA(VowelStress::PrimaryStress)),
        Phoneme::Consonant(Consonant::R),
      ]);
      arpa
    };
    let b = {
      let mut arpa = Arpabet::new();
      arpa.insert("foo".to_string(), vec![
        Phoneme::Consonant(Consonant::B),
        Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress)),
      ]);
      arpa.insert("baz".to_string(), vec![
        Phoneme::Consonant(Consonant::B),
        Phoneme::Vowel(Vowel::AE(VowelStress::PrimaryStress)),
        Phoneme::Consonant(Consonant::Z),
      ]);
      arpa
    };

    let c = a.combine(&b);

    assert_eq!(c.get_polyphone("foo"), Some(vec![
        Phoneme::Consonant(Consonant::B),
        Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress)),
      ]));
    assert_eq!(c.get_polyphone("bar"), Some(vec![
        Phoneme::Consonant(Consonant::B),
        Phoneme::Vowel(Vowel::AA(VowelStress::PrimaryStress)),
        Phoneme::Consonant(Consonant::R),
      ]));
    assert_eq!(c.get_polyphone("baz"), Some(vec![
        Phoneme::Consonant(Consonant::B),
        Phoneme::Vowel(Vowel::AE(VowelStress::PrimaryStress)),
        Phoneme::Consonant(Consonant::Z),
      ]));
    assert_eq!(c.get_polyphone("bin"), None);
  }

  #[test]
  fn merge_from() {
    let mut a = Arpabet::new();
    a.insert("foo".to_string(), vec![
      Phoneme::Consonant(Consonant::F),
      Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress)),
    ]);
    a.insert("bar".to_string(), vec![
      Phoneme::Consonant(Consonant::B),
      Phoneme::Vowel(Vowel::AA(VowelStress::PrimaryStress)),
      Phoneme::Consonant(Consonant::R),
    ]);

    let b = {
      let mut arpa = Arpabet::new();
      arpa.insert("foo".to_string(), vec![
        Phoneme::Consonant(Consonant::B),
        Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress)),
      ]);
      arpa.insert("baz".to_string(), vec![
        Phoneme::Consonant(Consonant::B),
        Phoneme::Vowel(Vowel::AE(VowelStress::PrimaryStress)),
        Phoneme::Consonant(Consonant::Z),
      ]);
      arpa
    };

    a.merge_from(&b);

    assert_eq!(a.get_polyphone("foo"), Some(vec![
      Phoneme::Consonant(Consonant::B),
      Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress)),
    ]));
    assert_eq!(a.get_polyphone("bar"), Some(vec![
      Phoneme::Consonant(Consonant::B),
      Phoneme::Vowel(Vowel::AA(VowelStress::PrimaryStress)),
      Phoneme::Consonant(Consonant::R),
    ]));
    assert_eq!(a.get_polyphone("baz"), Some(vec![
      Phoneme::Consonant(Consonant::B),
      Phoneme::Vowel(Vowel::AE(VowelStress::PrimaryStress)),
      Phoneme::Consonant(Consonant::Z),
    ]));
    assert_eq!(a.get_polyphone("bin"), None);
  }
}
