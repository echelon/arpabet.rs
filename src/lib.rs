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

const CMU_DICT : &'static str = include_str!("../cmudict/cmudict-0.7b");
//const CMU_DICT : &'static str = include_str!("../cmudict/test.txt");

pub struct Arpabet {
  /// A map of lowercase words to polyphone breakdown (phones are uppercase).
  /// eg. 'jungle' -> [JH, AH1, NG, G, AH0, L]
  dictionary: HashMap<Word, Polyphone>
}

impl Arpabet {
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
}

// 1. Core cmudict
// 2. Read in custom dictionar/ies
// 3. Merge / layer dictionaries

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
