pub mod align;
mod leven;
pub mod phonetics;

extern crate napi;
use napi_derive::napi;

use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;
use regex::Regex;

lazy_static! {
  pub static ref COMBINED_PHONEME_MAPPING: HashMap<&'static str, [&'static str; 2]> =
    HashMap::from([
      ("ðæɾə", ["ðæɾ", "ə"]),
      ("fɚðə", ["fɚ", "ðə"]),
      ("fɚɹə", ["fɚɹ", "ə"]),
      ("ɪnðə", ["ɪn", "ðə"]),
      ("ɔnðɪ", ["ɔn", "ðɪ"]),
      ("dɪdnɑːt", ["dɪd", "nɑːt"]),
      ("wɪððə", ["wɪð", "ðə"]),
      ("ʌvðə", ["ʌv", "ðə"]),
      ("ʌvðɪ", ["ʌv", "ðɪ"]),
      ("wʌzðə", ["wʌz", "ðə"]),
      ("dʌznɑːt", ["dʌz", "nɑːt"]),
      ("aʊɾəv", ["aʊɾ", "əv"]),
      ("fɹʌmðə", ["fɹʌm", "ðə"]),
      ("ðætwʌn", ["ðæt", "wʌn"]),
      ("ðætðɪ", ["ðæt", "ðɪ"]),
      ("meɪhɐv", ["meɪ", "hɐv"]),
      ("təbi", ["tə", "bi"]),
    ]);
  pub static ref PHONEME_TO_CHAR_MAPPING: HashMap<&'static str, &'static str> = HashMap::from([
    ("^", ""),
    (".", ""),
    ("~", ""),
    (":", ""),
    (";", ""),
    (",", ""),
    ("?", ""),
    ("!", ""),
    ("ʊ", ""),
    ("ː", ""),
    ("t", "t"),
    ("u", "u"),
    ("ɛ", "e"),
    ("k", "c"),
    ("s", "s"),
    ("ə", "e"),
    ("l", "l"),
    ("n", "n"),
    ("w", "w"),
    ("b", "b"),
    ("a", "a"),
    ("ɪ", "i"),
    ("f", "f"),
    ("ɔ", "o"),
    ("ɹ", "r"),
    ("d", "d"),
    ("v", "v"),
    ("p", "p"),
    ("ɚ", "er"),
    ("z", "s"),
    ("j", "j"),
    ("o", "o"),
    ("ð", "th"),
    ("h", "h"),
    ("ɾ", "t"),
    ("i", "e"),
    ("ɑ", "a"),
    ("ɡ", "g"),
    ("ɐ", "a"),
    ("ɜ", "er"),
    ("ʒ", "s"),
    ("ʌ", "o"),
    ("ʃ", "ur"),
    ("ŋ", "ng"),
    ("æ", "a"),
    ("ᵻ", "e"),
    ("e", "e"),
    ("m", "m"),
    ("θ", "th"),
  ]);
  pub static ref PHONEME_WORD_TO_WORD_MAPPING: HashMap<&'static str, &'static str> =
    HashMap::from([
      ("n", "an"),
      ("ə", "the"),
      ("ðɪ", "the"),
      ("juː", "you"),
      ("ʌs", "us"),
      ("t", "to"),
      ("wɹ", "where"),
      ("wɪl", "will"),
      ("tə", "to"),
      ("eɪ", "may"),
      ("meɪ", "may"),
      ("nɑːt", "not"),
      ("ɪl", "will"),
      ("baɪ", "by"),
      ("bɪ", "by"),
      ("æt", "that"),
      ("hæv", "have"),
      ("weɪ", "way"),
    ]);
  pub static ref LETTER_PHONEMES: [&'static str; 26] = [
    "ɐ",
    "bi",
    "si",
    "di",
    "i",
    "ɛf",
    "dʒi",
    "eɪtʃ",
    "aɪ",
    "dʒeɪ",
    "keɪ",
    "ɛl",
    "ɛm",
    "ɛn",
    "oʊ",
    "pi",
    "kju",
    "ɑɹ",
    "ɛs",
    "ti",
    "ju",
    "vi",
    "dʌbəlju",
    "ɛks",
    "waɪ",
    "zi",
  ];
  pub static ref PHONETIC_WORD_TAG_BOUNDARIES: [&'static str; 4] = ["<w>", "<s>", "</w>", "</s>"];
  pub static ref PHONETIC_WORD_BOUNDARY: [&'static str; 22] = [
    "!", "(", ")", "-", ";", ":", ",", ".", "?", "¡", "¿", "—", "…", "'", "«", "»", "“", "”", " ",
    "\n", "</w>", "</s>",
  ];
  pub static ref ACRONYM_REGEX: Regex = Regex::new(
    format!(
      "^({}){{3,}}s?$",
      LETTER_PHONEMES.map(|str| format!("({})", str)).join("|")
    )
    .as_str(),
  )
  .unwrap();
  pub static ref ACRONYM_REGEX_SPLIT: Regex = Regex::new(
    format!(
      "({})",
      LETTER_PHONEMES.map(|str| format!("({})", str)).join("|")
    )
    .as_str(),
  )
  .unwrap();
}

