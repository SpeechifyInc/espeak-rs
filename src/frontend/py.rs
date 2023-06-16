use pyo3::pyfunction;

use crate::NestedChunk;

#[pyfunction]
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

#[pyfunction]
pub fn force_align_phonemes_graphemes(
    text: &str,
    end_times: Vec<f64>,
    phonemes_list: Vec<&str>,
    align_phonemes: bool,
) -> NestedChunk {
    crate::align::align_phonemes_graphemes(text, end_times, phonemes_list, align_phonemes)
}

#[pyfunction]
pub fn phonemize(text: String, preserve_punctuation: bool) -> String {
    crate::phonetics::internal::phonemize(text, preserve_punctuation)
}
