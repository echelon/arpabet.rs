//! This module defines the various phonemes in the ARPABET.

use crate::constants::PHONEME_MAP;
use crate::error::ArpabetError;
use std::convert::TryFrom;

/// Consonants in ARPABET.
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Consonant {
  /// B, as in _b_uy.
  B,
  /// CH, as in _Ch_ina.
  CH,
  /// D, as in _d_ie.
  D,
  /// DH, as in _th_y.
  DH,
  /// DX, as in bu_tt_er.
  DX,
  /// EL, as in bott_le_.
  EL,
  /// EM, as in rhyth_m_.
  EM,
  /// EN, as in butt_on_.
  EN,
  /// F, as in _f_ight.
  F,
  /// G, as in _g_uy.
  G,
  /// HH, as in _h_igh.
  HH,
  /// JH, as in _j_ive.
  JH,
  /// K, as in _k_ite.
  K,
  /// L, as in _l_ie.
  L,
  /// M, as in _m_y.
  M,
  /// N, as in _n_igh.
  N,
  /// NG, as in si_ng_.
  NG,
  /// NX, as in wi_nn_er.
  NX,
  /// P, as in _p_ie.
  P,
  /// Q, the glottal stop, as in _uh-oh_.
  Q,
  /// R, as in _r_ye.
  R,
  /// S, as in _s_igh.
  S,
  /// SH, as in _sh_y.
  SH,
  /// T, as in _t_ie.
  T,
  /// TH, as in _th_igh.
  TH,
  /// V, as in _v_ie.
  V,
  /// W, as in _w_ise.
  W,
  /// WH, as in _wh_y.
  WH,
  /// Y, as in _y_acht.
  Y,
  /// Z, as in _z_oo.
  Z,
  /// ZH, as in plea_s_ure.
  ZH,
}

impl Consonant {
  /// Represent a consonant as a string.
  pub fn to_str(&self) -> &'static str {
    match self {
      Consonant::B => "B",
      Consonant::CH => "CH",
      Consonant::D => "D",
      Consonant::DH => "DH",
      Consonant::DX => "DX",
      Consonant::EL => "EL",
      Consonant::EM => "EM",
      Consonant::EN => "EN",
      Consonant::F => "F",
      Consonant::G => "G",
      Consonant::HH => "HH",
      Consonant::JH => "JH",
      Consonant::K => "K",
      Consonant::L => "L",
      Consonant::M => "M",
      Consonant::N => "N",
      Consonant::NG => "NG",
      Consonant::NX => "NX",
      Consonant::P => "P",
      Consonant::Q => "Q",
      Consonant::R => "R",
      Consonant::S => "S",
      Consonant::SH => "SH",
      Consonant::T => "T",
      Consonant::TH => "TH",
      Consonant::V => "V",
      Consonant::W => "W",
      Consonant::WH => "WH",
      Consonant::Y => "Y",
      Consonant::Z => "Z",
      Consonant::ZH => "ZH",
    }
  }
}

/// A stress value for a single vowel.
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum VowelStress {
  /// An unknown amount of stress, perhaps due to omission.
  /// This is not an included value in ARPABET but is added for the sake of parsing.
  UnknownStress,
  /// Denotes a vowel with no stress.
  NoStress,
  /// Denotes a vowel with the primary stress.
  PrimaryStress,
  /// Denotes a vowel with secondary stress.
  SecondaryStress,
}

impl VowelStress {
  /// Get the numeric value for vowel stress. (Unknown stress is -1.)
  pub fn to_i(&self) -> i8 {
    match self {
      VowelStress::UnknownStress => -1,
      VowelStress::NoStress => 0,
      VowelStress::PrimaryStress => 1,
      VowelStress::SecondaryStress => 2,
    }
  }
}

