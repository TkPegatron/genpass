use std::fs;
use std::env;
use std::path::{Path,PathBuf};
use directories_next::{BaseDirs, UserDirs, ProjectDirs};
use serde::{Deserialize, Serialize};

pub const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
pub const NUMBERS: &str = "0123456789";
pub const SYMBOLS: &str = ")(*&^%$#@!~";

// Structure containing modifiers that alter the
//   behavior of the generator for deriving the value of a token (Key or Phrase)
#[derive(Debug, Deserialize, Serialize)]
struct TokenOptions {
  length: Option<usize>,
  separator: Option<String>,
  generator_override: Option<String>,
}

// Structure containing options that control the generator
#[derive(Debug, Deserialize, Serialize)]
pub struct PasswordOptions {
  target_entropy: Option<f32>,
  token_separator: Option<String>,
  option_key: TokenOptions,
  option_phrase: TokenOptions,
  option_corpus: TokenOptions,
}

impl PasswordOptions {

  //TODO: I am a little confused about what functions should do what in implementations

  pub fn new() -> PasswordOptions {
    // Get default ApplicationOptions, this gives us the path to the config file
    //TODO: should I break-out the function to get this info???
    let mut app_options = ApplicationOptions::default();
    // Load configuration data from file
    //TODO: Rename these config_path, corpus_path
    //println!("DBG: Loading configuration data from file...\n");
    let ser_config_data: String = crate::utils::read_file(app_options.path_config);
    // Deserialize configuration data
    Self::deserialize(ser_config_data)
  }

  // Deserialize ron configuration data
  pub fn deserialize(data: String) -> PasswordOptions {
    return ron::de::from_str(&data).unwrap()
  }
}

impl Default for PasswordOptions {
  fn default() -> PasswordOptions {
    PasswordOptions {
      target_entropy: Some(0.0),
      token_separator: Some(String::from(":")),
      option_key: TokenOptions {
        length: Some(9),
        separator: None,
        generator_override: None,
      },
      option_phrase: TokenOptions {
        length: Some(4),
        separator: Some(String::from(".")),
        generator_override: None,
      },
      option_corpus: TokenOptions {
        length: Some(4),
        separator: None,
        generator_override: None,
      },
    }
  }
}

// Structure containing options that control the application
#[derive(Debug, Deserialize, Serialize)]
pub struct ApplicationOptions {
  pub path_config: PathBuf,
  pub path_corpus: PathBuf,
}

impl Default for ApplicationOptions {
  fn default() -> ApplicationOptions {
    // Get XDG compliant project directories
    let project_dirs = ProjectDirs::from(
      "xyz","zynthovian","genpass"
    ).unwrap();

    // Allow the environment variable take precedence over xdg path
    let config_home_path: PathBuf = match env::var_os("GENPASS_CONFIG_HOME") {
      Some(val) => PathBuf::from(val),
      None => PathBuf::from(project_dirs.config_dir())
    };

    // Retun a default ApplicationOptions struct
    ApplicationOptions {
      path_config: config_home_path.join("config.ron"),
      path_corpus: config_home_path.join("words.txt"),
    }
  }
}
