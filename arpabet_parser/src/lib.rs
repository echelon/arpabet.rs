#[cfg(test)] #[macro_use] extern crate expectest;
#[macro_use] extern crate lazy_static;

use arpabet_types::{Arpabet, ArpabetError, Word, Phoneme, PHONEME_MAP, Polyphone};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// TODO: When static constexpr are added to Rust, evaluate these at compile time.
lazy_static! {
  // Regex for reading CMU arpabet, or similarly formatted files.
  // Format resembles the following,
  // ABBREVIATE  AH0 B R IY1 V IY0 EY2 T
  static ref FILE_REGEX : Regex = Regex::new(r"^([\w\-\(\)\.']+)\s+([^\s].*)\s*$")
      .expect("Regex is correct.");

  // Comments begin with this preamble.
  static ref COMMENT_REGEX : Regex = Regex::new(r"^;;;\s+")
      .expect("Regex is correct.");
}

/// Load a dictionary from file
/// The file format is expected to match that of
/// [CMUdict](http://www.speech.cs.cmu.edu/cgi-bin/cmudict).
pub fn load_from_file(filename: &str) -> Result<Arpabet, ArpabetError> {
  let f = File::open(filename)?;
  let mut reader = BufReader::new(f);
  let mut map : HashMap<Word, Polyphone> = HashMap::new();

  let _r = read_lines(&mut reader, &mut map)?;

  if map.is_empty() {
    Err(ArpabetError::EmptyFile)
  } else {
    Ok(Arpabet::from_map(map))
  }
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

#[cfg(test)]
mod tests {
  use crate::load_from_file;
  use arpabet_types::{ArpabetError, Arpabet};

  #[test]
  fn test_load_from_file() {
    let arpabet = load_from_file("./tests/file_load_test.txt")
      .expect("File should load");

    assert_eq!(arpabet.get_polyphone_str("pokemon"),
               Some(vec!["P", "OW1", "K", "EY1", "AH0", "N"]));

    assert_eq!(arpabet.get_polyphone_str("pikachu"),
               Some(vec!["P", "IY1", "K", "AH0", "CH", "UW1"]));

    assert_eq!(arpabet.get_polyphone_str("bulbasaur"), None);
  }

  #[test]
  fn test_load_bad_file() {
    let result = load_from_file("./tests/bad_file.txt");

    match result {
      Ok(_) => panic!("Should not be okay!"),
      Err(err) => match err {
        ArpabetError::InvalidFormat { line_number, text } => {
          assert_eq!(line_number, 1);
          assert_eq!(text, "this is not arpabet");
        },
        _ => panic!("Wrong error type!")
      },
    }
  }
}
