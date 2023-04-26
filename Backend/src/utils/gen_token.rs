use serde::{Serialize, Deserialize};
use jsonwebtoken::{Header, encode, decode, EncodingKey, DecodingKey, errors::{ErrorKind}, Validation};
use sqlx::types::Uuid;
use crate::AppError;
use crate::config::Config;
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims{
    pub id : Uuid,
    pub email : String,
    pub exp : usize,
}
impl TokenClaims{
    pub fn generate(&self, config: &Config) -> Result<String,AppError>{
        match encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(config.jwt_key.as_ref())
        ){
            Ok(token) => Ok(token),
            Err(err) => {
                tracing::error!("Generating JWT error: {}",err);
                return Err(AppError::InternalServerError("".to_string()))
            },
        }
    }


    pub fn decode_token(token : &str, config: &Config)-> Result<TokenClaims, AppError>{
        match decode(
            &token, 
            &DecodingKey::from_secret(config.jwt_key.as_ref()),
            &Validation::default()
        ){
            Ok(data) =>return Ok(data.claims),
            Err(err) => match err.into_kind(){
                ErrorKind::ExpiredSignature => return Err(AppError::BadRequest("Token expired")),
                _ => {
                    tracing::error!("Decode token Error");
                    return Err(AppError::InternalServerError("Error while decoding token".to_string()))},
            },
        }
    }
}