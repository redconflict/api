use argon2::{
  password_hash::{rand_core::OsRng, SaltString},
  Argon2, PasswordHasher,
};

pub fn hash(raw: String) -> Result<String, argon2::password_hash::Error> {
  // Generating crypto-random salt. 
  let salt = SaltString::generate(&mut OsRng);
  let hash = Argon2::default()
    .hash_password(raw.as_bytes(), &salt)?
    .to_string();

  Ok(hash)
}
