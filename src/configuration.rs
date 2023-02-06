use crate::generator;
use crate::utils;
use directories_next::{BaseDirs, ProjectDirs, UserDirs};
use regex::{escape, Regex};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

// Structure containing options that control the application
#[derive(Debug, Deserialize, Serialize)]
pub struct ApplicationOptions {
    config_file_path: PathBuf,
    corpus_file_path: PathBuf,
    pub password_style: generator::Styles,
    pub password_options: generator::Options,
}

impl ApplicationOptions {
    pub fn new(fp_config: Option<&str>, fp_corpus: Option<&str>) -> ApplicationOptions {
        //TODO: Some of this can proably be simplified or optimized
        let config_home_path: PathBuf = get_default_homedir();

        let config_file_path: PathBuf = match fp_config {
            Some(fp) => fp.into(),
            None => config_home_path.join("config.ron")
        };

        let corpus_file_path: PathBuf = match fp_corpus {
            Some(fp) => fp.into(),
            None => config_home_path.join("words.txt")
        };

        // Read configuration file
        let config_data: ApplicationOptions = match fs::read_to_string(config_file_path.as_path()) {
            // Deserialize the configuration if it is available
            Ok(data) => ron::de::from_str(&data).unwrap(),
            Err(why) => {
                //TODO: Use a proper logging library
                println!("Could not read {}: {}. Using default configuration...", 
                    config_file_path.display(), why
                );
                ApplicationOptions::default()
            }
        };

        let corpus_words_vector = ApplicationOptions::load_corpus(corpus_file_path);
        let opt_default: generator::Options = generator::Options::default();

        ApplicationOptions {
            password_options: generator::Options {
                corpus_words_vector: Some(corpus_words_vector),
                corpus_numeral_vector: match config_data.password_options.corpus_numeral_vector {
                    Some(_) => { config_data.password_options.corpus_numeral_vector }
                    None => { opt_default.corpus_numeral_vector }
                },
                corpus_alpha_vector: match config_data.password_options.corpus_alpha_vector {
                    Some(_) => { config_data.password_options.corpus_alpha_vector }
                    None => { opt_default.corpus_alpha_vector }
                },
                ..config_data.password_options
            },
            ..config_data
        }
    }

    fn load_corpus(corpus_file_path: PathBuf) -> Vec<String> {
        // =={ Prepare a regex filter used to enforce minimum word length
        //let min_corp_word_len: u32 = 4; //? Allow this to be defined perhaps.
        //let re_word_length_filter: Regex = Regex::new(
        //    format!(
        //        r"[{}]{{{}}}",
        //        escape(crate::ASCII_SYMBOLS),
        //        min_corp_word_len
        //    )
        //    .as_str(),
        //)
        //.unwrap();
        // =={ Load the wordlist from file and construct the word corpus
        return utils::read_file(corpus_file_path)
            .split("\n")
            //? This filters by word length
            //.filter(|w| re_word_length_filter.is_match(w))
            //? This filters blank lines
            .filter(|s| {![""].contains(s)})
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
    }
}

impl Default for ApplicationOptions {
    fn default() -> ApplicationOptions {
        // =={ Get default filepaths
        let config_home_path: PathBuf = get_default_homedir();
        let config_file_path: PathBuf = config_home_path.join("config.ron");
        let corpus_file_path: PathBuf = config_home_path.join("words.txt");

        let corpus_words_vector = "
            abrase
            abrased
            abraser
            abrash
            abrasing
            abrasiometer
            abrasion
            abrasions
            abrasive
            abrasively
            abrasiveness
            abrasives
            abrastol
        "
        .split("\n")
        .filter(|s| {![""].contains(s)})
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

        // =={ Instanceate a default configuration
        ApplicationOptions {
            config_file_path: config_file_path,
            corpus_file_path: corpus_file_path,
            password_style: generator::Styles::FancyHorse {
                word_separator: "_-_".to_owned(),
                word_count: 3,
                key_num: 5,
                key_alp: 4,
            },
            password_options: generator::Options {
                corpus_words_vector: Some(corpus_words_vector),
                ..generator::Options::default()
            },
        }
    }
}

impl std::fmt::Display for ApplicationOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", display_encoded_config(&self))
    }
}


// cofigure the default application paths
fn get_default_homedir() -> PathBuf {
    // Get XDG compliant project directories
    let project_dirs = ProjectDirs::from("xyz", "zynthovian", "genpass").unwrap();
    // Allow the environment variable take precedence over xdg path
    let config_home_path: PathBuf = match env::var_os("GENPASS_CONFIG_HOME") {
        Some(val) => PathBuf::from(val),
        None => PathBuf::from(project_dirs.config_dir()),
    };
    // Deserialize configuration data
    return config_home_path;
}

pub fn display_encoded_config<T>(data_struct: &T) -> String
where T: ?Sized + serde::ser::Serialize {
    // Instanciate Pretty Configuration
    let prettyconfig = ron::ser::PrettyConfig::new()
        .depth_limit(2)
        .separate_tuple_members(true)
        .enumerate_arrays(true)
        .struct_names(true);

    // Return formated data
    return ron::ser::to_string_pretty(
        &data_struct,
        prettyconfig
    ).expect("Serialization failed");
}
