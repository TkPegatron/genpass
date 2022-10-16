
use std::fs;
use clap::Parser;
use regex::Regex;
use rand::{thread_rng,Rng};

pub const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
pub const NUMBERS: &str = "0123456789";
pub const SYMBOLS: &str = ")(*&^%$#@!~";

#[derive(Parser, Debug)]
#[command(author = "Elliana Perry <elliana.perry@gmail.com>")]
pub struct PasswordConfig {
    #[arg(long = "pl", default_value = "2", help="Number of words in passphrase section")]
    passphrase_length: usize,
    #[arg(long = "kll", default_value = "4", help="Length of letters in key section")]
    key_let_length: u32,
    #[arg(long = "knl", default_value = "5", help="Length of numbers in key section")]
    key_num_length: u32,
    #[arg(short = 'P', long = "passphrase-seperator", default_value = ":", help="Character to seperate the words of the phrase")]
    passphrase_seperator: String,
    #[arg(short = 'K', long = "key-seperator", default_value = "@", help="Character to seperate the key from the phrase")]
    key_seperator: String,
    #[arg(short = 'n', long = "number", default_value = "1", help = "Number of passwords to generate")]
    number_of_passwords: u32
}

fn main() {

    let config = PasswordConfig::parse();

    let key_let_length: u32 = config.key_let_length;
    let key_num_length: u32 = config.key_num_length;

    let key_seperator: &str = config.key_seperator.as_str();

    let passphrase_length: usize = config.passphrase_length;
    let passphrase_seperator: &str = config.passphrase_seperator.as_str();

    let minimum_passphrase_word_length: u32 = 4;

    let word_corpus_string: String = read_file("/home/eperry/Downloads/english-words/words.txt");

    let re_object: Regex = Regex::new(
        format!(r"[{}{}{}{}]{{{}}}",
                UPPERCASE,LOWERCASE,NUMBERS,SYMBOLS,
                minimum_passphrase_word_length.to_string()
            ).as_str()
        ).unwrap();

    let word_corpus_vector: &Vec<&str> = &word_corpus_string.split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .filter(|w| { re_object.is_match(w) })
        .collect::<Vec<&str>>()
    ;

    for _ in 0..config.number_of_passwords {
        // Build a Vec<str> for the passphrase
        let mut words: Vec<&str> = Vec::with_capacity(passphrase_length);
        for _ in 0..passphrase_length {
            let num: usize = thread_rng().gen_range(0..word_corpus_vector.len());
            let word: &str = word_corpus_vector[num];
            words.push(word);
        }

        //let mut charset = String::from("");
        //charset.push_str(SYMBOLS);
        //charset.push_str(UPPERCASE);
        //charset.push_str(LOWERCASE);
        //charset.push_str(NUMBERS);

        println!("{}{}{}{}",
            rng_alphanumeric(key_let_length, LOWERCASE),
            rng_alphanumeric(key_num_length, NUMBERS),
            key_seperator,
            words.join(passphrase_seperator)
        )
    }
}

//fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>())
//}

fn read_file(fp: &str) -> String {
    // This method opens a file in read only mode and stores the data in a string
    let path = std::path::Path::new(fp);
    return match fs::read_to_string(path) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why),
        Ok(data) => data,
    };
}

fn rng_alphanumeric(length: u32, charset: &str) -> String {
    let char_vec: Vec<char> = charset.chars().collect();
    return (0..length).map(|_| {
        let idx = thread_rng().gen_range(0..char_vec.len());
        char_vec[idx] as char
    }).collect();
}

//fn longest_contiguous_sequence(wordlist: &[String]) {
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
