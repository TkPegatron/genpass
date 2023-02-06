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
pub mod utils;
pub mod configuration;
use clap::Parser;
pub mod generator;
pub mod hash_utils;
use generator::Password;
use configuration::ApplicationOptions;

pub const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
pub const NUMBERS: &str = "0123456789";
pub const SYMBOLS: &str = ":;<=>?@!\"#$%&'()*+,-./[\\]^_`{|}~";
pub const ASCII_SYMBOLS: &str =
    "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    config: Option<String>,
    #[arg(long)]
    corpus: Option<String>,
    #[arg(short,long)]
    range: Option<u32>
}

fn main() {
    // Generate a fancy horse password
    // ===============================
    let args = Args::parse();

    // -{ Build application configurations
    let appopts = ApplicationOptions::new(args.config, args.corpus);
    //println!("{}", appopts);

    // -{ Display a collection of generated passwords
    let mut passwords: Vec<String> = Vec::new();
    match args.range {
        Some(i) => {
            for _ in 0..i {
                passwords.push(
                    Password::new(
                        appopts.password_options.clone(),
                        appopts.password_style.clone()
                    ).data
                )
            }
        },
        None => {
            passwords.push(
                Password::new(
                    appopts.password_options.clone(),
                    appopts.password_style.clone()
                ).data
            )
        }
    };

    println!("{}", passwords.join("\n"));
}
