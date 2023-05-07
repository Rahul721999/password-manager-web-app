use crate::{
    utils::{valid_email, valid_password, verify_pass},
    AppError, MyMiddleware, UserCred,
    
};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{error, info};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginCred {
    #[validate(custom(function = "valid_email"))]
    pub email: String,
    #[validate(custom(function = "valid_password"))]
    pub password: String,
}

#[tracing::instrument(
    name = "🚩 Web Del_Acc Req"
    skip (db, user_cred)
    fields(
        email = %user_cred.email.clone()
    )
)]
pub async fn del_acc(
    user_cred: web::Json<LoginCred>,
    db: web::Data<PgPool>,
    mid : MyMiddleware,
) -> Result<HttpResponse, AppError> {
    //1. form validation..
    match user_cred.validate() {
        Ok(..) => {}
        Err(err) => match err.field_errors() {
            errors if errors.contains_key("email") => {
                error!("❌Email Validation error");
                return Err(AppError::AuthError("Invalid email".to_string()));
            }
            errors if errors.contains_key("pass") => {
                error!("❌Password Validation error");
                return Err(AppError::AuthError("Passwod validation error".to_string()));
            }
            _ => return Err(AppError::BadRequest("Invalid input")),
        },
    };
    // Extract Data from the token..
    let (user_id, user_email) = ( mid.user_id, mid.user_email);
    // 3. Verify the credentiala..
    // 3.1 get the hashed_pass..
    let row = match sqlx::query_as!(
        UserCred,
        "SELECT * FROM user_cred WHERE id = $1",
        user_id
    )
    .fetch_optional(db.as_ref())
    .await
    {
        Ok(row) => {
            if let Some(r) = row {
                r
            } else {
                return Err(AppError::InternalServerError(format!(
                    "Email: {} not found, Try SignIn first",
                    user_email
                )));
            }
        }
        Err(err) => {
            error!("❌SELECT query failed: {}", err);
            return Err(AppError::InternalServerError("Failed to search your data while performing delete operation".to_string()));
        }
    };

    let password = row.password_hash;
    // 3.2 compare the hashed_pass with entered_pass
    if !verify_pass(user_cred.password.clone().as_str(), password.as_str()).await {
        return Err(AppError::AuthError("Unauthorize User".to_string()));
    }


    // 4. Delete user Data before comeplete the Delete req..
    match sqlx::query!("DELETE FROM website_credentials WHERE user_id = $1", user_id)
        .execute(db.as_ref())
        .await{
            Ok(..) => {info!("Deleting the user_data first")}
            Err(err) => {
                error!("❌ Error while deleting User_data");
                return Err(AppError::InternalServerError(format!("Failed to delete user_data, Error: {}",err)));}
        };

    // 5. Complete Delete req..
    match sqlx::query!(
        "DELETE FROM user_cred WHERE email = $1",
        user_cred.email.clone()
    )
    .execute(db.as_ref())
    .await
    {
        Ok(_res) => {
            info!("✅ Account deleted Successfully");
            Ok(HttpResponse::Ok()
                .json(serde_json::json!({"message" : "Account Deleted Successfully"})))
        }
        Err(err) => {
            error!("❌DELETE query failed : {}", err);
            Err(AppError::InternalServerError("Failed to delete your account".to_string()))
        }
    }
}
