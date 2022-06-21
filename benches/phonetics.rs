use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use espeak_rs::phonetics::punctuation::extract_punctuation;
// This is a struct that tells Criterion.rs to use the "futures" crate's current-thread executor
fn criterion_benchmark(c: &mut Criterion) {
  let input_text = "This, is a piece of text, that has punctuations.";

  c.bench_with_input(
    BenchmarkId::new("espeak_threaded", input_text),
    &input_text,
    |b, &s| {
        b.iter(|| { 
            extract_punctuation(s)
        })
    },
  );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
