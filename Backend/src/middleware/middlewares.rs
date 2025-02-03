use crate::{AppError, Settings, TokenClaims};
use actix_web::{web, FromRequest, HttpRequest};
use futures::future::{self, Ready};
use sqlx::types::Uuid;
#[derive(Debug)]
pub struct MyMiddleware {
    pub user_id: Uuid,
    pub user_email: String,
}

// impl debug trait manually

// impl FromRequest Extractor trait from actix-web
impl FromRequest for MyMiddleware {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(req: &HttpRequest) -> Self::Future {
        Self::from_request(req, &mut actix_web::dev::Payload::None)
    }

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        if !req.headers().contains_key("Authorization") {
            return futures::future::err::<Self, AppError>(AppError::BadRequest(
                "Authorization Key not present in header",
            ));
        }
        let config = req
            .app_data::<web::Data<Settings>>()
            .expect("Failed to get the config");

        let auth_token: &str;
        if let Some(token) = req.headers().get("Authorization") {
            auth_token = token.to_str().expect("Failed to get Auth Token as str");
            // 1. Extract Data from the token..
            match TokenClaims::decode_token(auth_token, config) {
                Ok(claims) => future::ok(MyMiddleware {
                    user_id: claims.id,
                    user_email: claims.email,
                }),
                Err(err) => future::err(err),
            }
        } else {
            futures::future::err(AppError::BadRequest("Authorization token not verified"))
        }
    }
}