pub fn text_to_phonemes(text: &str) -> String {
  let mut speaker = espeakng::initialise(Some("en-us")).unwrap().lock();

  text
    .split_inclusive([',', '.', '?', '!'])
    .map(|text| {
      speaker
        .text_to_phonemes(text, espeakng::PhonemeGenOptions::Standard)
        .unwrap()
        .unwrap()
    })
    .collect::<Vec<_>>()
    .join(" ")
}

#[napi(object)]
#[derive(Clone)]
pub struct NestedChunk {
  pub value: String,
  pub start: f64,
  pub end: f64,
  pub start_time: f64,
  pub end_time: f64,
  pub chunks: Vec<Chunk>,
}

#[napi(object)]
#[derive(Clone)]
pub struct Chunk {
  pub value: String,
  pub start: f64,
  pub end: f64,
  pub start_time: f64,
  pub end_time: f64,
}

#[napi(object)]
#[derive(Clone)]
pub struct PhonemeChunk {
  pub value: String,
  pub value_word: String,
  pub start: f64,
  pub end: f64,
  pub start_time: f64,
  pub end_time: f64,
}

pub fn transform_raw_phoneme_timestamps(
  phoneme_list: &Vec<&str>,
  end_times: &Vec<f64>,
) -> Vec<PhonemeChunk> {
  let mut words: Vec<PhonemeChunk> = Vec::new();

  let mut i = 0;
  while i < phoneme_list.len() {
    // Loop until we find a char that isnt a phonetic boundary
    while i < phoneme_list.len() && is_phonetic_word_boundary(phoneme_list.get(i).unwrap()) {
      i += 1;
    }

    // Gather chars until we reach another boundary or end of list
    let start = i;
    let mut value = String::new();
    while i < phoneme_list.len() && !is_phonetic_word_boundary(phoneme_list.get(i).unwrap()) {
      let phoneme = phoneme_list.get(i).unwrap();
      if !PHONETIC_WORD_TAG_BOUNDARIES.contains(phoneme) {
        value.push_str(phoneme);
      }
      i += 1;
    }

    // Split apart known combined phonemes
    let values = if COMBINED_PHONEME_MAPPING.contains_key(value.as_str()) {
      let values = COMBINED_PHONEME_MAPPING.get(value.as_str()).unwrap();
      values.iter().map(|str| str.to_string()).collect()
    } else {
      Vec::from([value])
    };

    // Create chunks for phonemes
    let mut start = start;
    for value in values {
      if value.len() == 0 {
        continue;
      }
      let length = value.chars().count();
      let end = start + length;

      let phoneme_word = phoneme_to_word(value.as_str()).trim().to_string();

      words.push(PhonemeChunk {
        value,
        value_word: phoneme_word,
        start: usize::min(start, phoneme_list.len()) as f64,
        end: usize::min(end, phoneme_list.len()) as f64,
        start_time: (end_times[usize::max(1, start) - 1] as f64) * 1000.0,
        end_time: (end_times[usize::min(usize::max(1, end), end_times.len()) - 1] as f64) * 1000.0,
      });

      start += length;
    }
  }

  return words;
}

fn is_phonetic_word_boundary(phoneme: &str) -> bool {
  return PHONETIC_WORD_BOUNDARY.contains(&phoneme);
}

pub fn phoneme_to_word(phoneme: &str) -> String {
  if ACRONYM_REGEX.is_match(phoneme) {
    let value: String = ACRONYM_REGEX
      .split(phoneme)
      .filter(|str| str.len() > 0)
      .map(|phoneme| {
        (LETTER_PHONEMES
          .iter()
          .position(|letter| (*letter).eq(phoneme))
          .unwrap()
          + 97) as u8 as char
      })
      .collect();
    return value;
  }

  if PHONEME_WORD_TO_WORD_MAPPING.contains_key(phoneme) {
    return PHONEME_WORD_TO_WORD_MAPPING
      .get(phoneme)
      .unwrap()
      .to_string();
  }

  let mut word = String::new();
  for phoneme_char in phoneme.chars() {
    word.push_str(
      PHONEME_TO_CHAR_MAPPING
        .get(phoneme_char.to_string().as_str())
        .unwrap_or(&""),
    );
  }
  return word;
}