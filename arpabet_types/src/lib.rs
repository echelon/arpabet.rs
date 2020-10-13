// Copyright (c) 2015, 2018, 2020 Brandon Thomas <bt@brand.io>

#![deny(dead_code)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![deny(unused_extern_crates)]
#![deny(unused_imports)]
#![deny(unused_qualifications)]

//! This crate contains the core types for the **arpabet** crate. This crate is
//! split into sub-crates to aid in compile-time loading of the CMUdict.
//!
//! You shouldn't need to import this crate directly. The **arpabet** crate
//! includes this transitively.
//! ```

#[cfg(test)] #[macro_use] extern crate expectest;

pub mod constants;
pub mod error;
pub mod extensions;
pub mod phoneme;

pub use constants::*;
pub use error::*;
pub use extensions::*;
pub use phoneme::*;
use std::collections::HashMap;
use std::collections::hash_map::Keys;

/// A word is a simple string containing no space characters.
pub type Word = String;

/// A polyphone is several phonemes read in order, typically as a single word.
pub type Polyphone = Vec<Phoneme>;

/// A dictionary that contains mappings of words to polyphones.
#[derive(Default, Clone)]
pub struct Arpabet {
  /// A map of lowercase words to polyphone breakdown.
  /// eg. 'jungle' -> [JH, AH1, NG, G, AH0, L]
  dictionary: HashMap<Word, Polyphone>,
}

impl Arpabet {
  /// Create an empty Arpabet.
  pub fn new() -> Arpabet {
    Self {
      dictionary: HashMap::new(),
    }
  }

  /// Create an Arpabet from a map.
  /// Consumes the map.
  pub fn from_map(map: HashMap<Word, Polyphone>) -> Self {
    Self {
      dictionary: map
    }
  }

  /// Create an Arpabet from a phf::Map.
  /// Used internally for allocation from codegen.
  /// Unfortunately this needs to allocate a new HashMap and copy data over.
  pub fn from_phf_map(map: &phf::Map<&str, &[Phoneme]>) -> Self {
    // TODO: An internal store over an enum of HashMap / phf::Map would be better.
    let mut hashmap = HashMap::with_capacity(map.len());

    for (k, v) in map.into_iter() {
      hashmap.insert(k.to_string(), v.to_vec());
    }

    Self {
      dictionary: hashmap,
    }
  }

  /// Get a polyphone from the dictionary.
  pub fn get_polyphone(&self, word: &str) -> Option<Polyphone> {
    self.dictionary.get(word).and_then(|p| {
      Some(p.iter()
        .map(|p| p.clone())
        .collect::<Vec<Phoneme>>())
    })
  }

  /// Get a polyphone from the dictionary.
  pub fn get_polyphone_ref(&self, word: &str) -> Option<&Polyphone> {
    self.dictionary.get(word)
  }

  /// Get a polyphone from the dictionary.
  pub fn get_polyphone_str(&self, word: &str) -> Option<Vec<&'static str>> {
    self.dictionary.get(word)
      .map(|polyphone| {
        polyphone.iter()
          .map(|phoneme| phoneme.to_str())
          .collect()
      })
  }

  /// Combine two Arpabets and return the result.
  /// Items in the second Arpabet take precedence.
  pub fn combine(&self, other: &Arpabet) -> Arpabet {
    let mut merged = self.dictionary.clone();
    for (k, v) in other.dictionary.iter() {
      merged.insert(k.clone(), v.clone());
    }
    Arpabet { dictionary: merged }
  }

  /// Merge the supplied Arpabet into the current one.
  /// Items in the supplied Arpabet override existing entries
  /// should they already exist.
  pub fn merge_from(&mut self, other: &Arpabet) {
    for (k, v) in other.dictionary.iter() {
      self.dictionary.insert(k.clone(), v.clone());
    }
  }

  /// Insert an entry into the Arpabet. If the entry is already present,
  /// replace it and return the old value.
  pub fn insert(&mut self, key: Word, value: Vec<Phoneme>) -> Option<Vec<Phoneme>> {
    self.dictionary.insert(key, value)
  }

  /// Remove an entry from the arpabet. If it is present, it will be returned.
  pub fn remove(&mut self, key: &str) -> Option<Vec<Phoneme>> {
    self.dictionary.remove(key)
  }

  /// Return a keys iterator that walks the keys in random order.
  pub fn keys(&self) -> Keys<String, Vec<Phoneme>> {
    self.dictionary.keys()
  }

  /// Reports the number of entries in the arpabet.
  pub fn len(&self) -> usize {
    self.dictionary.len()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use phoneme::{
    Consonant,
    Vowel,
    VowelStress,
  };

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
