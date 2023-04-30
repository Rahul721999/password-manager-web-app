use crate::AppError;

pub async fn encrypt(value : &str) -> Result<String, AppError>{
    let res = value.to_string();

    // error!("❌Failed to encrypt pass: {}", err);
    return Ok(res)
}

pub async fn decrypt(value : &str) -> Result<String,AppError>{
    let res = value.to_string();
    
    // error!("❌Failed to encrypt pass: {}", err);
    return Ok(res)
}