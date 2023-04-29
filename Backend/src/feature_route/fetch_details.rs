use crate::{analyze_pass, valid_password, AppError, Config, MyMiddleware, TokenClaims, utils::encrypt};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::{types::Uuid, PgPool};
use tracing::{error, info};

#[derive(Debug, Deserialize)]
pub struct Data {
    pub website_name: String,
    pub website_url: String,
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
    // Extract Data from the token..
    let token = mid.token;
    let (user_id, _user_email) = match TokenClaims::decode_token(&token, &config) {
        Ok(claims) => (claims.id, claims.email),
        Err(err) => return Err(err),
    };
    todo!()
}