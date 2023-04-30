use crate::{AppError, Config, MyMiddleware, TokenClaims, utils::decrypt};
use actix_web::{web, HttpResponse};
use serde::{Serialize,Deserialize};
use sqlx::{PgPool};
use tracing::{error, info};

#[derive(Debug, Deserialize)]
pub struct Data {
    pub website_name: String,
    pub website_url: String,
}
#[derive(Debug, Serialize)]
pub struct FetchedData{
    pub username : String,
    pub password_hash : String,
}
#[tracing::instrument(
	name="🚩 Web Data-Update request"
	skip_all
)]
pub async fn fetch(
    cred: web::Json<Data>,
    db: web::Data<PgPool>,
    mid: MyMiddleware,
    config: web::Data<Config>,
)-> Result<HttpResponse, AppError>{

// 1. Extract Data from the token..
    let token = mid.token;
    let (user_id, _user_email) = match TokenClaims::decode_token(&token, &config) {
        Ok(claims) => (claims.id, claims.email),
        Err(err) => return Err(err),
    };
// 2. fetch the data from the db..
    let mut row = 
    match sqlx::query_as!(FetchedData,
        "SELECT username, password_hash 
        FROM website_credentials 
        WHERE user_id = $1 AND website_url = $2", 
        user_id, 
        cred.website_url.clone()
    )
    .fetch_optional(db.as_ref())
    .await{
        Ok(row) => {
            if let Some(r) = row{ r } 
            else {
                info!("❗website_name : {}, website_url : {} couldn't be found", cred.website_name, cred.website_url);
                return Err(AppError::BadRequest("Sorry, the data you provided coudn't be found in our database"));
            } 
        },
        Err(err) =>  {
            error!("❌ failed to find the data in the db: {}", err);
            return Err(AppError::InternalServerError(format!("Sorry, data couldn't be found in the Database")));
        },
    };

    row.password_hash = match decrypt(&row.password_hash).await{
        Ok(pass) => pass,
        Err(err) => {
            error!("❌ failed to decrypt the password: {}",err);
            return Err(AppError::InternalServerError(format!("Data couldn't be fetched from the Database")));
        }
    };

    Ok(HttpResponse::Ok().json(row))
}