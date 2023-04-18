use crate::{
    utils::{valid_email, valid_password, verify_pass},
    AppError, Config, UserCred,
};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{error, info_span, Instrument};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginCred {
    #[validate(custom(function = "valid_email"))]
    pub email: String,
    #[validate(custom(function = "valid_password"))]
    pub password: String,
}

#[tracing::instrument(
    name = "Web Del_Acc Req"
    skip (db, user_cred)
    fields(
        email = %user_cred.email.clone()
    )
)]
pub async fn del_acc(
    user_cred: web::Json<LoginCred>,
    db: web::Data<PgPool>,
    config: web::Data<Config>,
) -> Result<HttpResponse, AppError> {
    //1. form validation..
    let _res = match user_cred.validate() {
        Ok(..) => {}
        Err(err) => match err.field_errors() {
            errors if errors.contains_key("email") => {
                error!("Email Validation error");
                return Err(AppError::AuthError("Invalid email".to_string()));
            }
            errors if errors.contains_key("pass") => {
                error!("Password Validation error");
                return Err(AppError::AuthError(format!("Passwod validation error")));
            }
            _ => return Err(AppError::BadRequest("Invalid input")),
        },
    };
    // 2. Verify the credentiala..
    // 2.1 get the hashed_pass..
    let row = match sqlx::query_as!(
        UserCred,
        "SELECT * FROM user_cred WHERE email = $1",
        user_cred.email.clone()
    )
    .fetch_optional(db.as_ref())
    .instrument(info_span!("Searching query"))
    .await
    {
        Ok(row) => {
            if let Some(r) = row {
                r
            } else {
                return Err(AppError::InternalServerError(format!(
                    "email: {} not present, Try SignIn first",
                    user_cred.email.clone()
                )));
            }
        }
        Err(err) => {
            return Err(AppError::InternalServerError(format!("Error : {}", err)));
        }
    };

    let password = row.password_hash;
    // 2.2 compare the hashed_pass with entered_pass
    if !verify_pass(user_cred.password.clone().as_str(), password.as_str()).await {
        return Err(AppError::AuthError("Unauthorize User".to_string()));
    }

    match sqlx::query!(
        "DELETE FROM user_cred WHERE email = $1",
        user_cred.email.clone()
    )
    .execute(db.as_ref())
    .instrument(info_span!("Delete Query"))
    .await
    {
        Ok(_res) => {
            return Ok(HttpResponse::Ok()
                .json(serde_json::json!({"message" : "Account Deleted Successfully"})));
        }
        Err(err) => {
            return Err(AppError::InternalServerError(format!("Error : {}", err)));
        }
    };
}
