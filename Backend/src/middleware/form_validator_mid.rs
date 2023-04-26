use actix_web::{
    HttpRequest, FromRequest, HttpMessage, web,
};
use futures::future::Ready;
use crate::{AppError, Config};

#[derive(Debug)]
pub struct UserCredentials{
    pub email : String,
    pub password : String,
}

impl FromRequest for UserCredentials{
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(req: &HttpRequest) -> Self::Future {
        Self::from_request(req, &mut actix_web::dev::Payload::None)
    }

    fn from_request(req: &HttpRequest, body: &mut actix_web::dev::Payload) -> Self::Future {
        let body =  *req.app_data::<Config>().unwrap();
        

        todo!()
    }
}