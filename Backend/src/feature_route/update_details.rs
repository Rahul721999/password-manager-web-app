use std::collections::HashMap;

use crate::{analyze_pass, valid_password, AppError, Config, MyMiddleware, TokenClaims, utils::encrypt};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::{types::Uuid, PgPool};
use tracing::{error, info};

#[derive(Debug, Deserialize)]
pub struct Data {
    pub id : Uuid,
    pub website_name: String,
    pub website_url: String,
    pub username : String,
    pub password : String,
}

#[tracing::instrument(
	name="🚩 Web Data-Update request"
	skip_all
)]
pub async fn update(
    cred: web::Json<HashMap<String, String>>,
    db: web::Data<PgPool>,
    mid: MyMiddleware,
    config: web::Data<Config>
)-> Result<HttpResponse, AppError>{
    // Extract Data from the token..
    let token = mid.token;
    let (user_id, _user_email) = match TokenClaims::decode_token(&token, &config) {
        Ok(claims) => (claims.id, claims.email),
        Err(err) => return Err(err),
    };
    // extract the data from the cred
    let mut fields = Vec::new();
    let mut values = Vec::new();
    for (field, value) in cred.iter() {
        fields.push(field);
        values.push(value);
    }

    // check the password validity and Strength..
    if let Some(new_password) = cred.get("password"){new_password};
    if let Err(_err) = valid_password(&new_password) {
        return Err(AppError::AuthError(format!("Password must contain at least one UPPER-CASE, one lower-case, 1 number & a $pecial char")));
    }
    if let Err(err) = analyze_pass(&cred.password) {
        return Err(err);
    }

    // Search if the (user_id & website_url) is already present in the DB..
    let data_present = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT * 
            FROM user_cred 
            INNER JOIN website_credentials
            ON user_cred.id = website_credentials.user_id 
            WHERE id = $1 AND user_id = $2
        )",
    )
    .bind(cred.)
    .bind(user_id)
    .fetch_one(db.as_ref())
    .await
    .map_err(|err| {
        error!("Exists query failed: {}", err);
        return AppError::InternalServerError(format!("Searching db failed"))}
    )?;

    // if the data coudn't found with the given combination
    if !data_present{
        return Err(AppError::AuthError(format!("Data not found or you are not authorized to access the data")))
    }
    // else store the credentials to the DB..
    let hash = match encrypt(&cred.password).await {
        Ok(hash) => hash,
        Err(_err) => {
            return Err(AppError::InternalServerError(format!("password encryption error")));
        }
    };
    // Update Data to DB..
    match sqlx::query!(
        "UPDATE website_credentials 
        SET 
        WHERE ",
        Uuid::new_v4().into(),
        user_id,
        cred.website_name,
        cred.website_url,
        cred.username,
        hash
    ).execute(db.as_ref()).await{
        Ok(_) =>  {
            info!("✅User added successfuly");
            return Ok(HttpResponse::Ok().body("Data added successfuly"));
        },
        Err(err) => {
            error!("❌Failed to add User: {}",err); 
            return Err(AppError::InternalServerError(format!("{}",err)))
        }
    };
}