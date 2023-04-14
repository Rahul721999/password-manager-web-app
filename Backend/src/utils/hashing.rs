use bcrypt::{DEFAULT_COST,verify, hash_with_salt};
use rand::{Rng, thread_rng};

pub async fn hash_pass(password: &str) -> Result<String, bcrypt::BcryptError> {
    // should also check if the password matches the requirement
    let mut rng = thread_rng();
    let salt = rng.gen::<[u8; 16]>();
    let hashed = hash_with_salt(password,  DEFAULT_COST, salt).expect("Failed to hash password");
    Ok(hashed.to_string())
}


pub async fn verify_pass(password: &str, hashed_pass: &str) -> bool{
    match verify(password, hashed_pass) {
        Ok(true) => true,
        _ => false,
    }
}