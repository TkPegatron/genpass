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

use generator::{Password, Options};

pub mod utils;
//pub mod configuration;
pub mod generator;
pub mod hash_utils;

fn main() {

    
    // Generate a fancy horse password
    // ===============================
    // -{ Setup the corpus
    let corpur_words_vector: Vec<String> = "One\nTwo\nthree\nfour\nfive\nsix\nseven\neight\nnine\nten\ntwenty\nthirty\nfourty"
        .to_string().split("\n").into_iter().map(|s| s.to_string()).collect::<Vec<String>>();

    // -{ Define the style
    let pasword_style: generator::Styles = generator::Styles::FancyHorse { 
        word_separator: ":".to_owned(), word_count: 3, key_num: 6, key_alp: 4 
    };

    // -{ Define the config
    let password_options: Options = Options {
        corpus_words_vector: corpur_words_vector,
      ..Default::default() //? Use the default trait to fill unspeced fields
    };

    // -{ Display a collection of generated passwords
    for _ in 0..5 {
        let password = Password::new(&password_options, &pasword_style);
        
        let hash = &password.hash_sha512();
        //let password_hash = &password.hash_sha512();
        //let password_entropy = &password.entropy;
        print!(
            "\nPassword: {}\nHash: {}\nEntropy: {}\n",
            password.data,hash,password.entropy
        );
    }



    // Instanciate configuration data
    //let configuration = configuration::PasswordOptions::new();

    //Print the configuration
    //println!("{}",utils::display_encoded_config(&configuration));

    //println!("{}",
    //    utils::rng_alphanumeric(21, "qwertyuiopasdfghjklzxcvbnm,.1234567890")
    //);

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
