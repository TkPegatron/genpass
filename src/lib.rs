use rand::{thread_rng, Rng};
use std::fs;
use std::path::PathBuf;

pub fn rng_subst(len: usize, pool: Vec<String>) -> Vec<String> {
    // Randomly select a subset from the set and return the subset
    let mut subset: Vec<String> = Vec::with_capacity(len);

    while subset.len() < subset.capacity() {
        let idx: usize = thread_rng().gen_range(0..pool.len());
        let element = pool[idx].clone();
        subset.push(element)
    }

    return subset;
}

pub fn str_vectorize_filter(target: &str, delimiter: &str) -> Vec<String> {
    // Split a str by delimiter and return it as a vector of strings
    // This function also filters for empty strings
    target
        .split(delimiter)
        .filter(|s| ![""].contains(s))
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

pub fn read_file(path: std::path::PathBuf) -> String {
    return match fs::read_to_string(path.as_path()) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why),
        Ok(data) => data,
    };
}

// Entropy calculation coverage depends on the format, charset & word list length
// ? Different formats will calculate entropy differently
//    For example a fancy-horse password will have a different bit-entropy
//     for the prefix/postfix, word list, and separator characters.
//TODO: --online {Check against the HIBP api for leaks??}
// ! Worst-case Scenario: Attacker is aware of format, charset, and wordlist used.
pub fn get_bit_entropy(length: f32, pool: f32) -> f32 {
    // Bit-Entropy calculation (E=L*P.log2())
    let entropy: f32 = length as f32 * (pool as f32).log2();
    return entropy;
}

//================================================================================================

fn read_to_vector(path: PathBuf) -> Vec<String> {
    match fs::read_to_string(&path) {
        Ok(data) => str_vectorize_filter(data.as_str(), "\n"),
        Err(why) => {
            panic!("Could not read {}: {}", path.display(), why);
        }
    }
}

pub fn load_words(corpus: Option<String>) -> Vec<String> {
    let basedirs = xdg::BaseDirectories::with_prefix("genpass").unwrap();
    let file_var = match std::env::var_os("GENPASS_CONFIG_HOME") {
        Some(val) => PathBuf::from(val),
        None => PathBuf::from(""),
    };
    match corpus {
        Some(val) => {
            let file_arg = PathBuf::from(val);
            if file_arg.is_file() {
                read_to_vector(file_arg)
            } else {
                panic!("Specified corpus is not a file.")
            }
        }
        None => {
            if file_var.is_file() {
                read_to_vector(file_var)
            } else {
                match basedirs.find_data_file("words.txt") {
                    Some(val) => read_to_vector(val),
                    None => panic!("Could not find corpus file in path"),
                }
            }
        }
    }
}
