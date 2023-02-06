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

fn main() {
    // Generate a fancy horse password
    // ===============================

    // -{ Build application configurations
    let appopts = ApplicationOptions::new(None, Some("/workspace/words.txt"));
    //println!("{}", appopts);

    // -{ Display a collection of generated passwords
    for _ in 0..5 {
        // Generate a password struct
        let password = Password::new(appopts.password_options.clone(), appopts.password_style.clone());
        //print!("\nPASS: {}\nENTR: {}\n", password.data, password.entropy)
        println!("{}", password)
    }

    print!("\nEND OF LINE\n")
}
