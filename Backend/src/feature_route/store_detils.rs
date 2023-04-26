use actix_web::HttpResponse;
use crate::AppError;
use serde::Deserialize;
use crate::{
    UserData,

};
/*
pub struct Deta{
    pub website_name : String,
    pub website_url : String,
    pub username : String,
    pub password  : String,
    pub created_at : usize,
    pub updated_at : usize,
}

 */



#[tracing::instrument(
	name="🚩Web Data-Store request"
	skip_all
    fields(
       
    )
)]
pub async fn store() -> Result<HttpResponse, AppError>{
     
    todo!()
}