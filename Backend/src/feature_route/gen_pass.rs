use crate::{AppError, MyMiddleware, generate_pass};
use actix_web::{HttpResponse};


#[tracing::instrument(
	name="🚩 User Gen-Pass request"
	skip_all
)]
pub async fn generate(
    _mid: MyMiddleware,
)-> Result<HttpResponse, AppError>{
    let pass_suggestion = match generate_pass(){
        Ok(p) => p,
        Err(err) => return Err(err), 
    };
    Ok(HttpResponse::Ok().json(serde_json::json!({"password_suggestion" : pass_suggestion})))
}