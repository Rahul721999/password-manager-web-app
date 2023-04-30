use bcrypt::{DEFAULT_COST,verify, hash_with_salt};
use rand::{Rng, thread_rng};
use crate::AppError;

pub async fn hash_pass(password: &str) -> Result<String, AppError> {
    // should also check if the password matches the requirement
    let mut rng = thread_rng();
    let salt = rng.gen::<[u8; 16]>();
    if let Ok(hashed) = hash_with_salt(password,  DEFAULT_COST, salt){
        return Ok(hashed.to_string());
    } return Err(AppError::InternalServerError(format!("Password couldn't be hashed")));
}


pub async fn verify_pass(password: &str, hashed_pass: &str) -> bool{
    match verify(password, hashed_pass) {
        Ok(true) => true,
        _ => false,
    }

}