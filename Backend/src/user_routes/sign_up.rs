#![allow(unused)]
use std::ptr::hash;
use crate::{
    models::UserCred,
    utils::hash_pass,
    utils::{AppError, valid_email, valid_password}
};
use actix_web::{HttpResponse, web, Responder, App, http::header::HttpDate};
use anyhow::Context;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::info;
use sqlx::types::Uuid;
use validator::{Validate, ValidationError};
use regex::Regex;

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct NewUser{
    #[validate(custom(
        function = "valid_email"))]
    pub email : String,
    #[validate(
        custom(
            function = "valid_password"))]

    pub pass : String,
    pub first_name : String,
    pub last_name : String,
}
#[tracing::instrument(
	name="Web signin request"
	skip_all
)]
pub async fn sign_up(
    new_user : web::Json<NewUser>,
    db : web::Data<PgPool>
)
-> Result<HttpResponse, AppError>
{  
//1. form validation..
    let _res =match new_user.validate(){
        Ok(..) => {},
        Err(err) =>{
            tracing::info!("validation failed");
            // return Err(AppError::Other(err))}
            match err.field_errors() {
                errors if errors.contains_key("email")=>{
                    dbg!(errors);
                    return Err(AppError::AuthError("Invalid email".to_string()))
                }
                errors if errors.contains_key("pass") =>{
                    return Err(AppError::AuthError("Must contain at least one upper-case, one lower-case,\na number & a special char".to_string()))
                }
                _ => return Err(AppError::BadRequest("Invalid input"))
            }   
        }
    };
//2. first check if the email already present in the DB.
    let data_present = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM user_cred WHERE email = $1)")
    .bind(new_user.email.clone())
    .fetch_one(db.as_ref())
    .await
    .map_err(|_| HttpResponse::InternalServerError().body("Failed to check if email exists")).expect("failed to map the error");

    if data_present{
            tracing::info!("Email already present in the db"); 
            return Err(AppError::EmailExists);
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
    tracing::info!("new user with email: {} signed in successfuly", new_user.email.clone());

    Ok(HttpResponse::Ok().body("User added Successfully"))
}