/// Vowels in ARPABET.
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Vowel {
  /// AA, as in b_al_m or b_o_t.
  AA(VowelStress),
  /// AE, as in b_a_t.
  AE(VowelStress),
  /// AH, as in b_u_tt.
  AH(VowelStress),
  /// AO, as in st_o_ry.
  AO(VowelStress),
  /// AW, as in b_ou_t.
  AW(VowelStress),
  /// AX, as in comm_a_.
  AX(VowelStress),
  /// AXR, as in lett_er_.
  AXR(VowelStress),
  /// AY, as in b_i_te.
  AY(VowelStress),
  /// EH, as in b_e_t.
  EH(VowelStress),
  /// ER, as in b_i_rd.
  ER(VowelStress),
  /// EY, as in b_ai_t.
  EY(VowelStress),
  /// IH, as in b_i_t.
  IH(VowelStress),
  /// IX, as in ros_e_s or rabb_i_t.
  IX(VowelStress),
  /// IY, as in b_ea_t.
  IY(VowelStress),
  /// OW, as in b_oa_t.
  OW(VowelStress),
  /// OY, as in b_oy_.
  OY(VowelStress),
  /// UH, as in b_oo_k.
  UH(VowelStress),
  /// UW, as in b_oo_t.
  UW(VowelStress),
  /// UX, as in d_u_de.
  UX(VowelStress),
}

impl Vowel {
  /// Get the stress level of the vowel.
  pub fn get_stress(&self) -> &VowelStress {
    match self {
      Vowel::AA(stress) => stress,
      Vowel::AE(stress) => stress,
      Vowel::AH(stress) => stress,
      Vowel::AO(stress) => stress,
      Vowel::AW(stress) => stress,
      Vowel::AX(stress) => stress,
      Vowel::AXR(stress) => stress,
      Vowel::AY(stress) => stress,
      Vowel::EH(stress) => stress,
      Vowel::ER(stress) => stress,
      Vowel::EY(stress) => stress,
      Vowel::IH(stress) => stress,
      Vowel::IX(stress) => stress,
      Vowel::IY(stress) => stress,
      Vowel::OW(stress) => stress,
      Vowel::OY(stress) => stress,
      Vowel::UH(stress) => stress,
      Vowel::UW(stress) => stress,
      Vowel::UX(stress) => stress,
    }
  }

