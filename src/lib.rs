
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
        .expect("should load");
}

pub struct Arpabet {
  /// A map of lowercase words to polyphone breakdown (phones are uppercase).
  /// eg. 'jungle' -> [JH, AH1, NG, G, AH0, L]
  dictionary: HashMap<Word, Polyphone>
}

impl Arpabet {
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
    let re = Regex::new(r"^([\w\-']+)\s+(.*)$")
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
    let re = Regex::new(r"^([\w-']+)\s+(.*)\n$")
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
  pub fn get_polyphone(&self, word: &str) -> Option<Polyphone> {
    self.dictionary.get(word).and_then(|p| {
      Some(p.iter().map(|s| s.to_string()).collect::<Vec<String>>())
    })
  }
}

// 1. Core cmudict
// 2. Read in custom dictionar/ies
// 3. Merge / layer dictionaries

#[cfg(test)]
mod tests {
  use super::*;

  fn to_strings(strs: Vec<&str>) -> Vec<String> {
    strs.iter().map(|s| s.to_string()).collect()
  }

  #[test]
  fn load_from_str() {
    let text = "DOCTOR  D AA1 K T ER0\n\
                MARIO  M AA1 R IY0 OW0";

    let arpabet = Arpabet::load_from_str(text).expect("Should load");

    assert_eq!(arpabet.get_polyphone("super"), None);

    assert_eq!(arpabet.get_polyphone("doctor"),
      Some(to_strings(vec!["D", "AA1", "K", "T","ER0"])));

    assert_eq!(arpabet.get_polyphone("mario"),
      Some(to_strings(vec!["M", "AA1", "R", "IY0","OW0"])));
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
}


