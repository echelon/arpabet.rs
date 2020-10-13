// Copyright (c) 2015, 2018, 2020 Brandon Thomas <bt@brand.io>

#![deny(dead_code)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![deny(unused_extern_crates)]
#![deny(unused_imports)]
#![deny(unused_qualifications)]

//! This crate contains a compile-time allocated
//! _[CMUdict](http://www.speech.cs.cmu.edu/cgi-bin/cmudict)_, which is
//! Carnegie Mellon University's massive dictionary of phoneme-annotated
//! English words.
//!
//! You shouldn't need to import this crate directly. The **arpabet** crate
//! includes this transitively.
//! ```

#[cfg(test)] #[macro_use] extern crate expectest;
#[cfg(test)] extern crate chrono;
#[macro_use] extern crate lazy_static;
extern crate arpabet_types;
extern crate phf;

use arpabet_types::Arpabet;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

// TODO: When static constexpr are added to Rust, evaluate this at compile time.
lazy_static! {
  // Lazily cached copy of the entire CMU arpabet.
  static ref ARPABET_CMU_DICT : Arpabet = load_cmudict_lazy();
}

/// Load the in-memory CMUdict.
/// The first call lazily converts an already in-memory phf::Map into wrapped HashMap.
/// In the future this may be further optimized away.
pub fn load_cmudict() -> &'static Arpabet {
  &ARPABET_CMU_DICT
}

/// Load the CMUdict lazily and cache it.
fn load_cmudict_lazy() -> Arpabet {
  Arpabet::from_phf_map(&CMU_DICT)
}

#[cfg(test)]
mod tests {
  use chrono::prelude::*;
  use expectest::prelude::*;

  // NB: Codegen.
  use super::{CMU_DICT, load_cmudict};
  use arpabet_types::{Phoneme, Consonant, Vowel, VowelStress};

  #[test]
  fn test_cmudict_codegen_length() {
    //assert_eq!(CMU_DICT.len(), 133_793);
  }

  #[test]
  fn test_cmudict_codegen_entries() {
    let expected = vec![
      Phoneme::Consonant(Consonant::Y),
      Phoneme::Vowel(Vowel::OW(VowelStress::PrimaryStress)),
      Phoneme::Consonant(Consonant::SH),
      Phoneme::Vowel(Vowel::IY(VowelStress::NoStress)),
    ];
    assert_eq!(CMU_DICT.get("yoshi").map(|res| res.to_vec()), Some(expected));
  }

  #[test]
  fn test_load_cmudict() {
    let arpabet = load_cmudict();

    assert_eq!(arpabet.get_polyphone_str("game"),
      Some(vec!["G", "EY1", "M"]));

    assert_eq!(arpabet.get_polyphone_str("boy"),
      Some(vec!["B", "OY1"]));

    assert_eq!(arpabet.get_polyphone_str("advance"),
      Some(vec!["AH0", "D", "V", "AE1", "N", "S"]));

    assert_eq!(arpabet.get_polyphone_str("sp"), None);

    assert_eq!(arpabet.get_polyphone_str("ZZZZZ"), None);
  }

  #[test]
  fn test_cmudict_is_cached() {
    let _ = load_cmudict(); // pre-cache

    let start = Utc::now();

    for _ in 0 .. 1000 {
      // This should be cached...
      let arpabet = load_cmudict();

      assert_eq!(arpabet.get_polyphone_str("yep"),
        Some(vec!["Y", "EH1", "P"]));
    }

    let end = Utc::now();
    let duration = end.signed_duration_since(start);
    expect!(duration.num_milliseconds()).to(be_less_than(1_000));
  }
}
