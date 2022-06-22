use napi_derive::napi;
use regex::Regex;

use crate::{
  leven::get_average_leven, text_to_phonemes, transform_raw_phoneme_timestamps, Chunk, NestedChunk,
  PhonemeChunk,
};

#[napi]
pub async fn force_align_phonemes_graphemes(
  text: String,
  phonemes: String,
  use_phoneme_bounds: bool,
) -> NestedChunk {
  let mut chunk = align_phonemes_graphemes(
    text.as_str(),
    (0..phonemes.len()).map(|_| 0.0).collect::<Vec<f64>>(),
    phonemes
      .chars()
      .map(|string| string.to_string())
      .collect::<Vec<String>>()
      .iter()
      .map(|string| string.as_str())
      .collect(),
    use_phoneme_bounds,
  )
  .await;
  chunk.start_time = chunk
    .chunks
    .get(0)
    .map(|chunk| chunk.start_time)
    .unwrap_or(0.0);
  chunk.end_time = chunk
    .chunks
    .get(0)
    .map(|chunk| chunk.end_time)
    .unwrap_or(0.0);
  chunk
}

#[napi]
pub async fn force_align_phonemes_graphemes_list(
  text: String,
  end_times: Vec<f64>,
  phonemes_list: Vec<String>,
  use_phoneme_bounds: bool,
) -> NestedChunk {
  let mut chunk = align_phonemes_graphemes(
    text.as_str(),
    end_times,
    phonemes_list.iter().map(|string| string.as_str()).collect(),
    use_phoneme_bounds,
  )
  .await;
  chunk.start_time = chunk
    .chunks
    .get(0)
    .map(|chunk| chunk.start_time)
    .unwrap_or(0.0);
  chunk.end_time = chunk
    .chunks
    .get(0)
    .map(|chunk| chunk.end_time)
    .unwrap_or(0.0);
  chunk
}

pub async fn align_phonemes_graphemes(
  text: &str,
  end_times: Vec<f64>,
  phonemes_list: Vec<&str>,
  use_phoneme_bounds: bool,
) -> NestedChunk {
  let phonemes = transform_raw_phoneme_timestamps(&phonemes_list, &end_times);

  let chunk: NestedChunk = NestedChunk {
    value: text.to_string(),
    start_time: 0.0,
    end_time: 0.0,
    start: 0.0,
    end: text.chars().count() as f64,
    chunks: Vec::new(),
  };

  let mut chunks: Vec<Chunk> = Vec::new();

  let mut phoneme_index = 0;
  let mut words: Vec<Chunk> = split_text_to_word_chunks(text);

  for word_index in 0..words.len() {
    let word = words.get_mut(word_index).unwrap();
    if phoneme_index >= phonemes.len() {
      break;
    }
    let phoneme = phonemes.get(phoneme_index).unwrap();

    let mut word = word.clone();

    // Fill word with data
    word.start_time = chunk
      .chunks
      .get(chunk.chunks.len() - 1)
      .map(|chunk| chunk.end_time)
      .unwrap_or(phonemes[phoneme_index].start_time);
    word.end_time = phonemes[phoneme_index].end_time;

    if use_phoneme_bounds {
      word.start = phoneme.start;
      word.end = phoneme.end;
    }

    let is_desync_detected = get_average_leven(
      &words,
      &phonemes,
      word_index,
      phoneme_index,
      usize::min(3, words.len() - word_index - 1).min(phonemes.len() - phoneme_index - 1),
      "forward",
    ) > 0.6;

    // Handle numbers. 2021 -> two thousand twenty one and anything else
    // that isComplex deems as too complicated for regular handling
    if is_complex(word.value.as_str()) {
      // Convert to phonemes and check how many words it was
      let word_phonemes = text_to_phonemes(word.value.as_str());
      let word_phoneme_word_count = usize::max(
        1,
        word_phonemes
          .split(" ")
          .filter(|string| string.len() > 0)
          .collect::<Vec<_>>()
          .len(),
      );

      phoneme_index += word_phoneme_word_count - 1;
      word.end_time = phonemes
        .get(phoneme_index)
        .map(|chunk| chunk.clone().end_time)
        .unwrap_or(phonemes[phonemes.len() - 1].end_time);
      if use_phoneme_bounds {
        word.end = phonemes[usize::min(phoneme_index, phonemes.len() - 1)].end
      };
    }
    // Detect and resolve desync
    else if is_desync_detected {
      // More accurately detect desyncing
      let offset =
        get_closest_correct_word_offset(&words, &phonemes, word_index, phoneme_index, 3, 8);

      if offset > 0 && word_index > 0 {
        let previous_word = words.get_mut(word_index - 1).unwrap();

        let start_time = previous_word.start_time;
        let end_time = previous_word.end_time;
        let middle_time = (end_time - start_time) / 2.0 + start_time;

        previous_word.end_time = middle_time;
        word.start_time = middle_time;
        word.end_time = end_time;
        phoneme_index -= 1;
      } else if offset < 0 {
        phoneme_index += 1;
        word.end_time = phonemes[phoneme_index - 1].end_time;
      }
    }

    phoneme_index += 1;

    chunks.push(word);
  }

  return NestedChunk {
    value: chunk.value,
    start: chunk.start,
    end: chunk.end,
    start_time: chunk.start_time,
    end_time: chunk.end_time,
    chunks,
  };
}

