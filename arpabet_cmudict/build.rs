#[macro_use] extern crate lazy_static;

extern crate arpabet_types;
extern crate arpabet_parser;
extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use phf_codegen::Map;
use arpabet_types::{Polyphone, VowelStress};
use arpabet_types::Consonant;
use arpabet_types::Arpabet;
use arpabet_types::Phoneme;

//pub type Word = String;
//pub type Polyphone = Vec<Phoneme>;

const CMU_DICT_TEXT : &'static str = include_str!("../cmudict/cmudict-0.7b");

lazy_static! {
  static ref CMUDICT : Arpabet = arpabet_parser::load_from_str(CMU_DICT_TEXT)
      .expect("Must parse at compile time");
}

fn main() {
  let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
  let mut file = BufWriter::new(File::create(&path).unwrap());

  write!(&mut file, "use arpabet_types::Consonant;\n").unwrap();
  write!(&mut file, "use arpabet_types::Vowel;\n").unwrap();
  write!(&mut file, "use arpabet_types::VowelStress;\n").unwrap();
  //write!(&mut file, "use arpabet_types::Polyphone;\n").unwrap();

  //write!(&mut file, "static KEYWORDS: phf::Map<&'static str, Keyword> = ").unwrap();
  write!(&mut file, "static CMU_DICT_2: phf::Map<&'static str, &'static [Phoneme]> = ").unwrap();

  let mut builder : Map<&'static str> = phf_codegen::Map::new();

  for key in CMUDICT.keys() {
    let polyphone = CMUDICT.get_polyphone(key).unwrap();

    let mut code = String::from("&[");

    for phone in polyphone.iter() {
      match phone {
        Phoneme::Consonant(consonant) => {
          code.push_str(&format!("Phoneme::Consonant(Consonant::{}), ", consonant.to_str()))
        },
        Phoneme::Vowel(vowel) => {
          let vowel_stress = format!("VowelStress::{}", match vowel.get_stress() {
            VowelStress::UnknownStress => "UnknownStress",
            VowelStress::NoStress => "NoStress",
            VowelStress::PrimaryStress => "PrimaryStress",
            VowelStress::SecondaryStress => "SecondaryStress",
          });
          code.push_str(&format!("Phoneme::Vowel(Vowel::{}({})), ", vowel.to_str_stressless(), vowel_stress))
        },
      }
    }

    code.push_str("]");

    //builder.entry(&key, "&[ Phoneme::Consonant(Consonant::B) ]");
    builder.entry(&key, &code);
    println!("Key: {}", key);

  }

  //let codegen_2 = builder
   // .entry("TEST", "&[ Phoneme::Consonant(Consonant::B) ]");
  //.entry("TEST", "unsafe { vec![] }");
  //.entry("continue", "Keyword::Continue")

  let codegen_map = builder
    .build();
    //.build(&mut file)
    //.unwrap();

  write!(&mut file, "{}", codegen_map.to_string()).unwrap();

  write!(&mut file, ";\n").unwrap();
}
