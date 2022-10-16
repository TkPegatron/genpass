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
    #[arg(long, default_value = ":", help="Character to separator the words of the phrase")] passphrase_separator: String,
    #[arg(long, default_value = "@", help="Character to separator the key from the phrase")] key_separator: String,
    #[arg(short = 'n', long, default_value = "1", help = "Number of passwords to generate")] number_of_passwords: u32,
    #[arg(short = 's', long, default_value = "fancy-horse", help = "Specify the style of password to generate")] password_style: String
}

fn main() {

    let config = PasswordConfig::parse();

    let horse_words = utils::get_correct_horse_words(&config);

    let mut password: String = String::new();

    for _ in 0..config.number_of_passwords {
        match config.password_style.as_str() {
            "fancy-horse" => {password = styles::fancy_correct_horse(&config, &horse_words)}
            "correct-horse" => {password = styles::correct_horse(&config, &horse_words)}
            "random" => {password = styles::random_ascii(&config)}
            _ => {println!("Error: Unknown Style `{}`", config.password_style); break;}
        }
        println!("{}", password)
    }
}
