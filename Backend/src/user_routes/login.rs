use crate::{
    models::AuthProvider,
    utils::{valid_email, valid_password, verify_pass},
    AppError, Settings, TokenClaims, UserCred,
};
use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::error;
use validator::Validate;
#[derive(Debug, Deserialize, Validate)]
pub struct LoginCred {
    #[validate(custom(function = "valid_email"))]
    pub email: String,
    #[validate(custom(function = "valid_password"))]
    pub password: String,
}

#[tracing::instrument(
    name = "🚩 Web Login Req"
    skip (db, user_cred, config)
    fields(
        email = %user_cred.email.clone()
    )
)]
pub async fn login(
    user_cred: web::Json<LoginCred>,
    db: web::Data<PgPool>,
    config: web::Data<Settings>,
) -> Result<HttpResponse, AppError> {
    //1. form validation..
    match user_cred.validate() {
        Ok(..) => {}
        Err(err) => match err.field_errors() {
            errors if errors.contains_key("email") => {
                error!("❌ Invalid Email");
                return Err(AppError::AuthError("Invalid email".to_string()));
            }
            errors if errors.contains_key("pass") => {
                error!("❌ Invalid Password");
                return Err(AppError::AuthError("Invalid Password".to_string()));
            }
            _ => return Err(AppError::BadRequest("Invalid input")),
        },
    };

    // 2. fetch the hashed_password from the db..
    let row = match sqlx::query_as::<_, UserCred>(
        "SELECT id, auth_provider, email, google_id, password_hash, first_name, last_name FROM user_cred WHERE email = $1", 
    )
    .bind(user_cred.email.clone())
    .fetch_optional(db.as_ref())
    .await
    {
        Ok(row) => {
            if let Some(r) = row {
                r
            } else {
                return Err(AppError::InternalServerError(format!(
                    "{} not present, Try SignIn first",
                    user_cred.email.clone()
                )));
            }
        }
        Err(err) => {
            return Err(AppError::InternalServerError(format!("Error : {}", err)));
        }
    };

    if row.auth_provider == AuthProvider::Google {
        tracing::warn!("Authprovider: {}", row.auth_provider);
        return Err(AppError::AuthError("Try login with google".to_string()));
    };
    let password = row.password_hash.unwrap_or("".to_string());

    // 3. compare the hashed_pass with entered_pass
    if !verify_pass(user_cred.password.clone().as_str(), password.as_str()).await {
        return Err(AppError::AuthError("Unauthorize User".to_string()));
    }
    // TODO: NEED TO ADD SECOND STEP VERIFICATION HERE...
    // 4. if credectials matches generate & return JWT
    let claim = TokenClaims {
        id: row.id,
        email: user_cred.email.clone(),
        exp: (Utc::now() + Duration::seconds(config.application.jwt_exp as i64)).timestamp()
            as usize,
    };

    let token = match claim.generate(&config) {
        Ok(token) => token,
        Err(_err) => {
            error!("❌ failed to generate token");
            return Err(AppError::InternalServerError(
                "Failed to gen JWT".to_string(),
            ));
        }
    };
    tracing::info!("✅ Logged In Successfully");
    Ok(
        HttpResponse::Ok()
            .json(serde_json::json!({"message" : "Success", "Authorization" : token})),
    )
}
