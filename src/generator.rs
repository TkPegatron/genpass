use rand::{thread_rng,Rng};
use crate::hash_utils;

pub enum Styles {
  BasicRand {
    length: usize
  },
  CorrectHorse {
    word_separator: String,
    word_count: usize
  },
  FancyHorse {
    word_separator: String,
    word_count: usize,
    key_num: usize,
    key_alp: usize,
  },
}

impl Clone for Styles {
  fn clone(&self) -> Self {
    match &self {
      Styles::FancyHorse {
        word_separator,
        word_count,
        key_num,
        key_alp
        } => Styles::FancyHorse { 
          word_separator: word_separator.clone(), 
          word_count: word_count.clone(),
          key_num: key_num.clone(),
          key_alp: key_alp.clone() 
        },
      Styles::CorrectHorse {
        word_separator,
        word_count 
        } => Styles::CorrectHorse {
          word_separator: word_separator.clone(),
          word_count: word_count.clone() 
        },
        _ => panic!("NOT IMPLEMENTED") //TODO Implement the other constructors
    }
  }
}

pub struct Options {
  // This should be a value >= 60 (according to Okta)
  pub target_entropy: f64,
  // These should be vectors of strings
  pub corpus_words_vector: Vec<String>,
  pub corpus_numeral_vector: Vec<String>,
  pub corpus_alpha_vector: Vec<String>,
  // This separates sections of a passphrase
  pub token_sparator: String,
}

impl Default for Options {
  fn default() -> Self {
      let fake_corpus = "some default test words"
        .to_string().split(" ")
        .into_iter().map(|s| s.to_string())
        .collect::<Vec<String>>();
      Options {
        target_entropy: 60.0,
        corpus_words_vector: fake_corpus,
        corpus_numeral_vector: "0123456789"
          .split("")
          .into_iter()
          .map(|s| s.to_string())
          .collect::<Vec<String>>(),
        corpus_alpha_vector: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
          .split("")
          .into_iter()
          .map(|s| s.to_string())
          .collect::<Vec<String>>(),
        token_sparator: ":".to_string(),
      }
  }
}

impl Clone for Options {
  fn clone(&self) -> Self {
    Options {
      target_entropy: self.target_entropy.clone(),
      corpus_words_vector: self.corpus_words_vector.clone(),
      corpus_numeral_vector: self.corpus_numeral_vector.clone(),
      corpus_alpha_vector: self.corpus_alpha_vector.clone(),
      token_sparator: self.token_sparator.clone(),
    }
  }
}

pub struct Password {
  pub data: String,
  pub entropy: f64,
  options: Options,
  style: Styles
}

impl Password {
  pub fn new(options: &Options, style: &Styles) -> Self {
    // loop {
    //   Set data to generated string return value
    //   Set entropy to return of get_entropy()
    //   Determine if entropy is sufficient
    //     ok: Return instanciated self
    //     _: Increment counter, panic after max retries
    // }
    let mut data: String = String::from("");
    
    // Iterate until the entropy check breaks the loop or we reach 10 itrs
    for _i in 0..10 {
      data = match &style {
        Styles::FancyHorse {
          ref word_separator,
          ref word_count,
          ref key_num,
          ref key_alp,
        } => { format!(
            "{}{}{}{}",
            random_pool_choice(key_alp, &options.corpus_alpha_vector).join(""),
            random_pool_choice(key_num, &options.corpus_numeral_vector).join(""),
            options.token_sparator,
            random_pool_choice(word_count, &options.corpus_words_vector)
              .join(word_separator.as_ref())
          )
        },
        //Styles::CorrectHorse { word_separator, word_count } => panic!("Not Implemented!"),
        //Styles::BasicRand { length } => panic!("Not Implemented!"),
        _ => panic!("Not Implemented!") //? Safety catch
      };
    }
    return Password { 
      data: data.clone(),
      entropy: calculate_entropy(data.len() as f64, 68 as f64).trunc(), //TODO: Softcode the value of pool_size
      options: options.clone(),
      style: style.clone()
    }
  }
  pub fn hash_argon2(&self) -> String {
    hash_utils::hash_argon2(self.data.clone())
  }
  pub fn hash_sha512(&self) -> String {
    hash_utils::hash_sha512(self.data.clone())
  }
}

impl std::fmt::Display for Password {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result {
    write!(f, "{}", self.data)
  }
}

// Calculate the entropy of the input values using E = log2(R^L)
fn calculate_entropy(data_size: f64, pool_size: f64) -> f64 {
  f64::powf(pool_size, data_size).log2()
}

// Randomly select a subset from the set and return the subset
fn random_pool_choice(len: &usize, set: &Vec<String>) -> Vec<String> {
  let mut subset: Vec<String> = Vec::with_capacity(*len);
  for _ in 0..*len {
    let sel_idx: usize = thread_rng().gen_range(0..set.len());
    let element: &str = &set[sel_idx];
    subset.push(element.to_string())
  }
  return subset
}
