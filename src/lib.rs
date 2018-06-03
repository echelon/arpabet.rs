// TODO:
// 1. load_from_file + test
// 2. merge_arpabet + test
// 3. dedup code
// 4. finalize API
// 5. strict compiler flags
// 6. cleanup and release

#[cfg(test)] extern crate chrono;
#[cfg(test)] #[macro_use] extern crate expectest;

#[macro_use] extern crate lazy_static;
extern crate regex;

mod error;

pub use error::ArpabetError;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub type Word = String;
pub type Phoneme = String;
pub type Polyphone = Vec<Phoneme>;

const CMU_DICT_TEXT : &'static str = include_str!("../cmudict/cmudict-0.7b");

lazy_static! {
  static ref CMU_DICT : Arpabet = Arpabet::load_from_str(CMU_DICT_TEXT)
      .expect("CMU dictionary should lazily load.");
}

#[derive(Default)]
pub struct Arpabet {
  /// A map of lowercase words to polyphone breakdown (phones are uppercase).
  /// eg. 'jungle' -> [JH, AH1, NG, G, AH0, L]
  dictionary: HashMap<Word, Polyphone>
}

impl Arpabet {
  /// Create a new instance.
  pub fn new() -> Arpabet {
    Arpabet { dictionary: HashMap::new() }
  }

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

    // Format resembles the following,
    // ABBREVIATE  AH0 B R IY1 V IY0 EY2 T
    let re = Regex::new(r"^([\w\-']+)\s+(.*)\s*$")
        .expect("Regex should be correct.");

    for line in text.lines() {
      match re.captures(&line) {
        None => {},
        Some(caps) => {
          let word_match = caps.get(1);
          let phonemes_match = caps.get(2);

          if word_match.is_some() && phonemes_match.is_some() {
            // FIXME: Error handling
            let word = word_match.unwrap().as_str().to_lowercase();
            let split = phonemes_match.unwrap().as_str().split(" ");
            let v1 = split.collect::<Vec<&str>>();
            let v2 = v1.iter().map(|s| s.to_string()).collect::<Vec<String>>();
            map.insert(word, v2);
          }
        },
      }
    }

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
    let mut buffer = String::new();

    // Format resembles the following,
    // ABBREVIATE  AH0 B R IY1 V IY0 EY2 T
    let re = Regex::new(r"^([\w\-']+)\s+(.*)\s*$")
        .expect("Regex should be correct.");

    while reader.read_line(&mut buffer)? > 0 {
      match re.captures(&buffer) {
        None => {},
        Some(caps) => {
          let word_match = caps.get(1);
          let phonemes_match = caps.get(2);

          if word_match.is_some() && phonemes_match.is_some() {
            // FIXME: Error handling
            let word = word_match.unwrap().as_str().to_lowercase();
            let split = phonemes_match.unwrap().as_str().split(" ");
            let v1 = split.collect::<Vec<&str>>();
            let v2 = v1.iter().map(|s| s.to_string()).collect::<Vec<String>>();
            map.insert(word, v2);
          }
        },
      }
      buffer.clear();
    }

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
  pub fn merge(&mut self, other: &Arpabet) {
    for (k, v) in other.dictionary.iter() {
      self.dictionary.insert(k.clone(), v.clone());
    }
  }

  /// Insert an entry into the Arpabet. If the entry is already present,
  /// replace it and return the old value.
  pub fn insert(&mut self, key: Word, value: Polyphone) -> Option<Polyphone> {
    self.dictionary.insert(key, value)
  }
}

// 1. Core cmudict
// 2. Read in custom dictionar/ies
// 3. Merge / layer dictionaries

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
  fn merge() {
    let mut a = Arpabet::new();
    a.insert("foo".to_string(), to_strings(vec!["F", "UW1"]));
    a.insert("bar".to_string(), to_strings(vec!["B", "A1", "R"]));

    let b = {
      let mut arpa = Arpabet::new();
      arpa.insert("foo".to_string(), to_strings(vec!["B", "OO"]));
      arpa.insert("baz".to_string(), to_strings(vec!["B", "AE1", "Z"]));
      arpa
    };

    a.merge(&b);

    assert_eq!(a.get_polyphone("foo"), Some(to_strings(vec!["B", "OO"])));
    assert_eq!(a.get_polyphone("bar"), Some(to_strings(vec!["B", "A1", "R"])));
    assert_eq!(a.get_polyphone("baz"), Some(to_strings(vec!["B", "AE1", "Z"])));
    assert_eq!(a.get_polyphone("bin"), None);
  }
}