  /// Get the string representation of the vowel phoneme, without the stress.
  pub fn to_str_stressless(&self) -> &'static str {
    match self {
      Vowel::AA(_) => "AA",
      Vowel::AE(_) => "AE",
      Vowel::AH(_) => "AH",
      Vowel::AO(_) => "AO",
      Vowel::AW(_) => "AW",
      Vowel::AX(_) => "AX",
      Vowel::AXR(_) => "AXR",
      Vowel::AY(_) => "AY",
      Vowel::EH(_) => "EH",
      Vowel::ER(_) => "ER",
      Vowel::EY(_) => "EY",
      Vowel::IH(_) => "IH",
      Vowel::IX(_) => "IX",
      Vowel::IY(_) => "IY",
      Vowel::OW(_) => "OW",
      Vowel::OY(_) => "OY",
      Vowel::UH(_) => "UH",
      Vowel::UW(_) => "UW",
      Vowel::UX(_) => "UX",
    }
  }

  /// Get the string representation of the vowel phoneme.
  pub fn to_str(&self) -> &'static str {
    match self {
      Vowel::AA(stress) => match stress {
        VowelStress::UnknownStress => "AA",
        VowelStress::NoStress => "AA0",
        VowelStress::PrimaryStress => "AA1",
        VowelStress::SecondaryStress => "AA2",
      },
      Vowel::AE(stress) => match stress {
        VowelStress::UnknownStress => "AE",
        VowelStress::NoStress => "AE0",
        VowelStress::PrimaryStress => "AE1",
        VowelStress::SecondaryStress => "AE2",
      },
      Vowel::AH(stress) => match stress {
        VowelStress::UnknownStress => "AH",
        VowelStress::NoStress => "AH0",
        VowelStress::PrimaryStress => "AH1",
        VowelStress::SecondaryStress => "AH2",
      },
      Vowel::AO(stress) => match stress {
        VowelStress::UnknownStress => "AO",
        VowelStress::NoStress => "AO0",
        VowelStress::PrimaryStress => "AO1",
        VowelStress::SecondaryStress => "AO2",
      },
      Vowel::AW(stress) => match stress {
        VowelStress::UnknownStress => "AW",
        VowelStress::NoStress => "AW0",
        VowelStress::PrimaryStress => "AW1",
        VowelStress::SecondaryStress => "AW2",
      },
      Vowel::AX(stress) => match stress {
        VowelStress::UnknownStress => "AX",
        VowelStress::NoStress => "AX0",
        VowelStress::PrimaryStress => "AX1",
        VowelStress::SecondaryStress => "AX2",
      },
      Vowel::AXR(stress) => match stress {
        VowelStress::UnknownStress => "AXR",
        VowelStress::NoStress => "AXR0",
        VowelStress::PrimaryStress => "AXR1",
        VowelStress::SecondaryStress => "AXR2",
      },
      Vowel::AY(stress) => match stress {
        VowelStress::UnknownStress => "AY",
        VowelStress::NoStress => "AY0",
        VowelStress::PrimaryStress => "AY1",
        VowelStress::SecondaryStress => "AY2",
      },
      Vowel::EH(stress) => match stress {
        VowelStress::UnknownStress => "EH",
        VowelStress::NoStress => "EH0",
        VowelStress::PrimaryStress => "EH1",
        VowelStress::SecondaryStress => "EH2",
      },
      Vowel::ER(stress) => match stress {
        VowelStress::UnknownStress => "ER",
        VowelStress::NoStress => "ER0",
        VowelStress::PrimaryStress => "ER1",
        VowelStress::SecondaryStress => "ER2",
      },
      Vowel::EY(stress) => match stress {
        VowelStress::UnknownStress => "EY",
        VowelStress::NoStress => "EY0",
        VowelStress::PrimaryStress => "EY1",
        VowelStress::SecondaryStress => "EY2",
      },
      Vowel::IH(stress) => match stress {
        VowelStress::UnknownStress => "IH",
        VowelStress::NoStress => "IH0",
        VowelStress::PrimaryStress => "IH1",
        VowelStress::SecondaryStress => "IH2",
      },
      Vowel::IX(stress) => match stress {
        VowelStress::UnknownStress => "IX",
        VowelStress::NoStress => "IX0",
        VowelStress::PrimaryStress => "IX1",
        VowelStress::SecondaryStress => "IX2",
      },
      Vowel::IY(stress) => match stress {
        VowelStress::UnknownStress => "IY",
        VowelStress::NoStress => "IY0",
        VowelStress::PrimaryStress => "IY1",
        VowelStress::SecondaryStress => "IY2",
      },
      Vowel::OW(stress) => match stress {
        VowelStress::UnknownStress => "OW",
        VowelStress::NoStress => "OW0",
        VowelStress::PrimaryStress => "OW1",
        VowelStress::SecondaryStress => "OW2",
      },
      Vowel::OY(stress) => match stress {
        VowelStress::UnknownStress => "OY",
        VowelStress::NoStress => "OY0",
        VowelStress::PrimaryStress => "OY1",
        VowelStress::SecondaryStress => "OY2",
      },
      Vowel::UH(stress) => match stress {
        VowelStress::UnknownStress => "UH",
        VowelStress::NoStress => "UH0",
        VowelStress::PrimaryStress => "UH1",
        VowelStress::SecondaryStress => "UH2",
      },
      Vowel::UW(stress) => match stress {
        VowelStress::UnknownStress => "UW",
        VowelStress::NoStress => "UW0",
        VowelStress::PrimaryStress => "UW1",
        VowelStress::SecondaryStress => "UW2",
      },
      Vowel::UX(stress) => match stress {
        VowelStress::UnknownStress => "UX",
        VowelStress::NoStress => "UX0",
        VowelStress::PrimaryStress => "UX1",
        VowelStress::SecondaryStress => "UX2",
      },
    }
  }
}

/// All of the phonemes in ARPABET.
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Phoneme {
  /// ARPABET consonants
  Consonant(Consonant),
  /// ARPABET vowels
  Vowel(Vowel),
}

impl Phoneme {
  /// Get the string representation for a phoneme.
  pub fn to_str(&self) -> &'static str {
    match self {
      Phoneme::Consonant(consonant) => consonant.to_str(),
      Phoneme::Vowel(vowel) => vowel.to_str(),
    }
  }
}

