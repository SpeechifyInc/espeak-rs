use markov::Chain;
use std::{
  fs,
  io::{self, BufRead, BufReader}, process::Command,
};

pub fn setup_markov() -> io::Result<Chain<String>> {
  let entries = fs::read_dir("./tests/assets")?
    .map(|result| result.map(|item| item.path()))
    .collect::<Result<Vec<_>, io::Error>>()?;
  let mut chain: Chain<String> = Chain::new();
  entries.iter().for_each(|item| {
    let file = fs::File::open(item).unwrap();
    let buf_reader = BufReader::new(file);
    buf_reader.lines().for_each(|line| {
      if let Ok(content) = line {
        chain.feed_str(&content);
      }
    })
  });

  Ok(chain)
}


pub fn call_espeak_cli(text: &str) -> String {
  let result = Command::new("sh")
    .arg("-c")
    .arg(format!("espeak-ng \"{}\" -q -x --ipa -v en-us", text))
    .output()
    .expect("cli execution failed")
    .stdout;
  let output = String::from_utf8_lossy(&result).into_owned();
  output
}
