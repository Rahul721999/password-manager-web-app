use serde::{Serialize, Deserialize};
use jsonwebtoken::{Header, encode, decode, EncodingKey, DecodingKey, errors::{ErrorKind, Error}, Validation, Algorithm};
use crate::AppError;
use crate::config::Config;
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims{
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
            Err(err) => match err.into_kind(){
                ErrorKind::ExpiredSignature => return Err(AppError::BadRequest("Token expired")),
                _ => return Err(AppError::InternalServerError("Error while decoding token".to_string())),
            },
        }
    }


    pub fn decode_token(token : &str, config: &Config)-> Result<TokenClaims, Error>{
        match decode(
            &token, 
            &DecodingKey::from_secret(config.jwt_key.as_ref()),
            &Validation::new(Algorithm::HS256)
        ){
            Ok(data) =>return Ok(data.claims),
            Err(err) => return Err(err),
        }
    }
}