use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{phoneme_to_word, Chunk, PhonemeChunk};

#[must_use]
pub fn leven_phoneme(a: &str, b: &str) -> usize {
  let a = a.to_lowercase();
  let b = b.to_lowercase();

  let mut result = 0;

  /* Shortcut optimizations / degenerate cases. */
  if a == b {
    return result;
  }

  let length_a = a.chars().count();
  let length_b = b.chars().count();

  if length_a == 0 {
    return length_b;
  }

  if length_b == 0 {
    return length_a;
  }

  /* Initialize the vector.
   *
   * This is why it’s fast, normally a matrix is used,
   * here we use a single vector. */
  let mut cache: Vec<usize> = (1..).take(length_a).collect();
  let mut distance_a;
  let mut distance_b;

  /* Loop. */
  for (index_b, code_b) in b.chars().enumerate() {
    result = index_b;
    distance_a = index_b;

    for (index_a, code_a) in a.chars().enumerate() {
      distance_b = if fuzzy_compare_chars(code_a, code_b) {
        distance_a
      } else {
        distance_a + 1
      };

      distance_a = cache[index_a];

      result = if distance_a > result {
        if distance_b > result {
          result + 1
        } else {
          distance_b
        }
      } else if distance_b > distance_a {
        distance_a + 1
      } else {
        distance_b
      };

      cache[index_a] = result;
    }
  }

  result
}

pub static SIMILAR_CHARS: Lazy<HashMap<char, Vec<char>>> = Lazy::new(|| {
  HashMap::from([
    ('j', Vec::from(['y'])),
    ('k', Vec::from(['c', 'q', 'x'])),
    ('c', Vec::from(['k', 's'])),
    ('s', Vec::from(['c', 'v'])),
    ('y', Vec::from(['i'])),
    ('i', Vec::from(['y', 'e'])),
    ('e', Vec::from(['i', 'a'])),
    ('z', Vec::from(['s'])),
    ('a', Vec::from(['o'])),
    ('d', Vec::from(['g'])),
  ])
});

fn fuzzy_compare_chars(a: char, b: char) -> bool {
  a == b || SIMILAR_CHARS.get(&b).unwrap_or(&Vec::new()).contains(&a)
}

pub fn leven_phoneme_relative(word: &Chunk, phoneme: &str) -> f32 {
  // We use toLowerCase since we don't want to be case sensitive
  // We use a minimum value of 4 for the length to avoid short words getting skewed values
  leven_phoneme(
    word.value.to_lowercase().as_str(),
    phoneme_to_word(phoneme).as_str(),
  ) as f32
    / (word.value.len() as f32).max(4.0)
}

pub fn get_average_leven(
  words: &[Chunk],
  phonemes: &[PhonemeChunk],
  word_index: usize,
  phoneme_index: usize,
  distance: usize,
  direction: &str,
) -> f32 {
  let direction_multi: isize = if direction == "forward" { 1 } else { -1 };

  let mut leven_sum: f32 = 0.0;
  for i in 0..(distance as isize) {
    leven_sum += leven_phoneme_relative(
      &words[((word_index as isize) + i * direction_multi) as usize],
      phonemes[((phoneme_index as isize) + i * direction_multi) as usize]
        .value
        .as_str(),
    );
  }

  let distance = distance as f32;

  leven_sum / distance
}
