use rand::{thread_rng,Rng};
use colored::*;

use crate::PasswordConfig;
use crate::UPPERCASE;
use crate::LOWERCASE;
use crate::NUMBERS;
use crate::SYMBOLS;
use crate::utils;

fn get_horse_words(config: &PasswordConfig, corpus: &Vec<&str>) -> Vec<String> {
  // DRY:: Build a fancy correct-horse password as Vec<str>
  let mut words: Vec<String> = Vec::with_capacity(config.passphrase_length);
  for _ in 0..config.passphrase_length {
    let num: usize = thread_rng().gen_range(0..corpus.len());
    let word: &str = &corpus[num];
    words.push(word.to_string());
  }
  return words
}

pub fn fancy_correct_horse(config: &PasswordConfig, corpus: &Vec<&str>) -> String {
  // Format a fancy-horse string
  let words: Vec<String> = get_horse_words(config, corpus);
  let fancy_correct_horse: String = format!("{}{}{}{}",
      utils::rng_alphanumeric(config.key_let_length, LOWERCASE),
      utils::rng_alphanumeric(config.key_num_length, NUMBERS),
      config.key_separator,
      words.join(&config.passphrase_separator)
  );
  if config.verbose {
    // Gather stats for the key portion
    let sep_entropy: f32 = utils::get_bit_entropy(
      (1 + config.passphrase_length) as f32,
      SYMBOLS.len() as f32
    );
    let key_pool = format!("{}{}",LOWERCASE,NUMBERS);
    let key_entropy: f32 = utils::get_bit_entropy(
      (config.key_let_length + config.key_num_length) as f32, 
      key_pool.len() as f32
    );
    let pass_entropy: f32 = utils::get_bit_entropy(
      config.passphrase_length as f32,
      corpus.len() as f32
    );
    let entropy_best_case: f32 = utils::get_bit_entropy(
      fancy_correct_horse.len() as f32,
      format!("{}{}{}{}",
        UPPERCASE,LOWERCASE,
        NUMBERS,SYMBOLS
      ).len() as f32
    );
    let entropy_worst_case: f32 = sep_entropy + key_entropy + pass_entropy;
    println!(
      "Entropy Statistics:\n  {}: {}\n  {}: {}\n",
      "Worst".red(),
      format!("{} bits {}",
        entropy_worst_case.to_string().yellow(),
        "// assumed the attacker knows format and dictionary".dimmed()
      ),
      "Best".green(),
      format!("{} bits {}",
        entropy_best_case.to_string().yellow(),
        "// assumed the attacker knows nothing".dimmed()
      )
    )
  }
  return fancy_correct_horse
}

pub fn correct_horse(config: &PasswordConfig, corpus: &Vec<&str>) -> String {
  // Format a correct-horse string
  let words: Vec<String> = get_horse_words(config, corpus);
  return format!("{}", words.join(&config.passphrase_separator))
}

pub fn random_ascii(config: &PasswordConfig) -> String {
  let mut charset = String::new();
  charset.push_str(UPPERCASE);
  charset.push_str(LOWERCASE);
  charset.push_str(NUMBERS);
  charset.push_str(SYMBOLS);
  return utils::rng_alphanumeric(config.passphrase_length as u32, &charset)
}
