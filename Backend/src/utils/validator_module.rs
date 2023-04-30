use validator::ValidationError;
use crate::email_regex;

pub fn valid_email(email: &str) -> Result<(), ValidationError>{
    
    if email_regex.is_match(email) {
        Ok(())
    } else {
        Err(ValidationError::new("Invalid Email"))
    }
}

pub fn valid_password(password : &str)-> Result<(), ValidationError>{
    let mut has_whitespace = false;
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;
    let mut has_special = false;

    for c in password.chars() {
        has_whitespace |= c.is_whitespace();
        has_lower |= c.is_lowercase();
        has_upper |= c.is_uppercase();
        has_digit |= c.is_digit(10);
        has_special |= c.is_ascii_punctuation();
    }
    if !has_whitespace && has_upper && has_lower && has_digit && has_special && password.len() >= 8 {
        Ok(())
    } else {
        return Err(ValidationError::new("Password is not strong enough"));
    }
}