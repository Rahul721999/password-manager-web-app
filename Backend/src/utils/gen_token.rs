use serde::{Serialize, Deserialize};
use jsonwebtoken::{Header, encode, EncodingKey, errors::Error};
use dotenv::dotenv;
use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims{
    pub email : String,
    pub exp : usize,
}
impl TokenClaims{
    pub fn generate(&self, config: &Config) -> Result<String,Error>{
        dotenv().ok();
        match encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(config.jwt_key.as_ref())
        ){
            Ok(token) => Ok(token),
            Err(err) => return Err(err)
        }
    }
}