mod common;

use espeakng;

// #[napi::tokio::test]
// async fn does_not_panic() {
//   let _runner = EspeakRunner::new();
//   let result = EspeakRunner::run_phoneme_task("this is a piece of text".to_string()).await;
//   println!("{}", result);
// }

// #[napi::tokio::test]
// async fn does_not_panic_multiple() {
//   const N: u32 = 10000;
//   let _runner = EspeakRunner::new();
//   let futures = (1..N)
//     .map(|_| EspeakRunner::run_phoneme_task("In this case, calling shutdown_timeout with an explicit wait timeout can work. The shutdown_timeout will signal all tasks to shutdown and will wait for at most duration for all spawned tasks to terminate. If timeout elapses before all tasks are dropped, the function returns and outstanding tasks are potentially leaked. ".to_string()))
//     .collect::<Vec<_>>();
//   let _res = join_all(futures).await;
// }

// fn markov_test_once(chain: &Chain<String>) {
//   let espeak = EspeakAddon::default();
//   let random_input = chain.generate_str();
//   let result = block_on(espeak.text_to_phonemes(random_input.clone()));
//   let result_from_cli = common::call_espeak_cli(&random_input);

//   assert_str_eq!(
//     result_from_cli.trim(),
//     result.replace("ˈ", "").trim(),
//     "\n Failed at Input: {}",
//     random_input
//   );
// }
// async fn markov_test_once_async(chain: &Chain<String>, count: u32) {
//   let random_input = chain.generate_str();
//   let result = EspeakRunner::run_phoneme_task(random_input.clone())
//     .await
//     .replace("ˈ", "")
//     .replace("ˌ", "");
//   let result_from_cli = common::call_espeak_cli(&random_input)
//     .replace("ˈ", "")
//     .replace("ˌ", "")
//     .replace("\n", " ");

//   assert_str_eq!(
//     result.trim(),
//     result_from_cli.trim(),
//     "\n Failed at Input: {} \n Generated Result:{} \n Expected result: {} \n in count: {}",
//     random_input,
//     result_from_cli,
//     result,
//     count
//   );
// }

// #[test]
// fn markov_test_lot() {
//   const N: u32 = 15;
//   let chain = common::setup_markov().unwrap();
//   (1..N).for_each(|_| markov_test_once(&chain));
// }

#[test]
fn markov_test_a_lot_threaded() -> Result<(), espeakng::Error> {
  const N: u32 = 10000;
  //   let chain = common::setup_markov().unwrap();
  //   let _runner = EspeakRunner::new();

  let join_handles = (1..N)
    .map(|_| {
      let mut speaker = espeakng::initialise(None).unwrap().lock();
      let phonemes = speaker
        .text_to_phonemes("The", espeakng::PhonemeGenOptions::Standard)
        .unwrap()
        .unwrap();
      phonemes
    })
    .collect::<Vec<_>>();

//   let join_handles = (1..N)
//     .map(|_| {
//       let espeak_handle = std::thread::spawn(move || {
//         let mut speaker = espeakng::initialise(None).unwrap().lock();
//         let phonemes = speaker
//           .text_to_phonemes("Hello", espeakng::PhonemeGenOptions::Standard)
//           .unwrap()
//           .unwrap();
//         println!("{}", phonemes);
//         phonemes
//       });
//       espeak_handle
//     })
//     .collect::<Vec<_>>();

//   for handle in join_handles {
//     handle.join().unwrap();
//   }

  Result::Ok(())

  //   let futures = (1..N)
  //     .map(|x| markov_test_once_async(&chain, x))
  //     .collect::<Vec<_>>();
  //   let _res = join_all(futures).await;
}
