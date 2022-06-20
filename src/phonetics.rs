use crate::text_to_phonemes;
use regex::Regex;

// https://github.com/espeak-ng/espeak-ng/issues/694
fn removeAdditionalSeparators(string: &str) -> &str {
    Regex::new("_+").unwrap().replace_all(string, "_").replace("_ ", " ").as_str()
}

fn remove_line_breaks(string: &str) -> &str { string.replace("\n", " ").as_str() }
fn remove_extra_spaces(string: &str) -> &str  {string.replace("  ", " ").as_str()}
fn remove_non_phonetic_chars(string: &str) -> &str { string.chars().filter(|char| phoneme_set.contains(char) ).collect() }

fn sanitize_espeak_output(string: &str) -> &str {
  remove_non_phonetic_chars(remove_extra_spaces(remove_line_breaks(removeAdditionalSeparators(string))))
}

// TODO: Handle pure whitespace
/** Adds the starting and ending whitespace from the source string to the target string */
fn preserve_boundary_whitespace(source: &str, target: &str) -> &str {
  let starting_whitespace_len = source.len() - source.trim_start().len();
  let ending_whitespace_len = source.len() - source.trim_end().len();

  let starting_whitespace = source[..starting_whitespace_len];
  let main_text = target.trim();
  let ending_whitespace = (if ending_whitespace_len == 0 { "" } else { &source[(source.len() - ending_whitespace_len)..source.len()]});
  return (
    starting_whitespace +
    main_text +
    ending_whitespace
  ).as_str();
}

fn collapseWhitespace(string: &str) -> &str { Regex::new("\\s*([!,-.:;? '()])\\s*").replace(string, "$1") }

fn to_phonetics(text: &str) -> &str { sanitize_espeak_output(text_to_phonemes(text)) }

pub async fn stringToPhonetics(
  text: Vec<&str>,
  preserve_punctuation: bool
) -> Vec<&str> {
  if (preserve_punctuation) {
    let split_texts = extract_punctuation(Regex::new("([0-9]),([0-9])").replace(text, "$1$2"));
    let phonemized_texts = split_texts.iter().map(to_phonetics);

    let combined_phonemized_text = restore_punctuation(split_texts.punctuations, phonemized_texts);
    remove_line_breaks(collapse_whitespace(preserve_boundary_whitespace(text, combined_phonemized_text)))
  }
  return to_phonetics(text);
}

const phoneme_set: [&str; 2] = [" ",
  "!",
  "'",
  "(",
  ")",
  ",",
  "-",
  ".",
  ":",
  ";",
  "?",
  "a",
  "b",
  "c",
  "d",
  "e",
  "f",
  "h",
  "i",
  "j",
  "k",
  "l",
  "m",
  "n",
  "o",
  "p",
  "q",
  "r",
  "s",
  "t",
  "u",
  "v",
  "w",
  "x",
  "y",
  "z",
  "æ",
  "ç",
  "ð",
  "ø",
  "ħ",
  "ŋ",
  "œ",
  "ǀ",
  "ǁ",
  "ǂ",
  "ǃ",
  "ɐ",
  "ɑ",
  "ɒ",
  "ɓ",
  "ɔ",
  "ɕ",
  "ɖ",
  "ɗ",
  "ɘ",
  "ə",
  "ɚ",
  "ɛ",
  "ɜ",
  "ɞ",
  "ɟ",
  "ɠ",
  "ɡ",
  "ɢ",
  "ɣ",
  "ɤ",
  "ɥ",
  "ɦ",
  "ɧ",
  "ɨ",
  "ɪ",
  "ɫ",
  "ɬ",
  "ɭ",
  "ɮ",
  "ɯ",
  "ɰ",
  "ɱ",
  "ɲ",
  "ɳ",
  "ɴ",
  "ɵ",
  "ɶ",
  "ɸ",
  "ɹ",
  "ɺ",
  "ɻ",
  "ɽ",
  "ɾ",
  "ʀ",
  "ʁ",
  "ʂ",
  "ʃ",
  "ʄ",
  "ʈ",
  "ʉ",
  "ʊ",
  "ʋ",
  "ʌ",
  "ʍ",
  "ʎ",
  "ʏ",
  "ʐ",
  "ʑ",
  "ʒ",
  "ʔ",
  "ʕ",
  "ʘ",
  "ʙ",
  "ʛ",
  "ʜ",
  "ʝ",
  "ʟ",
  "ʡ",
  "ʢ",
  "ˈ",
  "ˌ",
  "ː",
  "ˑ",
  "˞",
  "β",
  "θ",
  "χ",
  "ᵻ",
  "ⱱ"
];
