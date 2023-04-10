use thiserror::Error;

use actix_web::{error::ResponseError, HttpResponse, http::{header::ContentType, StatusCode}};
#[derive(Error, Debug)]
pub enum AppError{
    #[error("AuthError : {0}")]
    AuthError(String),
    #[error("{0}")]
    InternalServerError(String),
    #[error("Email already Exists")]
    EmailExists,
    #[error("Invalid Email or Password")]
    InvalidCredentials,
    #[error("{0}")]
    BadRequest(&'static str),
    #[error("{0}")]
    Conflict(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ResponseError for AppError{
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type(ContentType::json())  
            .json(serde_json::json!({"message" : self.to_string()}))
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::AuthError(_) | AppError::InvalidCredentials=> StatusCode::UNAUTHORIZED,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::EmailExists | AppError::Conflict(_)=> StatusCode::CONFLICT,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}