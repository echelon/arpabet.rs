//! This module contains a few non-standard tokens and mappings that are not declared in CMUDict.

use phoneme::{
  Consonant,
  Phoneme,
  Vowel,
  VowelStress
};

/// Punctuation devices.
/// These do not belong to Arpabet, but their inclusion can help with ML tasks.
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Punctuation {
  /// Denotes the beginning of an utterance.
  /// In a single clip, this is the start of audio.
  StartToken,
  /// Denotes the space between words or polyphones within a sentence of multiple words.
  Space,
  /// Denotes a comma (,) or shortened breath device within a sentence.
  Comma,
  /// Denotes a period (.) or full stop within a sentence.
  Period,
  /// Denotes a question mark (?) within a sentence.
  Question,
  /// Denotes an exclamation mark (!) within a sentence.
  Exclamation,
  /// Denotes an interjection (dash, parenthetical, etc.) within a sentence.
  Interjection,
  /// Denotes a quote (") within a sentence. Does not disambiguate between a start or end quote.
  Quote,
  /// Denotes an ellipsis (...) within a sentence.
  Ellipsis,
  /// Denotes the end of an utterance.
  /// In a single clip, this is the end of audio.
  EndToken,
}

impl Punctuation {
  /// Represent punctuation tokens as strings.
  pub fn to_str(&self) -> &'static str {
    match self {
      Punctuation::StartToken => "[start]",
      Punctuation::Space => "[space]",
      Punctuation::Comma => "[comma]",
      Punctuation::Period => "[period]",
      Punctuation::Question => "[question]",
      Punctuation::Exclamation => "[exclamation]",
      Punctuation::Interjection => "[interjection]",
      Punctuation::Quote => "[quote]",
      Punctuation::Ellipsis => "[ellipsis]",
      Punctuation::EndToken => "[end]",
    }
  }
}

impl From<Punctuation> for u8 {
  /// Map puncutation tokens to unsigned ints.
  /// These can serve as the numeric inputs into ML models.
  fn from(punctuation: Punctuation) -> Self {
    match punctuation {
      Punctuation::StartToken => 201,
      Punctuation::Space => 202,
      Punctuation::Comma => 203,
      Punctuation::Period => 204,
      Punctuation::Question => 205,
      Punctuation::Exclamation => 206,
      Punctuation::Interjection => 207,
      Punctuation::Quote => 208,
      Punctuation::Ellipsis => 209,
      Punctuation::EndToken => 254, // NB: Especially set to 254.
    }
  }
}

/// A sentence token is any phoneme or punctuation token.
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum SentenceToken {
  /// A phoneme within a sentence. Runs of phonemes constitute polyphones (words).
  Phoneme(Phoneme),
  /// Punctuation within a sentence.
  Punctuation(Punctuation),
}

impl SentenceToken {
  /// Represent sentence tokens as strings.
  pub fn to_str(&self) -> &'static str {
    match self {
      SentenceToken::Phoneme(phoneme) => phoneme.to_str(),
      SentenceToken::Punctuation(punctuation) => punctuation.to_str(),
    }
  }
}

impl From<SentenceToken> for u8 {
  /// Map sentence tokens to unsigned ints.
  /// These can serve as the numeric inputs into ML models.
  fn from(sentence_token: SentenceToken) -> Self {
    match sentence_token {
      SentenceToken::Phoneme(phoneme) => match phoneme {
        Phoneme::Consonant(consonant) => u8::from(consonant),
        Phoneme::Vowel(vowel) => u8::from(vowel),
      },
      SentenceToken::Punctuation(punctuation) => u8::from(punctuation),
    }
  }
}

impl From<Consonant> for u8 {
  /// Map consonants to unsigned ints.
  /// These can serve as the numeric inputs into ML models.
  fn from(consonant: Consonant) -> Self {
    match consonant {
      Consonant::B => 1,
      Consonant::CH => 2,
      Consonant::D => 3,
      Consonant::DH => 4,
      Consonant::DX => 5,
      Consonant::EL => 6,
      Consonant::EM => 7,
      Consonant::EN => 8,
      Consonant::F => 9,
      Consonant::G => 10,
      Consonant::HH => 11,
      Consonant::JH => 12,
      Consonant::K => 13,
      Consonant::L => 14,
      Consonant::M => 15,
      Consonant::N => 16,
      Consonant::NG => 17,
      Consonant::NX => 18,
      Consonant::P => 19,
      Consonant::Q => 20,
      Consonant::R => 21,
      Consonant::S => 22,
      Consonant::SH => 23,
      Consonant::T => 24,
      Consonant::TH => 25,
      Consonant::V => 26,
      Consonant::W => 27,
      Consonant::WH => 28,
      Consonant::Y => 29,
      Consonant::Z => 30,
      Consonant::ZH => 31,
    }
  }
}

