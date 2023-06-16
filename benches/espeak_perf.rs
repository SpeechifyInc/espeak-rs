use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

// This is a struct that tells Criterion.rs to use the "futures" crate's current-thread executor
fn criterion_benchmark(c: &mut Criterion) {
    let input_text = "Large piece of text";

    // c.bench_with_input(
    //   BenchmarkId::new("espeak_threaded", input_text),
    //   &input_text,
    //   |b, &s| {
    //     b.to_async(Runtime::new().unwrap())
    //       .iter(|| EspeakRunner::run_phoneme_task(s.to_owned()));
    //   },
    // );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
