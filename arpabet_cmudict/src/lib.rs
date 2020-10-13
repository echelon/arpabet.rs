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

extern crate arpabet_types;
extern crate phf;

use arpabet_types::Arpabet;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

/// Load the CMUdict.
pub fn load_cmudict() -> Arpabet {
  Arpabet::from_phf_map(&CMU_DICT)
}

#[cfg(test)]
mod tests {
  // NB: Codegen.
  use super::CMU_DICT;
  use arpabet_types::{Phoneme, Consonant, Vowel, VowelStress};

  #[test]
  fn test_cmudict_codegen_length() {
    assert_eq!(CMU_DICT.len(), 133_793);
  }

  #[test]
  fn test_cmudict_codegen_entries() {
    // Y OW1 SH IY0
    let expected = vec![
      Phoneme::Consonant(Consonant::Y),
      Phoneme::Vowel(Vowel::OW(VowelStress::PrimaryStress)),
      Phoneme::Consonant(Consonant::SH),
      Phoneme::Vowel(Vowel::IY(VowelStress::NoStress)),
    ];
    assert_eq!(CMU_DICT.get("yoshi").map(|res| res.to_vec()), Some(expected));
  }
}
