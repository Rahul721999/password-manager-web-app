use crate::{analyze_pass, valid_password, AppError, Config, MyMiddleware, TokenClaims, utils::encrypt};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::{types::Uuid, PgPool};
use tracing::{error, info};
#[derive(Debug, Deserialize)]
pub struct Data {
    pub website_name: String,
    pub website_url: String,
    pub username: String,
    pub password: String,
}

#[tracing::instrument(
	name="🚩 Web Data-Store request"
	skip_all
)]
pub async fn store(
    cred: web::Json<Data>,
    db: web::Data<PgPool>,
    mid: MyMiddleware,
    config: web::Data<Config>,
) -> Result<HttpResponse, AppError> {
    // Extract Data from the token..
    let token = mid.token;
    let (user_id, _user_email) = match TokenClaims::decode_token(&token, &config) {
        Ok(claims) => (claims.id, claims.email),
        Err(err) => return Err(err),
    };
    // check the password validity and Strength..
    if let Err(_err) = valid_password(&cred.password) {
        return Err(AppError::AuthError(format!("Password must contain at least one UPPER-CASE, one lower-case, 1 number & a $pecial char")));
    }
    if let Err(err) = analyze_pass(&cred.password) {
        return Err(err);
    }

    // Search if the (user_id & website_url) is already present in the DB..
    let data_present = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 
            FROM user_cred 
            INNER JOIN website_credentials
            ON user_cred.id = website_credentials.user_id 
            WHERE user_id = $1 AND website_url = $2
        )",
    )
    .bind(user_id)
    .bind(cred.website_url.clone())
    .fetch_one(db.as_ref())
    .await
    .map_err(|err| AppError::InternalServerError(format!("Exists query failed: {}", err)))?;

    // if (user_id & website_url) present in db
    if data_present {
        // get the pass & compare with the given pass..
        // if the (website_url & the pass) combination is the same..
        // return
            return Ok(HttpResponse::Ok().body("Data already present"));
        // else try updating the password for the given website_url.
    }
    // else store the credentials to the DB..
    let hash = match encrypt(&cred.password).await {
        Ok(hash) => hash,
        Err(err) => {
            return Err(AppError::InternalServerError(format!("password encryption error")));
        }
    };

    // Insert Data to DB..
    match sqlx::query!(
        "INSERT INTO website_credentials (id, user_id, website_name, website_url, username, password_hash, created_at, updated_at) 
        VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())",
        Uuid::new_v4().into(),
        user_id,
        cred.website_name,
        cred.website_url,
        cred.username,
        hash
    ).execute(db.as_ref()).await{
        Ok(_) =>  info!("✅User added successfully"),
        Err(err) => {error!("❌Failed to add User: {}",err); return Err(AppError::InternalServerError(format!("{}",err)))}
    };

    Ok(HttpResponse::Ok().body("Data added"))
}
