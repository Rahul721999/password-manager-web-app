use crate::AppError;
use bcrypt::{hash_with_salt, verify, DEFAULT_COST};
use rand::{thread_rng, Rng};

pub async fn hash_pass(password: &str) -> Result<String, AppError> {
    // should also check if the password matches the requirement
    let mut rng = thread_rng();
    let salt = rng.gen::<[u8; 16]>();
    if let Ok(hashed) = hash_with_salt(password, DEFAULT_COST, salt) {
        return Ok(hashed.to_string());
    }
    Err(AppError::InternalServerError(
        "Password couldn't be hashed".to_string(),
    ))
}

pub async fn verify_pass(password: &str, hashed_pass: &str) -> bool {
    matches!(verify(password, hashed_pass), Ok(true))
}
