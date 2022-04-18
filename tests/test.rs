use espeak_rs::EspeakAddon;

use markov::Chain;
use std::process::Command;

use pretty_assertions::{assert_str_eq};
mod common;

use futures::executor::block_on;
#[test]
fn does_not_panic() {
  let espeak = EspeakAddon::default();
  let result = block_on(espeak.text_to_phonemes("This is a piece of text".to_string()));
  println!("{}", result);
}

fn call_espeak_cli(text: &str) -> String {
  let result = Command::new("sh")
    .arg("-c")
    .arg(format!("espeak-ng \"{}\" -q -x --ipa -v en-us", text))
    .output()
    .expect("cli execution failed")
    .stdout;
  let output = String::from_utf8_lossy(&result).into_owned();
  output
}

fn markov_test_once(chain: &Chain<String>) {
  let espeak = EspeakAddon::default();
  let random_input = chain.generate_str();
  let result = block_on(espeak.text_to_phonemes(random_input.clone()));
  let result_from_cli = call_espeak_cli(&random_input);

  assert_str_eq!(result_from_cli.trim(), result.trim(), "\n Failed at Input: {}", random_input);
}

#[test]
fn markov_test_lot() {
  const N: u32 = 15;
  let chain = common::setup_markov().unwrap();
  (1..N).for_each(|_| markov_test_once(&chain));
}
