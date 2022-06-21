use crate::phonetics::punctuation::{extract_punctuation, restore_punctuations};
use crate::text_to_phonemes;
use regex::Regex;
use napi_derive::napi;

const PHONEME_SET: [&str; 126] = [
  " ", "!", "'", "(", ")", ",", "-", ".", ":", ";", "?", "a", "b", "c", "d", "e", "f", "h", "i",
  "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "æ", "ç",
  "ð", "ø", "ħ", "ŋ", "œ", "ǀ", "ǁ", "ǂ", "ǃ", "ɐ", "ɑ", "ɒ", "ɓ", "ɔ", "ɕ", "ɖ", "ɗ", "ɘ", "ə",
  "ɚ", "ɛ", "ɜ", "ɞ", "ɟ", "ɠ", "ɡ", "ɢ", "ɣ", "ɤ", "ɥ", "ɦ", "ɧ", "ɨ", "ɪ", "ɫ", "ɬ", "ɭ", "ɮ",
  "ɯ", "ɰ", "ɱ", "ɲ", "ɳ", "ɴ", "ɵ", "ɶ", "ɸ", "ɹ", "ɺ", "ɻ", "ɽ", "ɾ", "ʀ", "ʁ", "ʂ", "ʃ", "ʄ",
  "ʈ", "ʉ", "ʊ", "ʋ", "ʌ", "ʍ", "ʎ", "ʏ", "ʐ", "ʑ", "ʒ", "ʔ", "ʕ", "ʘ", "ʙ", "ʛ", "ʜ", "ʝ", "ʟ",
  "ʡ", "ʢ", "ˈ", "ˌ", "ː", "ˑ", "˞", "β", "θ", "χ", "ᵻ", "ⱱ",
];

// https://github.com/espeak-ng/espeak-ng/issues/694
fn remove_additional_separators(string: &str) -> String {
  Regex::new("_+")
    .unwrap()
    .replace_all(string, "_")
    .replace("_ ", " ")
    .to_owned()
}

fn remove_line_breaks(string: &str) -> String {
  string.replace("\n", " ").to_owned()
}
fn remove_extra_spaces(string: &str) -> String {
  string.replace("  ", " ").to_owned()
}
fn remove_non_phonetic_chars(string: &str) -> String {
  string
    .chars()
    .filter(|char| PHONEME_SET.contains(&(char.to_string().as_str())))
    .collect()
}

fn sanitize_espeak_output(string: &str) -> String {
  remove_non_phonetic_chars(&remove_extra_spaces(&remove_line_breaks(
    &remove_additional_separators(string),
  )))
}

// TODO: Handle pure whitespace
/** Adds the starting and ending whitespace from the source string to the target string */
fn preserve_boundary_whitespace(source: &str, target: &str) -> String {
  let starting_whitespace_len = source.len() - source.trim_start().len();
  let ending_whitespace_len = source.len() - source.trim_end().len();

  let starting_whitespace = &source[..starting_whitespace_len];
  let main_text = target.trim();
  let ending_whitespace = if ending_whitespace_len == 0 {
    ""
  } else {
    &source[(source.len() - ending_whitespace_len)..source.len()]
  };

  format!("{}{}{}", starting_whitespace, main_text, ending_whitespace)
}

fn collapse_whitespace(string: &str) -> String {
  Regex::new("\\s*([!,-.:;? '()])\\s*")
    .unwrap()
    .replace(string, "$1")
    .to_string()
}

pub fn to_phonetics(text: &str) -> String {
  sanitize_espeak_output(&text_to_phonemes(text))
}


pub async fn string_to_phonetics(text: &str, preserve_punctuation: bool) -> String {
  if preserve_punctuation {
    let res = Regex::new("([0-9]),([0-9])")
      .unwrap()
      .replace(text, "$1$2")
      .to_string();
    let (split_texts, punctuations) = extract_punctuation(&res);
    let phonemized_texts = split_texts.iter().map(|s| to_phonetics(s)).collect();

    let combined_phonemized_text = restore_punctuations(punctuations, phonemized_texts);

    return remove_line_breaks(&collapse_whitespace(&preserve_boundary_whitespace(
      text,
      &combined_phonemized_text,
    )))
    .to_owned();
  }
  return to_phonetics(text);
}

#[napi]
pub async fn phonemize(text: String, preserve_punctuation: bool) -> String {
    string_to_phonetics(text.as_str(), preserve_punctuation).await
}
