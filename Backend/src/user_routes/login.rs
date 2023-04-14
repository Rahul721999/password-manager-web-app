use actix_web::{HttpResponse, web};
use crate::{AppError,
    utils::{verify_pass, valid_email, valid_password},
};
use sqlx::{PgPool, Row};
use tracing::{error, Instrument, error_span};
use serde::Deserialize;
use validator::Validate;
#[derive(Debug, Deserialize, Validate)]
pub struct LoginCred{
    #[validate(custom(
        function = "valid_email"
    ))]
    pub email : String,
    #[validate(custom(
        function = "valid_password"
    ))]
    pub password : String,
}


#[tracing::instrument(
    name = "Web Login Req"
    skip (db, user_cred)
    fields(
        email = %user_cred.email.clone()
    )
)]
pub async fn login(
    user_cred : web::Json<LoginCred>,
    db : web::Data<PgPool>
) -> Result<HttpResponse, AppError>{
    //1. form validation..
    let _res =match user_cred.validate(){
        Ok(..) => {},
        Err(err) =>{
            
            match err.field_errors() {
                errors if errors.contains_key("email")=>{
                    error!("Email Validation error");
                    return Err(AppError::AuthError("Invalid email".to_string()))
                }
                errors if errors.contains_key("pass") =>{
                    error!("Password Validation error");
                    return Err(AppError::AuthError(format!("Passwod validation error")))
                }
                _ => return Err(AppError::BadRequest("Invalid input"))
            }   
        }
    };

    // 2. fetch the hashed_password from the db..
    let row = match sqlx::query("SELECT password_hash FROM user_cred WHERE email = $1")
                                    .bind(user_cred.email.clone())
                                    .fetch_optional(db.as_ref())
                                    .instrument(error_span!("Db query"))
                                    .await{
                                        Ok(row) => {
                                            if let Some(r) = row{ r } 
                                            else {
                                                return Err(AppError::InternalServerError(format!("email: {} not present, Try SignIn first", user_cred.email.clone()))); 
                                            } 
                                        },
                                        Err(_err) =>  {
                                            return Err(AppError::InternalServerError(format!("Error : {}", _err)));
                                        },
                                    };

    
    // let password: String = password.map(|r| r.get("password_hash")).expect("failed to get password");
    let password: String = match row.try_get("password_hash"){
        Ok(pass) => pass,
        Err(_err) => {
            error_span!("Failed to get the password");
            return Err(AppError::InternalServerError(format!("Error : {}", _err)));
        }
    };
    // 3. compare the hashed_pass with entered_pass
    if verify_pass(user_cred.password.clone().as_str(), password.as_str()).await {
        return Ok(HttpResponse::Ok().finish())
    }else {
        return Err(AppError::AuthError("Unauthorize User".to_string()))
    }

}