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

extern crate arpabet_types;
extern crate phf;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[cfg(test)]
mod tests {
  // NB: Codegen.
  use super::CMU_DICT;

  #[test]
  fn test_cmudict_codegen() {
    assert_eq!(CMU_DICT.len(), 133_793 + 1);
    //assert_eq!(CMU_DICT_2.get("A"), Some(&["T"]));
  }
}
