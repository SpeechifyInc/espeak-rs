
use regex::Regex;
use std::fmt::Display;

fn unenumerate<T: Display>(a: impl IntoIterator<Item = (usize, T)>) -> Vec<String> {
  a.into_iter()
    .map(|(_, e)| e)
    .map(|s| s.to_string())
    .collect()
}

pub fn extract_punctuation(text: &str) -> (Vec<String>, Vec<String>) {
  let split_text_regex = Regex::new("/([!(),-.:;?]+)/").unwrap();
  //   let (texts, punctuations) =
  let splits = split_text_regex.split(&text).collect::<Vec<&str>>();

  (
    splits.iter().step_by(2).map(|i| (*i).to_owned()).collect(),
    splits
      .iter()
      .skip(1)
      .step_by(2)
      .map(|i| (*i).to_owned())
      .collect(),
  )
}

pub fn whitespace_replacer(text: &str, replacer: &str, last_remaining_chars: &mut usize) -> String {
  let start = text.len() - text.trim_start().len();
  let end = text.len() - text.trim_end().len();

  let start_trimmed = &text[..usize::min(start, *last_remaining_chars)];
  *last_remaining_chars = usize::max(0, replacer.len() - end);

  start_trimmed[..start_trimmed.len() - usize::min(end, replacer.len()) + replacer.len()]
    .to_string()
}

pub fn add_strings_replacing_whitespace(str1: &str, str2: &str) -> String {
  let whiteSpaceLength = str1.len() - str1.trim_end().len();
  let mut result = str1[..str1.len() - usize::min(whiteSpaceLength, str2.len())].to_owned();
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
  use super::extract_punctuation;
  #[test]
  fn test_extract_punctuation() {
    let text = "Hey, this is a, with a bunch of punctuations!";
    let (texts, punctuations) = extract_punctuation(text);

    println!("{:?}, {:?}", texts, punctuations);
  }
}
