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

pub const CMU_DICT_TEXT : &'static str = include_str!("../../cmudict/cmudict-0.7b");

/// A word is a simple string containing no space characters.
pub type Word = String;

/// A polyphone is several phonemes read in order, typically as a single word.
pub type Polyphone = Vec<Phoneme>;

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

  fn read_lines(reader: &mut BufRead, map: &mut HashMap<Word, Vec<Phoneme>>)
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

          let phoneme_tokens = match caps.get(2) {
            None => return Err(ArpabetError::InvalidFormat {
              line_number: line_count,
              text: buffer.to_string(),
            }),
            Some(m) => m.as_str()
              .split(" ")
              .map(|s| s.to_string().to_uppercase())
              .collect::<Vec<String>>(),
          };

          if phoneme_tokens.is_empty() {
            return Err(ArpabetError::InvalidFormat {
              line_number: line_count,
              text: buffer.to_string(),
            });
          }

          let mut phonemes = Vec::new();

          for token in phoneme_tokens {
            match PHONEME_MAP.get(token.as_str()) {
              None => {
                return Err(ArpabetError::InvalidFormat {
                  line_number: line_count,
                  text: buffer.to_string(),
                });
              },
              Some(phoneme) => phonemes.push(phoneme.clone()),
            }
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
