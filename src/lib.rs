// Copyright (c) 2015, 2018 Brandon Thomas <bt@brand.io>

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
//! ```rust
//! extern crate arpabet;
//! use arpabet::Arpabet;
//!
//! let arpabet = Arpabet::load_cmudict();
//!
//! assert_eq!(arpabet.get_polyphone_ref("test"),
//!   Some(&vec!["T".into(), "EH1".into(), "S".into(), "T".into()]));
//! ```

#[macro_use] extern crate lazy_static;
extern crate regex;

#[cfg(test)] extern crate chrono;
#[cfg(test)] #[macro_use] extern crate expectest;

mod error;

use regex::Regex;
use std::collections::HashMap;
use std::collections::hash_map::Keys;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub use error::ArpabetError;

/// A word is a simple string containing no space characters.
pub type Word = String;

/// A phoneme is a simple string containing no space characters.
pub type Phoneme = String;

/// A polyphone is several phonemes read in order.
pub type Polyphone = Vec<Phoneme>;

const CMU_DICT_TEXT : &'static str = include_str!("../cmudict/cmudict-0.7b");

lazy_static! {
  // TODO: When static constexpr are added to Rust, evaluate this at compile time.
  // Lazily cached copy of the entire CMU arpabet.
  static ref CMU_DICT : Arpabet = Arpabet::load_from_str(CMU_DICT_TEXT)
      .expect("CMU dictionary should lazily load.");

  // Regex for reading CMU arpabet, or similarly formatted files.
  // Format resembles the following,
  // ABBREVIATE  AH0 B R IY1 V IY0 EY2 T
  static ref FILE_REGEX : Regex = Regex::new(r"^([\w\-\(\)\.']+)\s+([^\s].*)\s*$")
      .expect("Regex is correct.");

  // Comments begin with this preamble.
  static ref COMMENT_REGEX : Regex = Regex::new(r"^;;;\s+")
      .expect("Regex is correct.");
}

/// A dictionary that contains mappings of words to polyphones.
#[derive(Default, Clone)]
pub struct Arpabet {
  /// A map of lowercase words to polyphone breakdown (phones are uppercase).
  /// eg. 'jungle' -> [JH, AH1, NG, G, AH0, L]
  dictionary: HashMap<Word, Polyphone>
}

impl Arpabet {
  /// Create an empty Arpabet.
  pub fn new() -> Arpabet {
    Arpabet { dictionary: HashMap::new() }
  }

