use crate::{AppError};
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
pub async fn update(

)-> Result<HttpResponse, AppError>{

}