mod phonetics;

extern crate napi;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;
use napi_derive::napi;
use regex::Regex;

lazy_static! {
  pub static ref combinedPhonemeMapping: HashMap<&'static str, (&'static str, &'static str)> =
    HashMap::from([
      ("ðæɾə", ("ðæɾ", "ə")),
      ("fəðə", ("fə", "ðə")),
      ("fɚðə", ("fɚ", "ðə")),
      ("fɚɹə", ("fɚɹ", "ə")),
      ("ɪnðə", ("ɪn", "ðə")),
      ("ɔnðɪ", ("ɔn", "ðɪ")),
      ("dɪdnɑːt", ("dɪd", "nɑːt")),
      ("wɪððə", ("wɪð", "ðə")),
      ("ʌvðə", ("ʌv", "ðə")),
      ("ʌvðɪ", ("ʌv", "ðɪ")),
      ("wʌzðə", ("wʌz", "ðə")),
      ("dʌznɑːt", ("dʌz", "nɑːt")),
      ("aʊɾəv", ("aʊɾ", "əv")),
      ("fɹʌmðə", ("fɹʌm", "ðə")),
      ("ðætwʌn", ("ðæt", "wʌn")),
      ("ðætðɪ", ("ðæt", "ðɪ")),
      ("meɪhɐv", ("meɪ", "hɐv")),
      ("təbi", ("tə", "bi")),
    ]);
  pub static ref phonemeToCharMapping: HashMap<&'static str, &'static str> = HashMap::from([
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
  pub static ref phonemeWordToWordMapping: HashMap<&'static str, &'static str> = HashMap::from([
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
  pub static ref letterPhonemes: [&'static str; 26] = [
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
  pub static ref phoneticPunctuation: [&'static str; 9] =
    ["^", ".", "~", ":", ";", ",", "?", "!", " "];
  pub static ref phoneticWordTagBoundaries: [&'static str; 4] = ["<w>", "<s>", "</w>", "</s>"];
  pub static ref phoneticWordBoundary: [&'static str; 22] = [
    "!", "(", ")", "-", ";", ":", ",", ".", "?", "¡", "¿", "—", "…", "'", "«", "»", "“", "”", " ",
    "\n", "</w>", "</s>",
  ];
  pub static ref acronymRegex: Regex = Regex::new(
    format!(
      "^({}){{3,}}s?$",
      letterPhonemes.map(|str| format!("({})", str)).join("|")
    )
    .as_str(),
  )
  .unwrap();
  pub static ref acronymRegexSplit: Regex = Regex::new(
    format!(
      "({})",
      letterPhonemes.map(|str| format!("({})", str)).join("|")
    )
    .as_str(),
  )
  .unwrap();
}

pub fn text_to_phonemes(text: String) -> String {
  let mut speaker = espeakng::initialise(None).unwrap().lock();
  speaker
    .text_to_phonemes(text.as_str(), espeakng::PhonemeGenOptions::Standard)
    .unwrap()
    .unwrap()
}

#[napi(object)]
pub struct Chunk {
  pub value: String,
  pub valueWord: String,
  pub start: f64,
  pub end: f64,
  pub startTime: f64,
  pub endTime: f64,
}

#[napi]
pub fn transform_raw_phoneme_timestamps(
  phoneme_list: Vec<String>,
  end_times: Vec<f64>,
) -> Vec<Chunk> {
  let start_time = std::time::SystemTime::now();
  let mut words: Vec<Chunk> = Vec::new();

  let mut i = 0;
  while i < phoneme_list.len() {
    while i < phoneme_list.len() && isPhoneticWordBoundary(phoneme_list.get(i).unwrap()) {
      i += 1
    }

    let start = i;
    let mut value = String::new();
    while i < phoneme_list.len() && !isPhoneticWordBoundary(phoneme_list.get(i).unwrap().as_str()) {
      let phoneme = phoneme_list.get(i).unwrap();
      if (!phoneticWordTagBoundaries.contains(&phoneme.as_str())) {
        value.push_str(phoneme.as_str())
      }
      i += 1;
    }
    let end = i;
    let phoneme_word = phoneme_to_word(value.as_str()).trim().to_string();

    words.push(Chunk {
      value,
      valueWord: phoneme_word,
      start: start as f64,
      end: end as f64,
      startTime: (end_times[usize::max(1, start) - 1] as f64) * 1000.0,
      endTime: (end_times[usize::max(1, end) - 1] as f64) * 1000.0,
    });
  }
  let end_time = std::time::SystemTime::now();
  println!("{:?}", end_time.duration_since(start_time).unwrap());

  return words;
}

fn isPhoneticPunctuation(phoneme: &str) -> bool {
  return phoneticPunctuation.contains(&phoneme);
}

fn isPhoneticWordBoundary(phoneme: &str) -> bool {
  return phoneticWordBoundary.contains(&phoneme);
}

fn phoneme_to_word(phoneme: &str) -> String {
  if acronymRegex.is_match(phoneme) {
    let value: String = acronymRegex
      .split(phoneme)
      .filter(|str| str.len() > 0)
      .map(|phoneme| {
        (letterPhonemes
          .iter()
          .position(|letter| (*letter).eq(phoneme))
          .unwrap()
          + 97) as u8 as char
      })
      .collect();
    return value;
  }

  if phonemeWordToWordMapping.contains_key(phoneme) {
    return phonemeWordToWordMapping.get(phoneme).unwrap().to_string();
  }

  let mut word = String::new();
  for phonemeChar in phoneme.chars() {
    word.push_str(
      phonemeToCharMapping
        .get(phonemeChar.to_string().as_str())
        .unwrap_or(&""),
    );
  }
  return word;
}