impl TryFrom<&str> for Phoneme {
  type Error = ArpabetError;

  /// Attempt to parse a string-encoded phoneme into a strongly typed Phoneme.
  fn try_from(maybe_phoneme: &str) -> Result<Self, Self::Error> {
    PHONEME_MAP.get(maybe_phoneme)
        .map(|p| p.clone())
        .ok_or(ArpabetError::StringParseError {
          description: format!("Not a phoneme: '{}'", maybe_phoneme)
        })
  }
}

#[cfg(test)]
mod tests {
  use crate::constants::ALL_VOWELS;
  use expectest::prelude::*;
  use super::*;

  #[test]
  fn consonant_to_str() {
    expect!(Consonant::B.to_str()).to(be_eq("B"));
    expect!(Consonant::CH.to_str()).to(be_eq("CH"));
    expect!(Consonant::D.to_str()).to(be_eq("D"));
    expect!(Consonant::DH.to_str()).to(be_eq("DH"));
    expect!(Consonant::DX.to_str()).to(be_eq("DX"));
    expect!(Consonant::EL.to_str()).to(be_eq("EL"));
    expect!(Consonant::EM.to_str()).to(be_eq("EM"));
    expect!(Consonant::EN.to_str()).to(be_eq("EN"));
    expect!(Consonant::F.to_str()).to(be_eq("F"));
    expect!(Consonant::G.to_str()).to(be_eq("G"));
    expect!(Consonant::HH.to_str()).to(be_eq("HH"));
    expect!(Consonant::JH.to_str()).to(be_eq("JH"));
    expect!(Consonant::K.to_str()).to(be_eq("K"));
    expect!(Consonant::L.to_str()).to(be_eq("L"));
    expect!(Consonant::M.to_str()).to(be_eq("M"));
    expect!(Consonant::N.to_str()).to(be_eq("N"));
    expect!(Consonant::NG.to_str()).to(be_eq("NG"));
    expect!(Consonant::NX.to_str()).to(be_eq("NX"));
    expect!(Consonant::P.to_str()).to(be_eq("P"));
    expect!(Consonant::Q.to_str()).to(be_eq("Q"));
    expect!(Consonant::R.to_str()).to(be_eq("R"));
    expect!(Consonant::S.to_str()).to(be_eq("S"));
    expect!(Consonant::SH.to_str()).to(be_eq("SH"));
    expect!(Consonant::T.to_str()).to(be_eq("T"));
    expect!(Consonant::TH.to_str()).to(be_eq("TH"));
    expect!(Consonant::V.to_str()).to(be_eq("V"));
    expect!(Consonant::W.to_str()).to(be_eq("W"));
    expect!(Consonant::WH.to_str()).to(be_eq("WH"));
    expect!(Consonant::Y.to_str()).to(be_eq("Y"));
    expect!(Consonant::Z.to_str()).to(be_eq("Z"));
    expect!(Consonant::ZH.to_str()).to(be_eq("ZH"));
  }

