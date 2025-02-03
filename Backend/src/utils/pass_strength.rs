use crate::AppError;
use passwords::{analyzer, scorer, PasswordGenerator};
use tracing::{error, info};

pub fn analyze_pass(password: &str) -> Result<(), AppError> {
    let res = analyzer::analyze(password);
    let score = scorer::score(&res) as i64;
    match score {
        0..=80 => {
            let pass_suggestion = match generate_pass() {
                Ok(p) => p,
                Err(err) => return Err(err),
            };
            info!("Weak password");
            Err(AppError::AuthError(format!(
                "Weak Password, You can use something like: {}",
                pass_suggestion
            )))
        }
        81..=100 => Ok(()),
        _ => {
            error!("❌ Password analyze error");
            Err(AppError::InternalServerError(
                "Password analyze failed".to_string(),
            ))
        }
    }
}

pub fn generate_pass() -> Result<String, AppError> {
    let pg = PasswordGenerator {
        length: 10,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: false,
        exclude_similar_characters: true,
        strict: true,
    };
    match pg.generate_one() {
        Ok(pass) => Ok(pass),
        Err(err) => {
            error!("❌ Failed to generate password: {}", err);
            Err(AppError::InternalServerError(
                "Failed to generate password".to_string(),
            ))
        }
    }
}
