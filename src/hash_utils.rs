use sha_crypt::{Sha512Params, sha512_simple};

use scrypt::{
  password_hash::{
      rand_core::OsRng,
      PasswordHasher, SaltString
  },
  Scrypt
};

use argon2::{
  Argon2
};

pub fn hash_argon2(password: String) -> String {
  let salt: SaltString = SaltString::generate(&mut OsRng);
  let password_hash = Argon2::default().hash_password(&password.as_bytes(), &salt).expect("").to_string();
  return  password_hash;
}

pub fn hash_scrypt(password: String) -> String {
  let salt: SaltString = SaltString::generate(&mut OsRng);
  return Scrypt.hash_password(password.as_bytes(), &salt).unwrap().to_string();
}

pub fn hash_sha512(password: String) -> String {
  let params: Sha512Params = Sha512Params::new(10_000).expect("RandomError!");
  let hashed_password: String = sha512_simple(password.as_str(), &params)
    .expect("Should not fail");
  return hashed_password
}

#[cfg(test)]
mod tests {

  use super::*;
  use sha_crypt::sha512_check;
  use scrypt::{
    password_hash::{
      PasswordHash, PasswordVerifier
    }
  };

  const TEST_PASSWORD: &str = "laqc78418@theophagous:half-dozen";

  #[test]
  fn test_argon2_hash() {
    let password_hash_string = hash_argon2(TEST_PASSWORD.to_string());
    let password_hash = PasswordHash::new(&password_hash_string).unwrap();
    assert!(Argon2::default().verify_password(TEST_PASSWORD.as_bytes(), &password_hash).is_ok())
  }

  #[test]
  fn test_scrypt_hash() {
    let password_hash_string = hash_scrypt(TEST_PASSWORD.to_string());
    let password_hash = PasswordHash::new(&password_hash_string).unwrap();
    assert!(Scrypt.verify_password(TEST_PASSWORD.as_bytes(), &password_hash).is_ok());
  }

  #[test]
  fn test_sha512_hash() {
    assert!(sha512_check(TEST_PASSWORD, hash_sha512(TEST_PASSWORD.to_string()).as_str()).is_ok());
  }
}
