use std::fs;
use rand::{thread_rng,Rng};

use crate::PasswordConfig;
use crate::UPPERCASE;
use crate::LOWERCASE;
use crate::NUMBERS;
use crate::SYMBOLS;

pub fn read_file(fp: &str) -> String {
  // This method opens a file in read only mode and stores the data in a string
  let path = std::path::Path::new(fp);
  return match fs::read_to_string(path) {
      Err(why) => panic!("couldn't read {}: {}", path.display(), why),
      Ok(data) => data,
  };
}

pub fn rng_alphanumeric(length: u32, charset: &str) -> String {
  let char_vec: Vec<char> = charset.chars().collect();
  return (0..length).map(|_| {
      let idx = thread_rng().gen_range(0..char_vec.len());
      char_vec[idx] as char
  }).collect();
}
