use crate::{AppError, MyMiddleware, utils::decrypt};
use actix_web::{web, HttpResponse};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serde::{Serialize,Deserialize};
use sqlx::{PgPool, types::Uuid, FromRow};
use tracing::{error};

#[derive(Debug, Serialize)]
pub struct Data{
    pub id  : Uuid,
    pub username : String,
    pub password : String,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FetchedData{
    pub id : Uuid,
    pub username : String,
    pub password_hash : Vec<u8>,
}
#[tracing::instrument(
	name="🚩 User Data-Update request"
	skip_all
)]
pub async fn fetch_all(
    db: web::Data<PgPool>,
    mid: MyMiddleware
)-> Result<HttpResponse, AppError>{

// 1. Extract Data from the token..
    let user_id = mid.user_id;

// 2. fetch the data from the db..

    let rows = 
    match sqlx::query_as::<_,FetchedData>(
        "SELECT id, username, password_hash 
        FROM website_credentials 
        WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_all(db.as_ref())
    .await{
        Ok(all_row) => all_row,
        Err(err) =>  {
            error!("❌ failed to find the data in the db: {}", err);
            return Err(AppError::InternalServerError("Sorry, no data could be found in the Database".to_string()));
        },
    };

    // 3. create a VEC to store all the rows..
    let result_vec: Vec<Data> = rows.into_iter().map(|fetched_data| {
        let password_hash = fetched_data.password_hash.clone();

        // create a closure
        async move {
            let dec_password = match decrypt(password_hash).await {
                Ok(pass) => pass,
                Err(err) => {
                    error!("❌ failed to decrypt the password: {}",err);
                    return Err(AppError::InternalServerError("Data couldn't be fetched from the Database".to_string()));
                }
            };
            Ok(Data {
                id: fetched_data.id,
                username: fetched_data.username,
                password: dec_password,
            })
        }
        // call the closure here
    })
    .collect::<FuturesUnordered<_>>()
    .filter_map(|result| async move { result.ok() })
    .collect()
    .await;


   Ok(HttpResponse::Ok().json(serde_json::json!({"data" : result_vec})))
    
}