fn split_text_to_word_chunks(text: &str) -> Vec<Chunk> {
  // TODO: Replace with reduce or map with accumulator
  let mut character_counter = 0;
  let ignored_chars = [',', '.', '?', '!', ':', '[', ']', '{', '}', '"', ' '];

  return text
    .split_inclusive(ignored_chars)
    .map(|word| -> Option<Chunk> {
      // We use .chars().count() to get the number of characters rather than
      // the number of bytes
      let full_word_len = word.chars().count();

      // Remove the punctuation the end
      let trimmed_word = word
        .chars()
        .filter(|char| !ignored_chars.contains(char))
        .collect::<String>();
      let trimmed_word_len = trimmed_word.chars().count();

      let start = character_counter;
      let end = character_counter + trimmed_word_len;

      // Track the number of chars we've reached
      if word != "CS" {
        character_counter += full_word_len
      };

      // If it's not a word, continue
      if !is_word(trimmed_word.as_str()) {
        return Option::None;
      }

      // Otherwise, add the word to our list
      return Option::Some(Chunk {
        value: trimmed_word,
        start: start as f64,
        start_time: 0.0,
        end: end as f64,
        end_time: 0.0,
      });
    })
    .filter(|val| val.is_some())
    .map(|val| val.unwrap())
    .collect();
}

lazy_static! {
  pub static ref REGEX_COMPLEX_ACRONYM: Regex = Regex::new("(.*[A-Z].*){2,}").unwrap();
  pub static ref REGEX_COMPLEX_NUMBERS: Regex = Regex::new("[\\p{N}]").unwrap();
  pub static ref REGEX_COMPLEX_SPECIAL_CHARS: Regex =
    Regex::new("[/\\\\\\(\\)\\{\\}\\[\\]+@=]").unwrap();
  pub static ref REGEX_WORD: Regex = Regex::new(
    "[\\p{L}\\p{N}$¢£¤¥֏؋߾߿৲৳৻૱௹฿៛₠₡₢₣₤₥₦₧₨₩₪₫€₭₮₯₰₱₲₳₴₵₶₷₸₹₺₻₼₽₾₿꠸﷼﹩＄￠￡￥￦𑿝𑿞𑿟𑿠𞋿𞲰]"
  )
  .unwrap();
}

fn is_complex(string: &str) -> bool {
  REGEX_COMPLEX_ACRONYM.is_match(string)
    || REGEX_COMPLEX_NUMBERS.is_match(string)
    || REGEX_COMPLEX_SPECIAL_CHARS.is_match(string)
}

// Originally used \p{Sc} but rust doesnt have support so pulled it from
// https://www.compart.com/en/unicode/category/Sc
fn is_word(word: &str) -> bool {
  REGEX_WORD.is_match(word)
}

/**
 * Takes a wordIndex and phonemeIndex and finds the offset that will minimize the value from
 * the custom leven phoneme algorithm. Starts from the initial index and looks to the left
 * and right indices until the maxOffset is reached
 */
fn get_closest_correct_word_offset(
  words: &Vec<Chunk>,
  phonemes: &Vec<PhonemeChunk>,
  word_index: usize,
  phoneme_index: usize,
  max_offset: usize,
  max_distance: usize,
) -> isize {
  let word_index = word_index as isize;
  let phoneme_index = phoneme_index as isize;
  let max_distance = max_distance as isize;
  let max_offset = max_offset as isize;

  // Given wordIndex = 3, maxOffset = 4, words.length = 10, generates [-3, -2, -1, 0, 1, 2, 3, 4]
  let word_index_offsets = (0..(max_offset * 2 + 1))
    .enumerate()
    .map(|(i, _)| (max_offset as isize) - (i as isize))
    .filter(|i| {
      i + word_index >= 0
        && i + word_index + max_distance < (words.len() as isize)
        && phoneme_index + max_distance < (phonemes.len() as isize)
    })
    .collect::<Vec<_>>(); // Prevents offsets that go out of bounds

  if word_index_offsets.len() == 0 {
    return 0;
  }
  let average_leven_values = word_index_offsets
    .iter()
    .map(|offset| {
      get_average_leven(
        words,
        phonemes,
        (word_index + offset) as usize,
        phoneme_index as usize,
        max_distance as usize,
        "forward",
      )
    })
    .collect::<Vec<_>>();

  let lowest_leven_value = average_leven_values
    .clone()
    .into_iter()
    .reduce(|a, b| a.min(b))
    .unwrap();
  if lowest_leven_value > 0.4 {
    return 0;
  }

  // TODO: Check from middle outwards to minimize offset in the case of identical mins. Given -> [0 1 2 3 4 5 6], Do -> [3 2 4 1 5 0 6]
  word_index_offsets[average_leven_values
    .into_iter()
    .position(|a| a == lowest_leven_value)
    .unwrap()]
}
