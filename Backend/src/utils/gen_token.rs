use secrecy::ExposeSecret;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{Header, encode, decode, EncodingKey, DecodingKey, errors::{ErrorKind}, Validation};
use sqlx::types::Uuid;
use tracing::error;
use crate::AppError;
use crate::configs::Settings;
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims{
    pub id : Uuid,
    pub email : String,
    pub exp : usize,
}
impl TokenClaims{
    pub fn generate(&self, config: &Settings) -> Result<String,AppError>{
        match encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(config.application.jwt_key.expose_secret().as_ref())
        ){
            Ok(token) => Ok(token),
            Err(err) => {
                error!("Generating JWT error: {}",err);
                Err(AppError::InternalServerError("".to_string()))
            },
        }
    }


    pub fn decode_token(token : &str, config: &Settings)-> Result<TokenClaims, AppError>{
        match decode(
            token, 
            &DecodingKey::from_secret(config.application.jwt_key.expose_secret().as_ref()),
            &Validation::default()
        ){
            Ok(data) => Ok(data.claims),
            Err(err) => match err.into_kind(){
                ErrorKind::InvalidToken => Err(AppError::AuthError("Invalid token".to_string())),
                ErrorKind::ExpiredSignature => Err(AppError::BadRequest("Token expired")),
                ErrorKind::ImmatureSignature =>{ 
                    error!("❗Check JWT-Secret-key");
                    Err(AppError::InternalServerError("Error while decode Auth token".to_string()))},
                _ => {
                    tracing::error!("Decode token Error");
                    Err(AppError::InternalServerError("Error while decoding token".to_string()))},
            },
        }
    }
}