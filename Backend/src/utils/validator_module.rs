use regex::Regex;
use validator::ValidationError;

pub fn valid_email(email: &str) -> Result<(), ValidationError>{
    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})")
        .map_err(|_|{
            tracing::error!("Failed to create Regex for email_validation")
        }).expect("failed to map error for email_regex");
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