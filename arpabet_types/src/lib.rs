#[cfg(test)] #[macro_use] extern crate expectest;
#[macro_use] extern crate lazy_static;

pub mod constants;
pub mod error;
pub mod extensions;
pub mod phoneme;

pub use constants::*;
pub use error::*;
pub use extensions::*;
pub use phoneme::*;
use regex::Regex;
use std::collections::HashMap;
use std::collections::hash_map::Keys;
use std::fs::File;
use std::io::{BufReader, BufRead};

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
    Arpabet {
      dictionary: HashMap::new(),
    }
  }

  /// Create an Arpabet from a map.
  /// Consumes the map.
  pub fn from_map(map: HashMap<Word, Polyphone>) -> Self {
    Arpabet {
      dictionary: map
    }
  }

  // TODO: When static constexpr are added to Rust, evaluate this at compile time.
  /// Loads and caches the CMU Arpabet, which is already present in an unparsed
  /// form in memory.
  // TODO: Remove or replace API
  // pub fn load_cmudict() -> &'static Arpabet {
  //   &CMU_DICT
  // }

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
