use crate::{analyze_pass, utils::encrypt, valid_password, AppError, MyMiddleware, UserData};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use sqlx::{types::Uuid, PgPool};
use tracing::{error, info};
#[derive(Debug, Deserialize)]
pub struct Data {
    pub id: Option<Uuid>,
    pub website_name: Option<String>,
    pub website_url: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[tracing::instrument(
	name="🚩 Web Data-Update request"
	skip_all
)]
pub async fn update(
    cred: web::Json<Data>,
    db: web::Data<PgPool>,
    mid: MyMiddleware,
) -> Result<HttpResponse, AppError> {
    // if the ROW_id has not given...
    let row_id = if let Some(id) = cred.id {
        if id.to_string().len() == 0 {
            return Err(AppError::BadRequest("Invalid ID"));
        };
        id
    } else {
        return Err(AppError::BadRequest("id not provied"));
    };

    // Extract Data from the token..
    let user_id = mid.user_id;

    // Search if the (user_id & website_url) is already present in the DB..
    let mut data_present = sqlx::query_as::<_, UserData>(
        "SELECT *
            FROM website_credentials
            WHERE id = $1 AND user_id = $2
        ",
    )
    .bind(cred.id)
    .bind(user_id)
    .fetch_one(db.as_ref())
    .await
    .map_err(|err| {
        error!("❌ SELECT query failed: {}", err);
        AppError::InternalServerError("Data not found in our DB".to_string())
    })?;

    // update the data in existing_data if the None value is given
    if let Some(website_name) = &cred.website_name {
        data_present.website_name = website_name.to_owned();
    }
    if let Some(website_name) = &cred.website_url {
        data_present.website_url = website_name.to_owned();
    }
    if let Some(username) = &cred.username {
        data_present.username = username.to_owned();
    }

    let mut new_pass: String = "Prev-used-pass".to_string();
    if let Some(password) = &cred.password {
        new_pass = password.clone();
        // check the password validity and Strength..
        if password.len() < 8 {
            return Err(AppError::BadRequest(
                "You should choose password of length more than 8 character",
            ));
        }
        if let Err(_err) = valid_password(password) {
            return Err(AppError::BadRequest("Password must contain at least one UPPER-CASE, one lower-case, 1 number & a $pecial char"));
        }
        analyze_pass(password)?;

        let hash = match encrypt(password).await {
            Ok(hash) => hash,
            Err(_err) => {
                return Err(AppError::InternalServerError(
                    "password encryption error".to_string(),
                ));
            }
        };
        data_present.password_hash = hash;
    }
    // Update Data to DB..
    match sqlx::query(
        "UPDATE website_credentials 
        SET website_name = $1, website_url = $2, username = $3, password_hash = $4, updated_at = NOW()
        WHERE id = $5 AND user_id = $6"
    )
    .bind(&data_present.website_name)
    .bind(&data_present.website_url)
    .bind(&data_present.username)
    .bind(&data_present.password_hash)
    .bind(row_id)
    .bind(user_id)
    .execute(db.as_ref()).await{
        Ok(_) =>  {
            info!("✅User-Data updated successfuly");
            let result = json!({
                "website_name" : data_present.website_name,
                "website_url" : data_present.website_url,
                "username" : data_present.username,
                "password" : new_pass
            });
            Ok(HttpResponse::Ok().json(json!({"message" : "Data updated successfuly","updated_data" : result})))
        },
        Err(err) => {
            error!("❌Failed to add User: {}",err); 
            Err(AppError::InternalServerError(format!("{}",err)))
        }
    }
}
