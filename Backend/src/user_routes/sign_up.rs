// #![allow(unused)]

use crate::{
    utils::hash_pass,
    utils::{AppError, valid_email, valid_password}
};
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{info_span, info, error, Instrument};
use sqlx::types::Uuid;
use validator::Validate;

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
// logging
    let req_id = Uuid::new_v4();
    // info!("SignUp request: req_id : {}, email: {}, name: {}", req_id, new_user.email.clone(), new_user.first_name.clone());
    let request_span = info_span!(
        "Adding a new subscriber.",
        %req_id,
        email = %new_user.email.clone(),
        name= %new_user.first_name.clone()
        );
    let _res  = request_span.enter();
//1. form validation..
    let _res =match new_user.validate(){
        Ok(..) => {},
        Err(err) =>{
            error!("Validation error");
            match err.field_errors() {
                errors if errors.contains_key("email")=>{
                    return Err(AppError::AuthError("Invalid email".to_string()))
                }
                errors if errors.contains_key("pass") =>{
                    return Err(AppError::AuthError(format!("Must contain at least one upper-case, one lower-case,
        a number & a special char")))
                }
                _ => return Err(AppError::BadRequest("Invalid input"))
            }   
        }
    };
//2. first check if the email already present in the DB.
    let data_present = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM user_cred WHERE email = $1)").bind(new_user.email.clone())
    .fetch_one(db.as_ref())
    .instrument(info_span!("Searching for the details in db"))
    .await
    .map_err(|_| AppError::InternalServerError("Failed to check if email exists".to_string()))?;

    if data_present{
            info!("Email : {} already present in the db", new_user.email.clone()); 
            return Err(AppError::EmailExists);
        }
        
//3. hash the pass
    let hash_pass = hash_pass(&new_user.pass)
        .await
        .expect("failed to get the hash");

//4. store email and password hash to the db
    match sqlx::query!(
        "INSERT INTO user_cred (id, email, password_hash, first_name, last_name) VALUES ($1, $2, $3, $4, $5)",
        Uuid::new_v4().into(),
        new_user.email.clone(),
        hash_pass.clone(),
        new_user.first_name.clone(),
        new_user.last_name.clone(),
    ).execute(db.as_ref())
    .await{
        Ok(_) =>  info!("{} successfully added",new_user.email.clone()),
        Err(_) => tracing::error!("Failed to add User")
    };

    Ok(HttpResponse::Ok().body("User added Successfully"))
}
