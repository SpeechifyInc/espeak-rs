use regex::Regex;
use std::fmt::Display;

pub fn extract_punctuation(text: &str) -> (Vec<String>, Vec<String>) {
  let split_text_regex = Regex::new(r"([!(),-.:;?]+)").unwrap();
  //   let (texts, punctuations) =
  let splits = split_text_regex.split(&text).collect::<Vec<&str>>();
  let captures = split_text_regex
    .find_iter(text)
    .map(|m| m.as_str())
    .collect::<Vec<&str>>();

  (
    splits.iter().map(|i| (*i).to_owned()).collect(),
    captures.iter().map(|i| (*i).to_owned()).collect(),
  )
}

pub fn whitespace_replacer(text: &str, replacer: &str, last_remaining_chars: &mut usize) -> String {
  let start = text.len() - text.trim_start().len();
  let end = text.len() - text.trim_end().len();

  println!("{} {}", start, end);
  let start_trimmed = &text[0..usize::min(start, *last_remaining_chars)];
  println!("{}", &start_trimmed);
  *last_remaining_chars = usize::max(0, replacer.len() - end);

  start_trimmed[0..start_trimmed.len() - usize::min(end, replacer.len()) + replacer.len()]
    .to_string()
}

pub fn add_strings_replacing_whitespace(str1: &str, str2: &str) -> String {
  let whitespace_length = str1.len() - str1.trim_end().len();
  let mut result = str1[..str1.len() - usize::min(whitespace_length, str2.len())].to_owned();
  result.push_str(str2);
  result
}

pub fn restore_punctuations(punctuations: Vec<String>, texts: Vec<String>) -> String {
  let mut last_remaining_chars: usize = 0;
  texts
    .iter()
    .zip(punctuations.iter())
    .map(|(text, punctuation)| whitespace_replacer(text, punctuation, &mut last_remaining_chars))
    .collect::<Vec<_>>()
    .join("")
}

#[cfg(test)]
mod tests {
  use crate::phonetics::punctuation::restore_punctuations;

use super::extract_punctuation;
  #[test]
  fn test_extract_punctuation() {
    let text = "Hey, this is a, with a bunch of punctuations!";
    let (texts, punctuations) = extract_punctuation(text);
    let result_text = vec!["Hey", " this is a", " with a bunch of punctuations", ""];
    let result_punctuations = vec![",", ",", "!"];

    assert_eq!(result_text, texts);
    assert_eq!(punctuations, result_punctuations);

  }

  #[test] 
  fn test_restore_punctuations() {
    let text = "Hey, this is a, with a bunch of punctuations!";
    let (texts, punctuations) = extract_punctuation(text);

    let result = restore_punctuations(punctuations, texts);
    println!("{}", result);

  }
}
