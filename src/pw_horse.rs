use crate::constants;

use colored::{ColoredString, Colorize};
use constants::{ASCII_SYMBOLS, LOWERCASE, NUMBERS, SYMBOLS, UPPERCASE};
use genpass::{rng_subst, str_vectorize_filter};

pub struct Password {
    pub data: String,
    pub entropy: f64,
}

impl Password {
    pub fn new(words: Vec<String>) -> Password {
        let pfxkey = format!(
            "{}",
            rng_subst(
                9,
                str_vectorize_filter(
                    format!("#$%&!+-@~{}{}{}", UPPERCASE, LOWERCASE, NUMBERS).as_str(),
                    ""
                )
            )
            .join("")
        );
        let phrase_vec = rng_subst(3, words)
            .iter()
            .map(|s| {
                // break the string up and recompile it
                let mut vs: Vec<char> = s.chars().collect();
                vs[0] = vs[0].to_uppercase().nth(0).unwrap();
                vs.iter().collect::<String>().green()
            })
            .collect::<Vec<ColoredString>>();

        let mut phrase: String = String::new();
        for (i, s) in phrase_vec.iter().enumerate() {
            phrase = match i {
                0 => {
                    format!("{}{}", phrase, s)
                }
                _ => {
                    format!("{}{}{}", phrase, ".".blue(), s)
                }
            }
        }

        return Password {
            data: format!("{}{}{}", pfxkey.red(), ":".magenta(), phrase),
            entropy: 55.3,
        };
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}
