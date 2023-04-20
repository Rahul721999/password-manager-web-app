use actix_web::{
    HttpRequest, FromRequest,
};
use futures::future::Ready;
use crate::AppError;

#[derive(Debug)]
pub struct MyMiddleware;

// impl debug trait manually

// impl FromRequest Extractor trait from actix-web
impl FromRequest for MyMiddleware{
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        tracing::info!("🚩middleware fn called");
        if !req.headers().contains_key("Authorization"){
            return futures::future::err::<Self,AppError>(AppError::BadRequest("Authorization Key not present in header"));
        }
        futures::future::ok(MyMiddleware)
    }

    fn extract(req: &HttpRequest) -> Self::Future {
        Self::from_request(req, &mut actix_web::dev::Payload::None)
    }
}
