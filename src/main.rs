mod utils;
mod styles;
use clap::Parser;

pub const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
pub const NUMBERS: &str = "0123456789";
pub const SYMBOLS: &str = ")(*&^%$#@!~";

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct PasswordConfig {
    #[arg(long, default_value = "2", help="Number of words in passphrase section")] passphrase_length: usize,
    #[arg(long, default_value = "4", help="Length of letters in key section")] key_let_length: u32,
    #[arg(long, default_value = "5", help="Length of numbers in key section")] key_num_length: u32,
    #[arg(long, default_value = ":", help="Character to seperate the words of the phrase")] passphrase_seperator: String,
    #[arg(long, default_value = "@", help="Character to seperate the key from the phrase")] key_seperator: String,
    #[arg(long, default_value = "1", help = "Number of passwords to generate")] number_of_passwords: u32
}

fn main() {

    let config = PasswordConfig::parse();

    let horse_words = utils::get_correct_horse_words(&config);

    for _ in 0..config.number_of_passwords {
        let passphrase = styles::fancy_correct_horse(&config, &horse_words);
        println!("{}", passphrase)
    }
}
