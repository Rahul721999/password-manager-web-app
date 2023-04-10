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

pub fn valid_password(pass : &str)-> Result<(), ValidationError>{
    let pass_regex = Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)[a-zA-Z\d]{8,}$")
    .map_err(|_|{
        tracing::error!("Failed to create Regex for pass_validation")
    }).expect("failed to map error for email_regex");
    if pass_regex.is_match(pass) {
        Ok(())
    } else {
        Err(ValidationError::new("Invalid pass"))
    }
}