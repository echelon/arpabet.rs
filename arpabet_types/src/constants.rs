//! This module contains constant lists and maps of phonemes in given classes.

use crate::extensions::Punctuation;
use phf::phf_map;

use crate::phoneme::{
  Consonant,
  Phoneme,
  Vowel,
  VowelStress,
};

/// An array of all consonants.
pub const ALL_CONSONANTS : [Consonant; 31] = [
  Consonant::B,
  Consonant::CH,
  Consonant::D,
  Consonant::DH,
  Consonant::DX,
  Consonant::EL,
  Consonant::EM,
  Consonant::EN,
  Consonant::F,
  Consonant::G,
  Consonant::HH,
  Consonant::JH,
  Consonant::K,
  Consonant::L,
  Consonant::M,
  Consonant::N,
  Consonant::NG,
  Consonant::NX,
  Consonant::P,
  Consonant::Q,
  Consonant::R,
  Consonant::S,
  Consonant::SH,
  Consonant::T,
  Consonant::TH,
  Consonant::V,
  Consonant::W,
  Consonant::WH,
  Consonant::Y,
  Consonant::Z,
  Consonant::ZH,
];

/// An array of all vowels.
pub const ALL_VOWELS : [Vowel; 76] = [
  Vowel::AA(VowelStress::UnknownStress),
  Vowel::AA(VowelStress::NoStress),
  Vowel::AA(VowelStress::PrimaryStress),
  Vowel::AA(VowelStress::SecondaryStress),
  Vowel::AE(VowelStress::UnknownStress),
  Vowel::AE(VowelStress::NoStress),
  Vowel::AE(VowelStress::PrimaryStress),
  Vowel::AE(VowelStress::SecondaryStress),
  Vowel::AH(VowelStress::UnknownStress),
  Vowel::AH(VowelStress::NoStress),
  Vowel::AH(VowelStress::PrimaryStress),
  Vowel::AH(VowelStress::SecondaryStress),
  Vowel::AO(VowelStress::UnknownStress),
  Vowel::AO(VowelStress::NoStress),
  Vowel::AO(VowelStress::PrimaryStress),
  Vowel::AO(VowelStress::SecondaryStress),
  Vowel::AW(VowelStress::UnknownStress),
  Vowel::AW(VowelStress::NoStress),
  Vowel::AW(VowelStress::PrimaryStress),
  Vowel::AW(VowelStress::SecondaryStress),
  Vowel::AX(VowelStress::UnknownStress),
  Vowel::AX(VowelStress::NoStress),
  Vowel::AX(VowelStress::PrimaryStress),
  Vowel::AX(VowelStress::SecondaryStress),
  Vowel::AXR(VowelStress::UnknownStress),
  Vowel::AXR(VowelStress::NoStress),
  Vowel::AXR(VowelStress::PrimaryStress),
  Vowel::AXR(VowelStress::SecondaryStress),
  Vowel::AY(VowelStress::UnknownStress),
  Vowel::AY(VowelStress::NoStress),
  Vowel::AY(VowelStress::PrimaryStress),
  Vowel::AY(VowelStress::SecondaryStress),
  Vowel::EH(VowelStress::UnknownStress),
  Vowel::EH(VowelStress::NoStress),
  Vowel::EH(VowelStress::PrimaryStress),
  Vowel::EH(VowelStress::SecondaryStress),
  Vowel::ER(VowelStress::UnknownStress),
  Vowel::ER(VowelStress::NoStress),
  Vowel::ER(VowelStress::PrimaryStress),
  Vowel::ER(VowelStress::SecondaryStress),
  Vowel::EY(VowelStress::UnknownStress),
  Vowel::EY(VowelStress::NoStress),
  Vowel::EY(VowelStress::PrimaryStress),
  Vowel::EY(VowelStress::SecondaryStress),
  Vowel::IH(VowelStress::UnknownStress),
  Vowel::IH(VowelStress::NoStress),
  Vowel::IH(VowelStress::PrimaryStress),
  Vowel::IH(VowelStress::SecondaryStress),
  Vowel::IX(VowelStress::UnknownStress),
  Vowel::IX(VowelStress::NoStress),
  Vowel::IX(VowelStress::PrimaryStress),
  Vowel::IX(VowelStress::SecondaryStress),
  Vowel::IY(VowelStress::UnknownStress),
  Vowel::IY(VowelStress::NoStress),
  Vowel::IY(VowelStress::PrimaryStress),
  Vowel::IY(VowelStress::SecondaryStress),
  Vowel::OW(VowelStress::UnknownStress),
  Vowel::OW(VowelStress::NoStress),
  Vowel::OW(VowelStress::PrimaryStress),
  Vowel::OW(VowelStress::SecondaryStress),
  Vowel::OY(VowelStress::UnknownStress),
  Vowel::OY(VowelStress::NoStress),
  Vowel::OY(VowelStress::PrimaryStress),
  Vowel::OY(VowelStress::SecondaryStress),
  Vowel::UH(VowelStress::UnknownStress),
  Vowel::UH(VowelStress::NoStress),
  Vowel::UH(VowelStress::PrimaryStress),
  Vowel::UH(VowelStress::SecondaryStress),
  Vowel::UW(VowelStress::UnknownStress),
  Vowel::UW(VowelStress::NoStress),
  Vowel::UW(VowelStress::PrimaryStress),
  Vowel::UW(VowelStress::SecondaryStress),
  Vowel::UX(VowelStress::UnknownStress),
  Vowel::UX(VowelStress::NoStress),
  Vowel::UX(VowelStress::PrimaryStress),
  Vowel::UX(VowelStress::SecondaryStress),
];