  // TODO: When static constexpr are added to Rust, evaluate this at compile time.
  /// Loads and caches the CMU Arpabet, which is already present in an unparsed
  /// form in memory.
  pub fn load_cmudict() -> &'static Arpabet {
    &CMU_DICT
  }

  /// Load a dictionary from text
  /// The file format is expected to match that of
  /// [CMUdict](http://www.speech.cs.cmu.edu/cgi-bin/cmudict).
  pub fn load_from_str(text: &str) -> Result<Arpabet, ArpabetError> {
    let mut map = HashMap::new();
    let mut reader = BufReader::new(text.as_bytes());

    let _r = Arpabet::read_lines(&mut reader, &mut map)?;

    if map.is_empty() {
      Err(ArpabetError::EmptyFile)
    } else {
      Ok(Arpabet { dictionary: map })
    }
  }

  /// Load a dictionary from file
  /// The file format is expected to match that of
  /// [CMUdict](http://www.speech.cs.cmu.edu/cgi-bin/cmudict).
  pub fn load_from_file(filename: &str) -> Result<Arpabet, ArpabetError> {
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let mut map = HashMap::new();

    let _r = Arpabet::read_lines(&mut reader, &mut map)?;

    if map.is_empty() {
      Err(ArpabetError::EmptyFile)
    } else {
      Ok(Arpabet { dictionary: map })
    }
  }

  /// Get a polyphone from the dictionary.
  pub fn get_polyphone_ref(&self, word: &str) -> Option<&Polyphone> {
    self.dictionary.get(word)
  }

  /// Get a polyphone from the dictionary.
  pub fn get_polyphone(&self, word: &str) -> Option<Polyphone> {
    self.dictionary.get(word).and_then(|p| {
      Some(p.iter().map(|s| s.to_string()).collect::<Vec<String>>())
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
  pub fn insert(&mut self, key: Word, value: Polyphone) -> Option<Polyphone> {
    self.dictionary.insert(key, value)
  }

  /// Remove an entry from the arpabet. If it is present, it will be returned.
  pub fn remove(&mut self, key: &str) -> Option<Polyphone> {
    self.dictionary.remove(key)
  }

  /// Return a keys iterator that walks the keys in random order.
  pub fn keys(&self) -> Keys<String, Polyphone> {
    self.dictionary.keys()
  }

  /// Reports the number of entries in the arpabet.
  pub fn len(&self) -> usize {
    self.dictionary.len()
  }

  fn read_lines(reader: &mut BufRead, map: &mut HashMap<Word, Polyphone>)
                -> Result<(), ArpabetError> {

    let mut buffer = String::new();
    let mut line_count = 1;

    while reader.read_line(&mut buffer)? > 0 {
      if COMMENT_REGEX.is_match(&buffer) {
        buffer.clear();
        line_count += 1;
        continue;
      }

      match FILE_REGEX.captures(&buffer) {
        None => return Err(ArpabetError::InvalidFormat {
          line_number: line_count,
          text: buffer.to_string(),
        }),
        Some(caps) => {

          let word = match caps.get(1) {
            None => return Err(ArpabetError::InvalidFormat {
              line_number: line_count,
              text: buffer.to_string(),
            }),
            Some(m) => m.as_str()
                .to_lowercase(),
          };

          let phonemes = match caps.get(2) {
            None => return Err(ArpabetError::InvalidFormat {
              line_number: line_count,
              text: buffer.to_string(),
            }),
            Some(m) => m.as_str()
                .split(" ")
                .map(|s| s.to_string().to_uppercase())
                .collect::<Vec<String>>(),
          };

          if phonemes.is_empty() {
            return Err(ArpabetError::InvalidFormat {
              line_number: line_count,
              text: buffer.to_string(),
            });
          }

          map.insert(word, phonemes);
        },
      }

      buffer.clear();
      line_count += 1;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use chrono::prelude::*;
  use expectest::prelude::*;

  use super::*;

  fn to_strings(strs: Vec<&str>) -> Vec<String> {
    strs.iter().map(|s| s.to_string()).collect()
  }

  #[test]
  fn load_from_str() {
    let text = "DOCTOR  D AA1 K T ER0\n\
                MARIO  M AA1 R IY0 OW0";

    let arpabet = Arpabet::load_from_str(text).expect("Text should load");

    assert_eq!(arpabet.get_polyphone("super"), None);

    assert_eq!(arpabet.get_polyphone("doctor"),
      Some(to_strings(vec!["D", "AA1", "K", "T","ER0"])));

    assert_eq!(arpabet.get_polyphone("mario"),
      Some(to_strings(vec!["M", "AA1", "R", "IY0","OW0"])));
  }

  #[test]
  fn load_from_str_error() {
    let text = "DOCTOR  D AA1 K T ER0\n\
                MARIO  M AA1 R IY0 OW0\n\
                WAT    ";

    match Arpabet::load_from_str(text) {
      Ok(_) => panic!("Should have errored."),
      Err(err) => match err {
        ArpabetError::InvalidFormat { line_number, text } => {
          assert_eq!(line_number, 3);
          assert_eq!(text, "WAT    ");
        },
        _ => panic!("Wrong error"),
      }
    }
  }

  #[test]
  fn load_from_file() {
    let arpabet = Arpabet::load_from_file("./tests/file_load_test.txt")
        .expect("File should load");

    assert_eq!(arpabet.get_polyphone("pokemon"),
      Some(to_strings(vec!["P", "OW1", "K", "EY1", "AH0", "N"])));

    assert_eq!(arpabet.get_polyphone("pikachu"),
      Some(to_strings(vec!["P", "IY1", "K", "AH0", "CH", "UW1"])));

    assert_eq!(arpabet.get_polyphone("bulbasaur"), None);
  }

  #[test]
  fn load_cmudict() {
    let arpabet = Arpabet::load_cmudict();

    assert_eq!(arpabet.get_polyphone("game"),
      Some(to_strings(vec!["G", "EY1", "M"])));

    assert_eq!(arpabet.get_polyphone("boy"),
      Some(to_strings(vec!["B", "OY1"])));

    assert_eq!(arpabet.get_polyphone("advance"),
      Some(to_strings(vec!["AH0", "D", "V", "AE1", "N", "S"])));

    assert_eq!(arpabet.get_polyphone("sp"), None);

    assert_eq!(arpabet.get_polyphone("ZZZZZ"), None);
  }

  #[test]
  fn cmudict_is_cached() {
    let _ = Arpabet::load_cmudict(); // pre-cache

    let start = Utc::now();

    for _ in 0 .. 1000 {
      // This should be cached...
      let arpabet = Arpabet::load_cmudict();

      assert_eq!(arpabet.get_polyphone("yep"),
        Some(to_strings(vec!["Y", "EH1", "P"])));
    }

    let end = Utc::now();
    let duration = end.signed_duration_since(start);
    expect!(duration.num_milliseconds()).to(be_less_than(1_000));
  }

  #[test]
  fn insert() {
    let mut arpa = Arpabet::new();
    arpa.insert("foo".to_string(), to_strings(vec!["F", "UW1"]));

    assert_eq!(arpa.get_polyphone("foo"), Some(to_strings(vec!["F", "UW1"])));
    assert_eq!(arpa.get_polyphone("bar"), None);

    arpa.insert("foo".to_string(), to_strings(vec!["B", "UW1"]));

    assert_eq!(arpa.get_polyphone("foo"), Some(to_strings(vec!["B", "UW1"])));
  }

  #[test]
  fn remove() {
    let mut arpa = Arpabet::new();
    arpa.insert("foo".to_string(), to_strings(vec!["F", "UW1"]));
    arpa.insert("boo".to_string(), to_strings(vec!["B", "UW1"]));

    assert_eq!(arpa.get_polyphone("foo"), Some(to_strings(vec!["F", "UW1"])));
    assert_eq!(arpa.get_polyphone("boo"), Some(to_strings(vec!["B", "UW1"])));
    assert_eq!(arpa.len(), 2);

    arpa.remove("boo");
    assert_eq!(arpa.get_polyphone("foo"), Some(to_strings(vec!["F", "UW1"])));
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

    arpa.insert("foo".to_string(), to_strings(vec!["F", "UW1"]));
    assert_eq!(arpa.len(), 1);

    arpa.insert("boo".to_string(), to_strings(vec!["B", "UW1"]));
    assert_eq!(arpa.len(), 2);

    arpa.remove("boo");
    assert_eq!(arpa.len(), 1);

    arpa.remove("foo");
    assert_eq!(arpa.len(), 0);
  }

  #[test]
  fn keys() {
    let mut arpa = Arpabet::new();
    arpa.insert("foo".to_string(), to_strings(vec!["F", "UW1"]));
    arpa.insert("boo".to_string(), to_strings(vec!["B", "UW1"]));

    let keys: Vec<String> = arpa.keys().cloned().collect();
    assert_eq!(keys.len(), 2);

    // NB: contains is meh, see: https://github.com/rust-lang/rust/issues/42671
    assert!(keys.iter().any(|x| x == "foo"));
    assert!(keys.iter().any(|x| x == "boo"));
  }

  #[test]
  fn get_polyphone() {
    let mut a = Arpabet::new();
    a.insert("foo".to_string(), to_strings(vec!["F", "UW1"]));
    assert_eq!(a.get_polyphone("foo"), Some(to_strings(vec!["F", "UW1"])));
    assert_eq!(a.get_polyphone("bar"), None);
  }

  #[test]
  fn get_polyphone_ref() {
    let mut a = Arpabet::new();
    a.insert("foo".to_string(), to_strings(vec!["F", "UW1"]));
    assert_eq!(a.get_polyphone_ref("foo"), Some(&to_strings(vec!["F", "UW1"])));
    assert_eq!(a.get_polyphone_ref("bar"), None);
  }

  #[test]
  fn combine() {
    let a = {
      let mut arpa = Arpabet::new();
      arpa.insert("foo".to_string(), to_strings(vec!["F", "UW1"]));
      arpa.insert("bar".to_string(), to_strings(vec!["B", "A1", "R"]));
      arpa
    };
    let b = {
      let mut arpa = Arpabet::new();
      arpa.insert("foo".to_string(), to_strings(vec!["B", "OO"]));
      arpa.insert("baz".to_string(), to_strings(vec!["B", "AE1", "Z"]));
      arpa
    };

    let c = a.combine(&b);

    assert_eq!(c.get_polyphone("foo"), Some(to_strings(vec!["B", "OO"])));
    assert_eq!(c.get_polyphone("bar"), Some(to_strings(vec!["B", "A1", "R"])));
    assert_eq!(c.get_polyphone("baz"), Some(to_strings(vec!["B", "AE1", "Z"])));
    assert_eq!(c.get_polyphone("bin"), None);
  }

  #[test]
  fn merge_from() {
    let mut a = Arpabet::new();
    a.insert("foo".to_string(), to_strings(vec!["F", "UW1"]));
    a.insert("bar".to_string(), to_strings(vec!["B", "A1", "R"]));

    let b = {
      let mut arpa = Arpabet::new();
      arpa.insert("foo".to_string(), to_strings(vec!["B", "OO"]));
      arpa.insert("baz".to_string(), to_strings(vec!["B", "AE1", "Z"]));
      arpa
    };

    a.merge_from(&b);

    assert_eq!(a.get_polyphone("foo"), Some(to_strings(vec!["B", "OO"])));
    assert_eq!(a.get_polyphone("bar"), Some(to_strings(vec!["B", "A1", "R"])));
    assert_eq!(a.get_polyphone("baz"), Some(to_strings(vec!["B", "AE1", "Z"])));
    assert_eq!(a.get_polyphone("bin"), None);
  }
}
