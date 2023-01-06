use std::fs;
use rand::{thread_rng,Rng};

// Entropy calculation coverage depends on the format, charset & word list length
// ? Different formats will calculate entropy differently
//    For example a fancy-horse password will have a different bit-entropy 
//     for the prefix/postfix, word list, and separator characters.
//TODO: --online {Check against the HIBP api for leaks??}
// ! Worst-case Scenario: Attacker is aware of format, charset, and wordlist used.
pub fn get_bit_entropy(length: f32, pool: f32) -> f32 {
  // Bit-Entropy calculation (E=L*P.log2())
  let entropy: f32 = length as f32 * (pool as f32).log2();
  return entropy
}

pub fn read_file(path: std::path::PathBuf) -> String {
  return match fs::read_to_string(path.as_path()) {
      Err(why) => panic!("couldn't read {}: {}", path.display(), why),
      Ok(data) => data,
  };
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