  #[test]
  fn vowel_to_str() {
    expect!(Vowel::AA(VowelStress::UnknownStress).to_str()).to(be_eq("AA"));
    expect!(Vowel::AA(VowelStress::NoStress).to_str()).to(be_eq("AA0"));
    expect!(Vowel::AA(VowelStress::PrimaryStress).to_str()).to(be_eq("AA1"));
    expect!(Vowel::AA(VowelStress::SecondaryStress).to_str()).to(be_eq("AA2"));
    expect!(Vowel::AE(VowelStress::UnknownStress).to_str()).to(be_eq("AE"));
    expect!(Vowel::AE(VowelStress::NoStress).to_str()).to(be_eq("AE0"));
    expect!(Vowel::AE(VowelStress::PrimaryStress).to_str()).to(be_eq("AE1"));
    expect!(Vowel::AE(VowelStress::SecondaryStress).to_str()).to(be_eq("AE2"));
    expect!(Vowel::AH(VowelStress::UnknownStress).to_str()).to(be_eq("AH"));
    expect!(Vowel::AH(VowelStress::NoStress).to_str()).to(be_eq("AH0"));
    expect!(Vowel::AH(VowelStress::PrimaryStress).to_str()).to(be_eq("AH1"));
    expect!(Vowel::AH(VowelStress::SecondaryStress).to_str()).to(be_eq("AH2"));
    expect!(Vowel::AO(VowelStress::UnknownStress).to_str()).to(be_eq("AO"));
    expect!(Vowel::AO(VowelStress::NoStress).to_str()).to(be_eq("AO0"));
    expect!(Vowel::AO(VowelStress::PrimaryStress).to_str()).to(be_eq("AO1"));
    expect!(Vowel::AO(VowelStress::SecondaryStress).to_str()).to(be_eq("AO2"));
    expect!(Vowel::AW(VowelStress::UnknownStress).to_str()).to(be_eq("AW"));
    expect!(Vowel::AW(VowelStress::NoStress).to_str()).to(be_eq("AW0"));
    expect!(Vowel::AW(VowelStress::PrimaryStress).to_str()).to(be_eq("AW1"));
    expect!(Vowel::AW(VowelStress::SecondaryStress).to_str()).to(be_eq("AW2"));
    expect!(Vowel::AX(VowelStress::UnknownStress).to_str()).to(be_eq("AX"));
    expect!(Vowel::AX(VowelStress::NoStress).to_str()).to(be_eq("AX0"));
    expect!(Vowel::AX(VowelStress::PrimaryStress).to_str()).to(be_eq("AX1"));
    expect!(Vowel::AX(VowelStress::SecondaryStress).to_str()).to(be_eq("AX2"));
    expect!(Vowel::AXR(VowelStress::UnknownStress).to_str()).to(be_eq("AXR"));
    expect!(Vowel::AXR(VowelStress::NoStress).to_str()).to(be_eq("AXR0"));
    expect!(Vowel::AXR(VowelStress::PrimaryStress).to_str()).to(be_eq("AXR1"));
    expect!(Vowel::AXR(VowelStress::SecondaryStress).to_str()).to(be_eq("AXR2"));
    expect!(Vowel::AY(VowelStress::UnknownStress).to_str()).to(be_eq("AY"));
    expect!(Vowel::AY(VowelStress::NoStress).to_str()).to(be_eq("AY0"));
    expect!(Vowel::AY(VowelStress::PrimaryStress).to_str()).to(be_eq("AY1"));
    expect!(Vowel::AY(VowelStress::SecondaryStress).to_str()).to(be_eq("AY2"));
    expect!(Vowel::EH(VowelStress::UnknownStress).to_str()).to(be_eq("EH"));
    expect!(Vowel::EH(VowelStress::NoStress).to_str()).to(be_eq("EH0"));
    expect!(Vowel::EH(VowelStress::PrimaryStress).to_str()).to(be_eq("EH1"));
    expect!(Vowel::EH(VowelStress::SecondaryStress).to_str()).to(be_eq("EH2"));
    expect!(Vowel::ER(VowelStress::UnknownStress).to_str()).to(be_eq("ER"));
    expect!(Vowel::ER(VowelStress::NoStress).to_str()).to(be_eq("ER0"));
    expect!(Vowel::ER(VowelStress::PrimaryStress).to_str()).to(be_eq("ER1"));
    expect!(Vowel::ER(VowelStress::SecondaryStress).to_str()).to(be_eq("ER2"));
    expect!(Vowel::EY(VowelStress::UnknownStress).to_str()).to(be_eq("EY"));
    expect!(Vowel::EY(VowelStress::NoStress).to_str()).to(be_eq("EY0"));
    expect!(Vowel::EY(VowelStress::PrimaryStress).to_str()).to(be_eq("EY1"));
    expect!(Vowel::EY(VowelStress::SecondaryStress).to_str()).to(be_eq("EY2"));
    expect!(Vowel::IH(VowelStress::UnknownStress).to_str()).to(be_eq("IH"));
    expect!(Vowel::IH(VowelStress::NoStress).to_str()).to(be_eq("IH0"));
    expect!(Vowel::IH(VowelStress::PrimaryStress).to_str()).to(be_eq("IH1"));
    expect!(Vowel::IH(VowelStress::SecondaryStress).to_str()).to(be_eq("IH2"));
    expect!(Vowel::IX(VowelStress::UnknownStress).to_str()).to(be_eq("IX"));
    expect!(Vowel::IX(VowelStress::NoStress).to_str()).to(be_eq("IX0"));
    expect!(Vowel::IX(VowelStress::PrimaryStress).to_str()).to(be_eq("IX1"));
    expect!(Vowel::IX(VowelStress::SecondaryStress).to_str()).to(be_eq("IX2"));
    expect!(Vowel::IY(VowelStress::UnknownStress).to_str()).to(be_eq("IY"));
    expect!(Vowel::IY(VowelStress::NoStress).to_str()).to(be_eq("IY0"));
    expect!(Vowel::IY(VowelStress::PrimaryStress).to_str()).to(be_eq("IY1"));
    expect!(Vowel::IY(VowelStress::SecondaryStress).to_str()).to(be_eq("IY2"));
    expect!(Vowel::OW(VowelStress::UnknownStress).to_str()).to(be_eq("OW"));
    expect!(Vowel::OW(VowelStress::NoStress).to_str()).to(be_eq("OW0"));
    expect!(Vowel::OW(VowelStress::PrimaryStress).to_str()).to(be_eq("OW1"));
    expect!(Vowel::OW(VowelStress::SecondaryStress).to_str()).to(be_eq("OW2"));
    expect!(Vowel::OY(VowelStress::UnknownStress).to_str()).to(be_eq("OY"));
    expect!(Vowel::OY(VowelStress::NoStress).to_str()).to(be_eq("OY0"));
    expect!(Vowel::OY(VowelStress::PrimaryStress).to_str()).to(be_eq("OY1"));
    expect!(Vowel::OY(VowelStress::SecondaryStress).to_str()).to(be_eq("OY2"));
    expect!(Vowel::UH(VowelStress::UnknownStress).to_str()).to(be_eq("UH"));
    expect!(Vowel::UH(VowelStress::NoStress).to_str()).to(be_eq("UH0"));
    expect!(Vowel::UH(VowelStress::PrimaryStress).to_str()).to(be_eq("UH1"));
    expect!(Vowel::UH(VowelStress::SecondaryStress).to_str()).to(be_eq("UH2"));
    expect!(Vowel::UW(VowelStress::UnknownStress).to_str()).to(be_eq("UW"));
    expect!(Vowel::UW(VowelStress::NoStress).to_str()).to(be_eq("UW0"));
    expect!(Vowel::UW(VowelStress::PrimaryStress).to_str()).to(be_eq("UW1"));
    expect!(Vowel::UW(VowelStress::SecondaryStress).to_str()).to(be_eq("UW2"));
    expect!(Vowel::UX(VowelStress::UnknownStress).to_str()).to(be_eq("UX"));
    expect!(Vowel::UX(VowelStress::NoStress).to_str()).to(be_eq("UX0"));
    expect!(Vowel::UX(VowelStress::PrimaryStress).to_str()).to(be_eq("UX1"));
    expect!(Vowel::UX(VowelStress::SecondaryStress).to_str()).to(be_eq("UX2"));
  }

  #[test]
  fn vowel_get_stress() {
    for (i, vowel) in ALL_VOWELS.iter().enumerate() {
      match i % 4 {
        0 => expect!(vowel.get_stress()).to(be_eq(&VowelStress::UnknownStress)),
        1 => expect!(vowel.get_stress()).to(be_eq(&VowelStress::NoStress)),
        2 => {
          expect!(vowel.get_stress()).to(be_eq(&VowelStress::PrimaryStress));
          expect!(vowel.to_str().ends_with("1")).to(be_true())
        },
        3 => {
          expect!(vowel.get_stress()).to(be_eq(&VowelStress::SecondaryStress));
          expect!(vowel.to_str().ends_with("2")).to(be_true())
        },
        _ => unreachable!(),
      };
    }
  }

  #[test]
  fn phoneme_to_str() {
    expect!(Phoneme::Vowel(Vowel::AA(VowelStress::PrimaryStress)).to_str()).to(be_eq("AA1"));
    expect!(Phoneme::Consonant(Consonant::B).to_str()).to(be_eq("B"));
  }
}
