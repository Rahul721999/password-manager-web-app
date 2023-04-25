use passwords::{analyzer, scorer};
use crate::AppError;

pub fn analyze_pass(password : &str)-> Result<(), AppError>{
    let res = analyzer::analyze(password);
    let score = scorer::score(&res) as i64;
    match score{
        0..=80 =>{return Err(AppError::AuthError(format!("Week Password")))},
        81..=100=>{return Ok(())},
        _ => {
            tracing::error!("❌ Password analyze error");
            Err(AppError::InternalServerError(format!("Password analyze failed")))}
    }
}