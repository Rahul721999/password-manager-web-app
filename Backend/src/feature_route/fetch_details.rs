use crate::{utils::decrypt, AppError, MyMiddleware};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, PgPool};
use tracing::{error, info};

#[derive(Debug, Deserialize)]
pub struct Data {
    pub website_name: String,
    pub website_url: String,
}
#[derive(Debug, Serialize)]
pub struct FetchedData {
    pub id: Uuid,
    pub username: String,
    pub password_hash: Vec<u8>,
}
#[tracing::instrument(
	name="🚩 User Data-Update request"
	skip_all
)]
pub async fn fetch(
    cred: web::Json<Data>,
    db: web::Data<PgPool>,
    mid: MyMiddleware,
) -> Result<HttpResponse, AppError> {
    // 1. Extract Data from the token..
    let user_id = mid.user_id;

    // 2. fetch the data from the db..
    let row = match sqlx::query_as!(
        FetchedData,
        "SELECT id, username, password_hash 
        FROM website_credentials 
        WHERE user_id = $1 AND website_url = $2 AND website_name = $3",
        user_id,
        cred.website_url.clone(),
        cred.website_name.clone()
    )
    .fetch_optional(db.as_ref())
    .await
    {
        Ok(row) => {
            if let Some(r) = row {
                r
            } else {
                info!(
                    "❗website_name : {}, website_url : {} couldn't be found",
                    cred.website_name, cred.website_url
                );
                return Err(AppError::BadRequest(
                    "Sorry, the data you provided coudn't be found in our database",
                ));
            }
        }
        Err(err) => {
            error!("❌ failed to find the data in the db: {}", err);
            return Err(AppError::InternalServerError(
                "Sorry, data couldn't be found in the Database".to_string(),
            ));
        }
    };

    let dec_password = match decrypt(row.password_hash).await {
        Ok(pass) => pass,
        Err(err) => {
            error!("❌ failed to decrypt the password: {}", err);
            return Err(AppError::InternalServerError(
                "Data couldn't be fetched from the Database".to_string(),
            ));
        }
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "row_id" : row.id,
        "website_url" : row.username,
        "password" : dec_password,
    })))
}
