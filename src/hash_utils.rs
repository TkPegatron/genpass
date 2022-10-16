use sha_crypt::{Sha512Params, sha512_simple, sha512_check};
use scrypt::{ScryptParams,scrypt_simple};
use argon2::{
  password_hash::{
    rand_core::OsRng,
    PasswordHash, PasswordHasher,SaltString
  },
  Argon2
};
use colored::*;
use rand::{thread_rng,Rng};

pub fn hash_argon2(password: String) -> String {
  let salt = SaltString::generate(&mut OsRng);
  let password_hash = Argon2::default().hash_password(&password.as_bytes(), &salt).expect("").to_string();
  return  password_hash;
}

pub fn hash_scrypt(password: String) -> String {
  let params = ScryptParams::new(15, 8, 1).expect("");
  return scrypt_simple(password.as_str(), &params).expect("");
}

pub fn compile_test(password: String) -> String {
  return String::new()
}

pub fn hash_sha512(password: String) -> String {
  let params = Sha512Params::new(10_000).expect("RandomError!");
  let hashed_password = sha512_simple(password.as_str(), &params)
    .expect("Should not fail");
  assert!(sha512_check(password.as_str(), &hashed_password).is_ok());
  return hashed_password
}
