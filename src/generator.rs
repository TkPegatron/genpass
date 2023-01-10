use crate::hash_utils;
use colored::{Colorize,ColoredString};
use rand::{thread_rng, Rng};

pub enum Styles {
    BasicRand {
        length: usize,
    },
    CorrectHorse {
        word_separator: String,
        word_count: usize,
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
        match self {
            Styles::FancyHorse {
                word_separator,
                word_count,
                key_num,
                key_alp,
            } => Styles::FancyHorse {
                word_separator: word_separator.clone(),
                word_count: word_count.clone(),
                key_num: key_num.clone(),
                key_alp: key_alp.clone(),
            },
            Styles::CorrectHorse {
                word_separator,
                word_count,
            } => Styles::CorrectHorse {
                word_separator: word_separator.clone(),
                word_count: word_count.clone(),
            },
            _ => panic!("NOT IMPLEMENTED"), //TODO Implement the other constructors
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
            .to_string()
            .split(" ")
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        Options {
            target_entropy: 60.0,
            corpus_words_vector: fake_corpus,
            corpus_numeral_vector: crate::NUMBERS
                .split("")
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            corpus_alpha_vector: crate::LOWERCASE
                .split("")
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            token_sparator: "@".to_string(),
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
//    options: Options,
//    style: Styles,
}

impl Password {
    pub fn new(options: Options, style: Styles) -> Self {
        // loop {
        //   Set data to generated string return value
        //   Set entropy to return of get_entropy()
        //   Determine if entropy is sufficient
        //     ok: Return instanciated self
        //     _: Increment counter, panic after max retries
        // }
        let mut data: String = String::from("");
        let ascii_sym_string: String = String::from (crate::ASCII_SYMBOLS);

        // Iterate until the entropy check breaks the loop or we reach 10 itrs
        for _i in 0..10 {
            data = match style {
                Styles::FancyHorse {
                    ref word_separator,
                    ref word_count,
                    ref key_num,
                    ref key_alp,
                } => {
                    //? There is a bug in this code that does not allow the generator to satisfy its directives
                    let key_alpha =
                        random_pool_choice(*key_alp, options.corpus_alpha_vector.clone()).join("");
                    //assert!(key_alpha.len() == *key_alp);

                    let key_numer = 
                        random_pool_choice(*key_num, options.corpus_numeral_vector.clone()).join("");
                    //assert!(key_numer.len() == *key_num);

                    // Generate and process the result into colored strings for visability
                    let phrase_vec: Vec<ColoredString> =
                        random_pool_choice(
                            *word_count,
                            options.corpus_words_vector.clone()
                        ).into_iter()
                        .map(|w| w.green())
                        .collect::<Vec<ColoredString>>();

                    // Format the result into a joined string
                    //// Colored's API does not allow this using a join() method like std::String
                    let mut phrase_str: String = String::new();
                    for (i,s) in phrase_vec.iter().enumerate() {
                        phrase_str = match i {
                            0 => format!("{}{}", phrase_str, s),
                            _ => format!("{}{}{}", phrase_str, word_separator.blue() ,s),
                        };
                    }

                    format!(
                        "{}{}{}{}",
                        key_alpha.red(),
                        key_numer.blue(),
                        options.token_sparator.magenta(),
                        phrase_str
                    )
                }
                //Styles::CorrectHorse { word_separator, word_count } => panic!("Not Implemented!"),
                //Styles::BasicRand { length } => panic!("Not Implemented!"),
                _ => panic!("Not Implemented!"), //? Safety catch
            };
            break;
        }
        //let i_options = options.clone();
        //let i_style = style.clone();
        return Password {
            data: data.clone(),
            entropy: calculate_entropy(data.len() as f64, ascii_sym_string.len() as f64).trunc(), //TODO: Softcode the value of pool_size
            //options: i_options,
            //style: i_style,
        };
    }
    pub fn hash_argon2(&self) -> String {
        hash_utils::hash_argon2(self.data.clone())
    }
    pub fn hash_sha512(&self) -> String {
        hash_utils::hash_sha512(self.data.clone())
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

// Calculate the entropy of the input values using E = log2(R^L)
pub fn calculate_entropy(data_size: f64, pool_size: f64) -> f64 {
    f64::powf(pool_size, data_size).log2()
}

// Randomly select a subset from the set and return the subset
pub fn random_pool_choice(len: usize, set: Vec<String>) -> Vec<String> {
    let mut subset: Vec<String> = Vec::with_capacity(len);
    while subset.len() < subset.capacity() {
        let sel_idx: usize = thread_rng().gen_range(0..set.len());
        let element = set[sel_idx].clone();
        if element != "" { subset.push(element) }; //? The if statement here resolves a bug where blank characters are inserted.
    }
    return subset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_pool_choice() {
        // Test that the output of random subset constructor
        //   returns a vector of the expected length.
        for _ in 0..100 {
            let len = thread_rng().gen_range(0..50);
            let pool = crate::ASCII_SYMBOLS
                .split("")
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let genreturn = random_pool_choice(len, pool);
            assert!(genreturn.len() == len);
            // Ensure that none of the returned items are a blank string
            for i in genreturn {
                assert!(i != "".to_string())
            }
        }
    }
}
