use rand::{thread_rng,Rng};

use crate::PasswordConfig;
use crate::UPPERCASE;
use crate::LOWERCASE;
use crate::NUMBERS;
use crate::SYMBOLS;
use crate::utils;

pub fn fancy_correct_horse(config: &PasswordConfig, corpus: &Vec<String>) -> String {
  // Build a fancy correct-horse password
  // Build a Vec<str> for the passphrase
  let mut words: Vec<&str> = Vec::with_capacity(config.passphrase_length);
  for _ in 0..config.passphrase_length {
      let num: usize = thread_rng().gen_range(0..corpus.len());
      let word: &str = corpus[num].as_str();
      words.push(word);
  }
  let fancy_correct_horse: String = format!("{}{}{}{}",
      utils::rng_alphanumeric(config.key_let_length, LOWERCASE),
      utils::rng_alphanumeric(config.key_num_length, NUMBERS),
      config.key_seperator,
      words.join(&config.passphrase_seperator)
  );
  return fancy_correct_horse
}
