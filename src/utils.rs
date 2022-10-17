use std::fs;
use regex::Regex;
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

pub fn get_correct_horse_words(_config: &PasswordConfig) -> Vec<String> {
  // Load a word corpus file as a string
  // TODO: Read user-defined corpus
  // TODO: Automatically download and store this in user's cache home
  let word_corpus_string: String = read_file("/home/eperry/Downloads/english-words/words.txt");
  // Template regular expression to ensure a minimum word-length for correct-horse.
  let minimum_passphrase_word_length: u32 = 4; //? Allow this to be defined perhaps.
  let re_object: Regex = Regex::new(
      format!(r"[{}{}{}{}]{{{}}}",
              UPPERCASE,LOWERCASE,NUMBERS,SYMBOLS,
              minimum_passphrase_word_length.to_string()
          ).as_str()
      ).unwrap();
  // Break the corpus into a vector of strings and filter it against the regular expression
  // TODO: Ensure the first character in each string is capitalized
  // TODO: Filter out hyphenated words for memorability?
  
  let corpus_vector = word_corpus_string
      .split("\n")
      .map(|word| word.to_string())
      .filter(|word| {re_object.is_match(word)})
      .collect::<Vec<String>>();
  
  //let word_corpus_vector: Vec<String> = word_corpus_string.split("\n")
  //    .collect::<Vec<String>>()
  //    .into_iter()
  //    .filter(|w| { re_object.is_match(w) })
  //    .collect::<Vec<String>>()
  //;
  return corpus_vector
}


//fn print_type_of<T>(_: &T) {
//    ! I have been told not to do this?
//    println!("{}", std::any::type_name::<T>())
//}

//fn longest_contiguous_sequence(wordlist: &[String]) {
  // For forcing the words into an alliteration
//    // TODO: Implement algorithm
//    //    class Solution:
//    //    def longestContiguousSequence(self, words):
//    //        l = len(words)
//    //        if l == 0:
//    //            return 0
//    //        i, ans, p = 1, 1, 0
//    //        while i < l:
//    //            if words[i][0] != words[p][0]:
//    //                p = i
//    //            else:
//    //                ans = max(ans, i - p + 1)
//    //            i += 1
//    //        return ans
//    let length: usize = wordlist.len();
//    if length == 0 {panic!("Sequence vector contained no items for enumerations")};
//    
//    // iteration trackers
//    let mut iteration = 0;
//    
//    while iteration < length {
//        
//    }
//}
