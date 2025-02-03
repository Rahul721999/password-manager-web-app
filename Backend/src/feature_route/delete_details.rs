use crate::{AppError, MyMiddleware};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::{types::Uuid, PgPool};
use tracing::{error, info};
#[derive(Debug, Deserialize)]
pub struct Data {
    pub id: Uuid,
}

#[tracing::instrument(
	name="🚩 User Data-Delete request"
	skip_all
)]
pub async fn delete(
    cred: web::Json<Data>,
    db: web::Data<PgPool>,
    mid: MyMiddleware,
) -> Result<HttpResponse, AppError> {
    // 1. Extract Data from the token..
    let user_id = mid.user_id;

    // 2. Delete the data from the db..
    match sqlx::query!(
        "DELETE 
        FROM website_credentials 
        WHERE id = $1 AND user_id = $2",
        cred.id,
        user_id,
    )
    .execute(db.as_ref())
    .await
    {
        Ok(res) => {
            match res.rows_affected() {
                0 => {
                    info!("✅ Details coudn't be found in the DB");
                    Ok(HttpResponse::Ok()
                        .json(serde_json::json!({"message" : "Data couldn't be found in the DB"})))
                }
                1 => {
                    info!("✅ Details deleted Successfully");
                    Ok(HttpResponse::Ok()
                        .json(serde_json::json!({"message" : "Credentials Deleted Successfully"})))
                }
                _ => {
                    // this is not possible though..cause the (id) is unique
                    error!("❌Multiple row has been Deleted❌");
                    Err(AppError::InternalServerError(
                        "Multiple Data row has been deleted".to_string(),
                    ))
                }
            }
        }
        Err(err) => {
            error!("❌DELETE query failed : {}", err);
            Err(AppError::InternalServerError(
                "Failed to delete your credentials".to_string(),
            ))
        }
    }
}