/// An array of all punctuation.
pub const ALL_PUNCTUATION: [Punctuation; 10] = [
  Punctuation::StartToken,
  Punctuation::Space,
  Punctuation::Comma,
  Punctuation::Period,
  Punctuation::Question,
  Punctuation::Exclamation,
  Punctuation::Interjection,
  Punctuation::Quote,
  Punctuation::Ellipsis,
  Punctuation::EndToken,
];

/// A map of strings to consonants.
pub const PHONEME_MAP : phf::Map<&'static str, Phoneme> = phf_map! {
  "B" => Phoneme::Consonant(Consonant::B),
  "CH" => Phoneme::Consonant(Consonant::CH),
  "D" => Phoneme::Consonant(Consonant::D),
  "DH" => Phoneme::Consonant(Consonant::DH),
  "DX" => Phoneme::Consonant(Consonant::DX),
  "EL" => Phoneme::Consonant(Consonant::EL),
  "EM" => Phoneme::Consonant(Consonant::EM),
  "EN" => Phoneme::Consonant(Consonant::EN),
  "F" => Phoneme::Consonant(Consonant::F),
  "G" => Phoneme::Consonant(Consonant::G),
  "HH" => Phoneme::Consonant(Consonant::HH),
  "JH" => Phoneme::Consonant(Consonant::JH),
  "K" => Phoneme::Consonant(Consonant::K),
  "L" => Phoneme::Consonant(Consonant::L),
  "M" => Phoneme::Consonant(Consonant::M),
  "N" => Phoneme::Consonant(Consonant::N),
  "NG" => Phoneme::Consonant(Consonant::NG),
  "NX" => Phoneme::Consonant(Consonant::NX),
  "P" => Phoneme::Consonant(Consonant::P),
  "Q" => Phoneme::Consonant(Consonant::Q),
  "R" => Phoneme::Consonant(Consonant::R),
  "S" => Phoneme::Consonant(Consonant::S),
  "SH" => Phoneme::Consonant(Consonant::SH),
  "T" => Phoneme::Consonant(Consonant::T),
  "TH" => Phoneme::Consonant(Consonant::TH),
  "V" => Phoneme::Consonant(Consonant::V),
  "W" => Phoneme::Consonant(Consonant::W),
  "WH" => Phoneme::Consonant(Consonant::WH),
  "Y" => Phoneme::Consonant(Consonant::Y),
  "Z" => Phoneme::Consonant(Consonant::Z),
  "ZH" => Phoneme::Consonant(Consonant::ZH),
  "AA" => Phoneme::Vowel(Vowel::AA(VowelStress::UnknownStress)),
  "AA0" => Phoneme::Vowel(Vowel::AA(VowelStress::NoStress)),
  "AA1" => Phoneme::Vowel(Vowel::AA(VowelStress::PrimaryStress)),
  "AA2" => Phoneme::Vowel(Vowel::AA(VowelStress::SecondaryStress)),
  "AE" => Phoneme::Vowel(Vowel::AE(VowelStress::UnknownStress)),
  "AE0" => Phoneme::Vowel(Vowel::AE(VowelStress::NoStress)),
  "AE1" => Phoneme::Vowel(Vowel::AE(VowelStress::PrimaryStress)),
  "AE2" => Phoneme::Vowel(Vowel::AE(VowelStress::SecondaryStress)),
  "AH" => Phoneme::Vowel(Vowel::AH(VowelStress::UnknownStress)),
  "AH0" => Phoneme::Vowel(Vowel::AH(VowelStress::NoStress)),
  "AH1" => Phoneme::Vowel(Vowel::AH(VowelStress::PrimaryStress)),
  "AH2" => Phoneme::Vowel(Vowel::AH(VowelStress::SecondaryStress)),
  "AO" => Phoneme::Vowel(Vowel::AO(VowelStress::UnknownStress)),
  "AO0" => Phoneme::Vowel(Vowel::AO(VowelStress::NoStress)),
  "AO1" => Phoneme::Vowel(Vowel::AO(VowelStress::PrimaryStress)),
  "AO2" => Phoneme::Vowel(Vowel::AO(VowelStress::SecondaryStress)),
  "AW" => Phoneme::Vowel(Vowel::AW(VowelStress::UnknownStress)),
  "AW0" => Phoneme::Vowel(Vowel::AW(VowelStress::NoStress)),
  "AW1" => Phoneme::Vowel(Vowel::AW(VowelStress::PrimaryStress)),
  "AW2" => Phoneme::Vowel(Vowel::AW(VowelStress::SecondaryStress)),
  "AX" => Phoneme::Vowel(Vowel::AX(VowelStress::UnknownStress)),
  "AX0" => Phoneme::Vowel(Vowel::AX(VowelStress::NoStress)),
  "AX1" => Phoneme::Vowel(Vowel::AX(VowelStress::PrimaryStress)),
  "AX2" => Phoneme::Vowel(Vowel::AX(VowelStress::SecondaryStress)),
  "AXR" => Phoneme::Vowel(Vowel::AXR(VowelStress::UnknownStress)),
  "AXR0" => Phoneme::Vowel(Vowel::AXR(VowelStress::NoStress)),
  "AXR1" => Phoneme::Vowel(Vowel::AXR(VowelStress::PrimaryStress)),
  "AXR2" => Phoneme::Vowel(Vowel::AXR(VowelStress::SecondaryStress)),
  "AY" => Phoneme::Vowel(Vowel::AY(VowelStress::UnknownStress)),
  "AY0" => Phoneme::Vowel(Vowel::AY(VowelStress::NoStress)),
  "AY1" => Phoneme::Vowel(Vowel::AY(VowelStress::PrimaryStress)),
  "AY2" => Phoneme::Vowel(Vowel::AY(VowelStress::SecondaryStress)),
  "EH" => Phoneme::Vowel(Vowel::EH(VowelStress::UnknownStress)),
  "EH0" => Phoneme::Vowel(Vowel::EH(VowelStress::NoStress)),
  "EH1" => Phoneme::Vowel(Vowel::EH(VowelStress::PrimaryStress)),
  "EH2" => Phoneme::Vowel(Vowel::EH(VowelStress::SecondaryStress)),
  "ER" => Phoneme::Vowel(Vowel::ER(VowelStress::UnknownStress)),
  "ER0" => Phoneme::Vowel(Vowel::ER(VowelStress::NoStress)),
  "ER1" => Phoneme::Vowel(Vowel::ER(VowelStress::PrimaryStress)),
  "ER2" => Phoneme::Vowel(Vowel::ER(VowelStress::SecondaryStress)),
  "EY" => Phoneme::Vowel(Vowel::EY(VowelStress::UnknownStress)),
  "EY0" => Phoneme::Vowel(Vowel::EY(VowelStress::NoStress)),
  "EY1" => Phoneme::Vowel(Vowel::EY(VowelStress::PrimaryStress)),
  "EY2" => Phoneme::Vowel(Vowel::EY(VowelStress::SecondaryStress)),
  "IH" => Phoneme::Vowel(Vowel::IH(VowelStress::UnknownStress)),
  "IH0" => Phoneme::Vowel(Vowel::IH(VowelStress::NoStress)),
  "IH1" => Phoneme::Vowel(Vowel::IH(VowelStress::PrimaryStress)),
  "IH2" => Phoneme::Vowel(Vowel::IH(VowelStress::SecondaryStress)),
  "IX" => Phoneme::Vowel(Vowel::IX(VowelStress::UnknownStress)),
  "IX0" => Phoneme::Vowel(Vowel::IX(VowelStress::NoStress)),
  "IX1" => Phoneme::Vowel(Vowel::IX(VowelStress::PrimaryStress)),
  "IX2" => Phoneme::Vowel(Vowel::IX(VowelStress::SecondaryStress)),
  "IY" => Phoneme::Vowel(Vowel::IY(VowelStress::UnknownStress)),
  "IY0" => Phoneme::Vowel(Vowel::IY(VowelStress::NoStress)),
  "IY1" => Phoneme::Vowel(Vowel::IY(VowelStress::PrimaryStress)),
  "IY2" => Phoneme::Vowel(Vowel::IY(VowelStress::SecondaryStress)),
  "OW" => Phoneme::Vowel(Vowel::OW(VowelStress::UnknownStress)),
  "OW0" => Phoneme::Vowel(Vowel::OW(VowelStress::NoStress)),
  "OW1" => Phoneme::Vowel(Vowel::OW(VowelStress::PrimaryStress)),
  "OW2" => Phoneme::Vowel(Vowel::OW(VowelStress::SecondaryStress)),
  "OY" => Phoneme::Vowel(Vowel::OY(VowelStress::UnknownStress)),
  "OY0" => Phoneme::Vowel(Vowel::OY(VowelStress::NoStress)),
  "OY1" => Phoneme::Vowel(Vowel::OY(VowelStress::PrimaryStress)),
  "OY2" => Phoneme::Vowel(Vowel::OY(VowelStress::SecondaryStress)),
  "UH" => Phoneme::Vowel(Vowel::UH(VowelStress::UnknownStress)),
  "UH0" => Phoneme::Vowel(Vowel::UH(VowelStress::NoStress)),
  "UH1" => Phoneme::Vowel(Vowel::UH(VowelStress::PrimaryStress)),
  "UH2" => Phoneme::Vowel(Vowel::UH(VowelStress::SecondaryStress)),
  "UW" => Phoneme::Vowel(Vowel::UW(VowelStress::UnknownStress)),
  "UW0" => Phoneme::Vowel(Vowel::UW(VowelStress::NoStress)),
  "UW1" => Phoneme::Vowel(Vowel::UW(VowelStress::PrimaryStress)),
  "UW2" => Phoneme::Vowel(Vowel::UW(VowelStress::SecondaryStress)),
  "UX" => Phoneme::Vowel(Vowel::UX(VowelStress::UnknownStress)),
  "UX0" => Phoneme::Vowel(Vowel::UX(VowelStress::NoStress)),
  "UX1" => Phoneme::Vowel(Vowel::UX(VowelStress::PrimaryStress)),
  "UX2" => Phoneme::Vowel(Vowel::UX(VowelStress::SecondaryStress)),
};

