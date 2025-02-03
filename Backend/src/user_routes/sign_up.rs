use crate::{
    analyze_pass,
    utils::hash_pass,
    utils::{valid_email, valid_password, AppError},
};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::types::Uuid;
use sqlx::PgPool;
use tracing::{error, info};
use validator::Validate;

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct NewUser {
    #[validate(custom(function = "valid_email"))]
    pub email: String,
    #[validate(custom(function = "valid_password"))]
    pub pass: String,
    pub first_name: String,
    pub last_name: String,
}
#[tracing::instrument(
	name="🚩Web signin request"
	skip_all
    fields(
        email = %new_user.email.clone(),
        name = %new_user.first_name.clone()
    )
)]
pub async fn sign_up(
    new_user: web::Json<NewUser>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    // 1 Pre-Req Validations
    // 1.1 form validation..
    match new_user.validate() {
        Ok(..) => {}
        Err(err) => match err.field_errors() {
            errors if errors.contains_key("email") => {
                error!("❌ Email Validation error");
                return Err(AppError::AuthError("Invalid email".to_string()));
            }
            errors if errors.contains_key("pass") => {
                error!("❌ Password Validation error");
                return Err(AppError::AuthError("Password must contain at least one UPPER-CASE, one lower-case, 1 number & a $pecial char".to_string()));
            }
            _ => return Err(AppError::BadRequest("Invalid input")),
        },
    };

    // 1.2 Analyze password..
    analyze_pass(&new_user.pass)?;

    //2. first check if the email already present in the DB.
    let data_present =
        sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM user_cred WHERE email = $1)")
            .bind(new_user.email.clone())
            .fetch_one(db.as_ref())
            .await
            .map_err(|_| AppError::InternalServerError("EXIST query failed".to_string()))?;

    if data_present {
        info!(
            "🚫Email : {} already present in the db, Try to login",
            new_user.email.clone()
        );
        return Err(AppError::EmailExists);
    }

    //3. hash the pass
    let hash_pass = match hash_pass(&new_user.pass).await {
        Ok(hash) => hash,
        Err(err) => return Err(err),
    };

    //4. store email and password hash to the db
    match sqlx::query!(
        "INSERT INTO user_cred (id, email, password_hash, first_name, last_name) VALUES ($1, $2, $3, $4, $5)",
        Uuid::new_v4().into(),
        new_user.email.clone(),
        hash_pass,
        new_user.first_name.clone(),
        new_user.last_name.clone(),
    ).execute(db.as_ref())
    .await{
        Ok(_) =>  info!("✅User added successfully"),
        Err(_) => error!("❌Failed to add User")
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({"message" : "User added Successfully"})))
}
