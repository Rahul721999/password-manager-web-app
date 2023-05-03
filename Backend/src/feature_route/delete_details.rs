use crate::{AppError, MyMiddleware};
use actix_web::{web, HttpResponse};
use serde::{Deserialize};
use sqlx::{PgPool};
use tracing::{error, info};
#[derive(Debug, Deserialize)]
pub struct Data {
    pub website_name: String,
    pub website_url: String,
}

#[tracing::instrument(
	name="🚩 User Data-Delete request"
	skip_all
)]
pub async fn delete(
    cred: web::Json<Data>,
    db: web::Data<PgPool>,
    mid: MyMiddleware
)-> Result<HttpResponse, AppError>{
    // 1. Extract Data from the token..
    let user_id = mid.user_id;

// 2. Delete the data from the db..
    match sqlx::query!(
        "DELETE 
        FROM website_credentials 
        WHERE user_id = $1 AND website_url = $2 AND website_name = $3", 
        user_id, 
        cred.website_url.clone(),
        cred.website_name.clone()
    )
    .execute(db.as_ref())
    .await{
        Ok(res) => { match res.rows_affected(){
            0 => {
                info!("✅ Details coudn't be found in the DB");
                return Ok(HttpResponse::Ok()
                .json(serde_json::json!({"message" : "Data couldn't be found in the DB"})));
            }
            1 => {
                info!("✅ Details deleted Successfully");
                return Ok(HttpResponse::Ok()
                .json(serde_json::json!({"message" : "Credentials Deleted Successfully"})));
            }
            _ =>{
                error!("❌Multiple row has been Deleted❌");
                return Err(AppError::InternalServerError(format!("Multiple Data row has been deleted")));
            }
        }
        }
        Err(err) => {
            error!("❌DELETE query failed : {}", err);
            return Err(AppError::InternalServerError(format!("Failed to delete your credentials")));
        }
    };
}