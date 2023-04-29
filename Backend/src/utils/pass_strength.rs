use passwords::{analyzer, scorer, PasswordGenerator};
use crate::AppError;

pub fn analyze_pass(password : &str)-> Result<(), AppError>{
    let res = analyzer::analyze(password);
    let score = scorer::score(&res) as i64;
    match score{
        0..=80 =>{
            let pass_suggestion = match gen_pass(){
                Ok(p) => p,
                Err(err) => return Err(err), 
            };
            return Err(AppError::AuthError(format!("Weak Password, You can use something like: {}",pass_suggestion)))},
        81..=100=>{return Ok(())},
        _ => {
            tracing::error!("❌ Password analyze error");
            Err(AppError::InternalServerError(format!("Password analyze failed")))}
    }
}

pub fn gen_pass() -> Result<String, AppError>{
    let pg = PasswordGenerator{
        length: 8,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };
    match pg.generate_one(){
        Ok(pass) => return Ok(pass),
        Err(err) => {
            tracing::error!("❌ Failed to generate password: {}", err);
            return Err(AppError::InternalServerError(format!("Failed to generate password")))}
    }
}