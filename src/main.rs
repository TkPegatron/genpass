// This program will generate passwords in the following form.
//
// sddfg16162:Linking.Information.Serially
//  │    │   │└ Passphrase ──────┼───────┘
//  │    │   └ Separator         └ Passphrase_Separator
//  │    └ Key_Numbers
//  └ Key_Chars
//

// Considerations for controling the generator behavior
//   Minimum length of words gathered from wordlist
//   Minimum guestimated entropy acceptable to user
//   Characters used as token seperators
//   Number of words desired by user
//   Style of password desired by user

// Considerations for controlling the application behavior
//   Where the configuration file is located
//   Where the wordlist file is located

// In-memory data needed
//   Filtered word list

//TODO: Format further and organize, I am sure there is a better organization structure

use std::{path::PathBuf};
use regex::{escape,Regex};

use generator::{Options, Password};

pub mod utils;
//pub mod configuration;
pub mod generator;
pub mod hash_utils;

pub const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
pub const NUMBERS: &str = "0123456789";
pub const SYMBOLS: &str = ":;<=>?@!\"#$%&'()*+,-./[\\]^_`{|}~";
pub const ASCII_SYMBOLS: &str =
    "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

fn main() {
    // Generate a fancy horse password
    // ===============================

    // -{ Setup the corpus
    let min_corp_word_len: u32 = 4; //? Allow this to be defined perhaps.

    // =={ Prepare a regex filter used to enforce minimum word length
    let re_word_length_filter: Regex = Regex::new(format!(
        r"[{}]{{{}}}",
        escape(ASCII_SYMBOLS),
        min_corp_word_len
    ).as_str()).unwrap();

    // =={ Load the wordlist from file
    let words_file: PathBuf = PathBuf::from("/workspace/words.txt");
    let corpus_words_vector = utils::read_file(words_file)
        .split("\n")
        .filter(|w| {re_word_length_filter.is_match(w)})
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    println!("///Words file loaded....");

    // -{ Define the style
    let password_style: generator::Styles = generator::Styles::FancyHorse {
        word_separator: "_-_".to_owned(),
        word_count: 3,
        key_num: 5,
        key_alp: 5,
    };

    // -{ Define the config
    let password_options: Options = Options {
        corpus_words_vector: corpus_words_vector.clone(),
        ..Default::default() //? Use the default trait to fill unspeced fields
    };

    // -{ Display a collection of generated passwords
    for _ in 0..5 {
        // Generate a password struct
        let password = Password::new(password_options.clone(), password_style.clone());
        //print!("\nPASS: {}\nENTR: {}\n", password.data, password.entropy)
        println!("{}", password)
    }

    print!("\nEND OF LINE\n")
}

//fn load_wordlist_pool(&self) -> Vec<String> {
//    let min_corp_word_len: u32 = 4; //? Allow this to be defined perhaps.
//    // Prepare a regex filter used to enforce minimum word length
//    let re_word_length_filter: Regex = Regex::new(format!(
//      r"[{}{}{}{}]{{{}}}",UPPERCASE,LOWERCASE,
//      NUMBERS,SYMBOLS,min_corp_word_len
//    ).as_str()).unwrap();
//    // build vector from wordlist file
//    let wordlist_pool = GenpassConfig::read_file(&self.wordlist_file)
//      .split("\n")
//      .filter(|word| {re_word_length_filter.is_match(word)})
//      .collect::<Vec<&str>>();
//    // Return vector
//    return wordlist_pool
//}

//fn config_read_test(config: configuration::ApplicationOptions) {
//    println!("DBG: Loading configuration data from file...\n");
//
//    // Read config file to a String
//    let ser_config_data: String = utils::read_file(config.path_config);
//
//    // Deserialize config data
//    let config_data: configuration::PasswordOptions = ron::de::from_str(&ser_config_data).unwrap();
//
//    // Display Serialization
//    println!("{}\n", utils::display_encoded_config(&config_data));
//}

//fn config_default_test(config: configuration::ApplicationOptions) {
//    println!("DBG: Loading default configuration data...\n");
//
//    // Instanciate deserialized config
//    let appconfig: configuration::ApplicationOptions = configuration::ApplicationOptions::default();
//
//    // Display Serialized defaults
//    println!("{}\n", utils::display_encoded_config(&configuration::PasswordOptions::default()));
//}
