use actix_web::{
    HttpRequest, FromRequest,
};
use futures::future::Ready;
use crate::{AppError};
#[derive(Debug)]
pub struct MyMiddleware{
    pub token :  String,
}

// impl debug trait manually

// impl FromRequest Extractor trait from actix-web
impl FromRequest for MyMiddleware{
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(req: &HttpRequest) -> Self::Future {
        Self::from_request(req, &mut actix_web::dev::Payload::None)
    }

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        if !req.headers().contains_key("Authorization"){
            return futures::future::err::<Self,AppError>(AppError::BadRequest("Authorization Key not present in header"));
        }

        let auth_token: String;
        if let Some(token) = req.headers().get("Authorization"){
            auth_token = token.to_str().expect("Failed to get Auth Token as str").to_string();
        }else{
            return futures::future::err(AppError::BadRequest("Authorization token not verified"));
        };

        futures::future::ok(MyMiddleware{token: auth_token})
    }

}