#[cfg(test)]
mod tests {
  use expectest::prelude::*;
  use super::*;

  #[test]
  pub fn has_all_consonants() {
    expect!(ALL_CONSONANTS.len()).to(be_eq(31));
  }

  #[test]
  pub fn has_all_vowels() {
    expect!(ALL_VOWELS.len()).to(be_eq(76));
  }

  #[test]
  pub fn has_all_punctuation() {
    expect!(ALL_PUNCTUATION.len()).to(be_eq(10));
  }

  #[test]
  pub fn phoneme_map_has_all_phonemes() {
    expect!(PHONEME_MAP.len()).to(be_eq(107));

    for consonant in ALL_CONSONANTS.iter() {
      let consonant_str = consonant.to_str();
      match PHONEME_MAP.get(consonant_str)
          .expect("should be present") {
        Phoneme::Consonant(c) => expect!(c).to(be_eq(consonant)),
        Phoneme::Vowel(_) => unreachable!(),
      };
    }

    for vowel in ALL_VOWELS.iter() {
      let vowel_str = vowel.to_str();
      match PHONEME_MAP.get(vowel_str)
          .expect("should be present") {
        Phoneme::Consonant(_) => unreachable!(),
        Phoneme::Vowel(v) => expect!(v).to(be_eq(vowel)),
      };
    }
  }
}