impl From<Vowel> for u8 {
  /// Map vowels to unsigned ints.
  /// These can serve as the numeric inputs into ML models.
  fn from(vowel: Vowel) -> Self {
    match vowel {
      Vowel::AA(stress) => match stress {
        VowelStress::UnknownStress => 101,
        VowelStress::NoStress => 102,
        VowelStress::PrimaryStress => 103,
        VowelStress::SecondaryStress => 104,
      },
      Vowel::AE(stress) => match stress {
        VowelStress::UnknownStress => 105,
        VowelStress::NoStress => 106,
        VowelStress::PrimaryStress => 107,
        VowelStress::SecondaryStress => 108,
      },
      Vowel::AH(stress) => match stress {
        VowelStress::UnknownStress => 109,
        VowelStress::NoStress => 110,
        VowelStress::PrimaryStress => 111,
        VowelStress::SecondaryStress => 112,
      },
      Vowel::AO(stress) => match stress {
        VowelStress::UnknownStress => 113,
        VowelStress::NoStress => 114,
        VowelStress::PrimaryStress => 115,
        VowelStress::SecondaryStress => 116,
      },
      Vowel::AW(stress) => match stress {
        VowelStress::UnknownStress => 117,
        VowelStress::NoStress => 118,
        VowelStress::PrimaryStress => 119,
        VowelStress::SecondaryStress => 120,
      },
      Vowel::AX(stress) => match stress {
        VowelStress::UnknownStress => 121,
        VowelStress::NoStress => 122,
        VowelStress::PrimaryStress => 123,
        VowelStress::SecondaryStress => 124,
      },
      Vowel::AXR(stress) => match stress {
        VowelStress::UnknownStress => 125,
        VowelStress::NoStress => 126,
        VowelStress::PrimaryStress => 127,
        VowelStress::SecondaryStress => 128,
      },
      Vowel::AY(stress) => match stress {
        VowelStress::UnknownStress => 129,
        VowelStress::NoStress => 130,
        VowelStress::PrimaryStress => 131,
        VowelStress::SecondaryStress => 132,
      },
      Vowel::EH(stress) => match stress {
        VowelStress::UnknownStress => 133,
        VowelStress::NoStress => 134,
        VowelStress::PrimaryStress => 135,
        VowelStress::SecondaryStress => 136,
      },
      Vowel::ER(stress) => match stress {
        VowelStress::UnknownStress => 137,
        VowelStress::NoStress => 138,
        VowelStress::PrimaryStress => 139,
        VowelStress::SecondaryStress => 140,
      },
      Vowel::EY(stress) => match stress {
        VowelStress::UnknownStress => 141,
        VowelStress::NoStress => 142,
        VowelStress::PrimaryStress => 143,
        VowelStress::SecondaryStress => 144,
      },
      Vowel::IH(stress) => match stress {
        VowelStress::UnknownStress => 145,
        VowelStress::NoStress => 146,
        VowelStress::PrimaryStress => 147,
        VowelStress::SecondaryStress => 148,
      },
      Vowel::IX(stress) => match stress {
        VowelStress::UnknownStress => 149,
        VowelStress::NoStress => 150,
        VowelStress::PrimaryStress => 151,
        VowelStress::SecondaryStress => 152,
      },
      Vowel::IY(stress) => match stress {
        VowelStress::UnknownStress => 153,
        VowelStress::NoStress => 154,
        VowelStress::PrimaryStress => 155,
        VowelStress::SecondaryStress => 156,
      },
      Vowel::OW(stress) => match stress {
        VowelStress::UnknownStress => 157,
        VowelStress::NoStress => 158,
        VowelStress::PrimaryStress => 159,
        VowelStress::SecondaryStress => 160,
      },
      Vowel::OY(stress) => match stress {
        VowelStress::UnknownStress => 161,
        VowelStress::NoStress => 162,
        VowelStress::PrimaryStress => 163,
        VowelStress::SecondaryStress => 164,
      },
      Vowel::UH(stress) => match stress {
        VowelStress::UnknownStress => 165,
        VowelStress::NoStress => 166,
        VowelStress::PrimaryStress => 167,
        VowelStress::SecondaryStress => 168,
      },
      Vowel::UW(stress) => match stress {
        VowelStress::UnknownStress => 169,
        VowelStress::NoStress => 170,
        VowelStress::PrimaryStress => 171,
        VowelStress::SecondaryStress => 172,
      },
      Vowel::UX(stress) => match stress {
        VowelStress::UnknownStress => 173,
        VowelStress::NoStress => 174,
        VowelStress::PrimaryStress => 175,
        VowelStress::SecondaryStress => 176,
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use constants::{ALL_CONSONANTS, ALL_VOWELS};
  use expectest::prelude::*;
  use super::*;

  #[test]
  fn punctuation_to_u8() {
    expect!(u8::from(Punctuation::StartToken)).to(be_eq(201));
    expect!(u8::from(Punctuation::Space)).to(be_eq(202));
    expect!(u8::from(Punctuation::Comma)).to(be_eq(203));
    expect!(u8::from(Punctuation::Period)).to(be_eq(204));
    expect!(u8::from(Punctuation::Question)).to(be_eq(205));
    expect!(u8::from(Punctuation::Exclamation)).to(be_eq(206));
    expect!(u8::from(Punctuation::Interjection)).to(be_eq(207));
    expect!(u8::from(Punctuation::Quote)).to(be_eq(208));
    expect!(u8::from(Punctuation::Ellipsis)).to(be_eq(209));
    expect!(u8::from(Punctuation::EndToken)).to(be_eq(254)); // NB: Exception
  }

  #[test]
  fn punctuation_to_str() {
    expect!(Punctuation::StartToken.to_str()).to(be_eq("[start]"));
    expect!(Punctuation::Space.to_str()).to(be_eq("[space]"));
    expect!(Punctuation::Comma.to_str()).to(be_eq("[comma]"));
    expect!(Punctuation::Period.to_str()).to(be_eq("[period]"));
    expect!(Punctuation::Question.to_str()).to(be_eq("[question]"));
    expect!(Punctuation::Exclamation.to_str()).to(be_eq("[exclamation]"));
    expect!(Punctuation::Interjection.to_str()).to(be_eq("[interjection]"));
    expect!(Punctuation::Quote.to_str()).to(be_eq("[quote]"));
    expect!(Punctuation::Ellipsis.to_str()).to(be_eq("[ellipsis]"));
    expect!(Punctuation::EndToken.to_str()).to(be_eq("[end]"));
  }

  #[test]
  fn consonant_to_u8() {
    for (i, consonant) in ALL_CONSONANTS.iter().enumerate() {
      let expected = i as u8 + 1; // Offset by one.
      expect!(u8::from(*consonant)).to(be_eq(expected));
    }
  }

  #[test]
  fn vowel_to_u8() {
    for (i, vowel) in ALL_VOWELS.iter().enumerate() {
      let expected = i as u8 + 101; // Offset by 101.
      expect!(u8::from(*vowel)).to(be_eq(expected));
    }
  }

  #[test]
  fn sentence_token_to_str() {
    expect!(SentenceToken::Phoneme(Phoneme::Vowel(Vowel::AA(VowelStress::PrimaryStress))).to_str())
        .to(be_eq("AA1"));
    expect!(SentenceToken::Phoneme(Phoneme::Consonant(Consonant::B)).to_str())
        .to(be_eq("B"));
    expect!(SentenceToken::Punctuation(Punctuation::Period).to_str())
        .to(be_eq("[period]"));
  }

  #[test]
  fn sentence_token_to_u8() {
    expect!(u8::from(SentenceToken::Phoneme(Phoneme::Vowel(Vowel::AA(VowelStress::PrimaryStress)))))
        .to(be_eq(103));
    expect!(u8::from(SentenceToken::Phoneme(Phoneme::Consonant(Consonant::B))))
        .to(be_eq(1));
    expect!(u8::from(SentenceToken::Punctuation(Punctuation::Period)))
        .to(be_eq(204));
  }
}
