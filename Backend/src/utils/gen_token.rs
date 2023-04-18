use serde::{Serialize, Deserialize};
use jsonwebtoken::{Header, encode, decode, EncodingKey, DecodingKey, errors::Error, Validation, Algorithm};
use crate::config::Config;
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims{
    pub email : String,
    pub exp : usize,
}
impl TokenClaims{
    pub fn generate(&self, config: &Config) -> Result<String,Error>{
        match encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(config.jwt_key.as_ref())
        ){
            Ok(token) => Ok(token),
            Err(err) => return Err(err)
        }
    }


    pub fn decode_token(token : String, config: &Config)-> Result<TokenClaims, Error>{
        match decode(
            &token.as_str(), 
            &DecodingKey::from_secret(config.jwt_key.as_ref()),
            &Validation::new(Algorithm::HS256)
        ){
            Ok(data) =>return Ok(data.claims),
            Err(err) => return Err(err),
        }
    }
}