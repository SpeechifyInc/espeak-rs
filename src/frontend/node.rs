use napi_derive::napi;

use crate::NestedChunk;

#[cfg_attr(feature = "napi", napi)]
pub fn force_align_phonemes_graphemes_list(
    text: String,
    end_times: Vec<f64>,
    phonemes_list: Vec<String>,
    align_phonemes: bool,
) -> NestedChunk {
    crate::align::force_align_phonemes_graphemes_list(
        text,
        end_times,
        phonemes_list,
        align_phonemes,
    )
}

#[cfg_attr(feature = "napi", napi)]
pub fn force_align_phonemes_graphemes(
    text: String,
    end_times: Vec<f64>,
    phonemes_list: Vec<String>,
    align_phonemes: bool,
) -> NestedChunk {
    let phonemes_list: Vec<&str> = phonemes_list.iter().map(|string| string.as_str()).collect();
    crate::align::align_phonemes_graphemes(&text, end_times, phonemes_list, align_phonemes)
}

#[cfg_attr(feature = "napi", napi)]
pub fn phonemize(text: String, preserve_punctuation: bool) -> String {
    crate::phonetics::internal::phonemize(text, preserve_punctuation)
}
