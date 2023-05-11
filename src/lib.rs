use std::collections::HashMap;

use napi_derive::napi;
use once_cell::sync::Lazy;
use pyo3::prelude::PyModule;
use pyo3::{pyclass, pymodule, wrap_pyfunction, PyResult, Python};
use regex::Regex;

pub mod align;
mod frontend;
mod leven;
pub mod phonetics;

pub static COMBINED_PHONEME_MAPPING: Lazy<HashMap<&'static str, [&'static str; 2]>> =
  Lazy::new(|| {
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
    ])
  });
pub static PHONEME_TO_CHAR_MAPPING: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
  HashMap::from([
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
  ])
});
pub static PHONEME_WORD_TO_WORD_MAPPING: Lazy<HashMap<&'static str, &'static str>> =
  Lazy::new(|| {
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
    ])
  });
pub static LETTER_PHONEMES: Lazy<[&'static str; 26]> = Lazy::new(|| {
  [
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
  ]
});
pub static PHONETIC_WORD_TAG_BOUNDARIES: Lazy<[&'static str; 4]> =
  Lazy::new(|| ["<w>", "<s>", "</w>", "</s>"]);
pub static PHONETIC_WORD_BOUNDARY: Lazy<[&'static str; 22]> = Lazy::new(|| {
  [
    "!", "(", ")", "-", ";", ":", ",", ".", "?", "¡", "¿", "—", "…", "'", "«", "»", "“", "”", " ",
    "\n", "</w>", "</s>",
  ]
});
pub static ACRONYM_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(
    format!(
      "^({}){{3,}}s?$",
      LETTER_PHONEMES
        .iter()
        .map(|str| format!("({})", str))
        .collect::<Vec<_>>()
        .join("|")
    )
    .as_str(),
  )
  .unwrap()
});
pub static ACRONYM_REGEX_SPLIT: Lazy<Regex> = Lazy::new(|| {
  Regex::new(
    format!(
      "({})",
      LETTER_PHONEMES
        .iter()
        .map(|str| format!("({})", str))
        .collect::<Vec<_>>()
        .join("|")
    )
    .as_str(),
  )
  .unwrap()
});

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

#[cfg_attr(feature = "napi", napi(object))]
#[pyclass]
#[derive(Clone)]
pub struct NestedChunk {
  pub value: String,
  pub start: f64,
  pub end: f64,
  pub start_time: f64,
  pub end_time: f64,
  pub chunks: Vec<Chunk>,
}

#[cfg_attr(feature = "napi", napi(object))]
#[pyclass]
#[derive(Clone)]
pub struct Chunk {
  pub value: String,
  pub start: f64,
  pub end: f64,
  pub start_time: f64,
  pub end_time: f64,
}

#[cfg_attr(feature = "napi", napi(object))]
#[pyclass]
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
  phoneme_list: &[&str],
  end_times: &[f64],
) -> Vec<PhonemeChunk> {
  let mut words: Vec<PhonemeChunk> = Vec::new();

  let mut i = 0;
  while i < phoneme_list.len() {
    // Loop until we find a char that isn't a phonetic boundary
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
      if value.is_empty() {
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
        start_time: end_times[usize::max(1, start) - 1] * 1000.0,
        end_time: end_times[usize::min(usize::max(1, end), end_times.len()) - 1] * 1000.0,
      });

      start += length;
    }
  }

  words
}

fn is_phonetic_word_boundary(phoneme: &str) -> bool {
  PHONETIC_WORD_BOUNDARY.contains(&phoneme)
}

pub fn phoneme_to_word(phoneme: &str) -> String {
  if ACRONYM_REGEX.is_match(phoneme) {
    let value: String = ACRONYM_REGEX
      .split(phoneme)
      .filter(|str| !str.is_empty())
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
  word
}

/// A Python module implemented in Rust.
#[pymodule]
fn espeak_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
  use frontend::py;
  m.add_function(wrap_pyfunction!(py::phonemize, m)?)?;
  m.add_function(wrap_pyfunction!(
    py::force_align_phonemes_graphemes_list,
    m
  )?)?;
  m.add_function(wrap_pyfunction!(py::force_align_phonemes_graphemes, m)?)?;

  m.add_class::<NestedChunk>()?;
  m.add_class::<Chunk>()?;
  m.add_class::<PhonemeChunk>()?;
  Ok(())
}
