#![allow(unused)]
use std::ptr::hash;
use crate::{
    models::UserCred,
    utils::hash_pass,
    utils::{AppError, valid_email, valid_password}
};
use actix_web::{HttpResponse, web, Responder};
use anyhow::Context;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::info;
use sqlx::types::Uuid;
use validator::{Validate, ValidationError};
#[derive(Debug, Deserialize, Clone, Validate)]
pub struct NewUser{
    #[validate(custom(
        function = "valid_email", 
        message = "Invalid email"))]
    pub email : String,
    #[validate(custom(function = "valid_password",
        message = "Must contain at least one upper-case, lower-case & a no."))]
    pub pass : String,
    pub first_name : String,
    pub last_name : String,
}
#[tracing::instrument(
	name="Web login request"
	skip_all
)]
pub async fn sign_up(
    new_user : web::Json<NewUser>,
    db : web::Data<PgPool>
)
-> HttpResponse
{  
//1. validate the email first..

//2. first check if the email already present in the DB.
    let data_present = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM user_cred WHERE email = $1)")
    .bind(new_user.email.clone())
    .fetch_one(db.as_ref())
    .await
    .map(|_| true)
    .map_err(|e| {
        // Return a 500 error if there's a database error.
        HttpResponse::InternalServerError().body("email already exists")
    }).expect("failed to get the error msg.");

    if data_present{
            tracing::info_span!("email already present in the db"); 
            return HttpResponse::InternalServerError().body("email already exists");
        }
        
//3. hash the pass
    let hash_pass = hash_pass(&new_user.pass)
        .await
        .expect("failed to get the hash");

//4. store email and password hash to the db
    let res = sqlx::query!(
        "INSERT INTO user_cred (id, email, password_hash, first_name, last_name) VALUES ($1, $2, $3, $4, $5)",
        Uuid::new_v4().into(),
        new_user.email.clone(),
        hash_pass.clone(),
        new_user.first_name.clone(),
        new_user.last_name.clone(),
    ).execute(db.as_ref())
    .await
    .expect("failed to execute the query");

    HttpResponse::Ok().body("User added Successfully")
}
