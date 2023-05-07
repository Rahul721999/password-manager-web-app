const PRIV_KEY_STR : &str = "-----BEGIN PRIVATE KEY-----
MIIEvwIBADANBgkqhkiG9w0BAQEFAASCBKkwggSlAgEAAoIBAQDh4JTD0elyjZh0
qwp3YXqpsiT3YbC+IavONo7WbN9aPJp6KhBAY58a02kkwpwzUO8yxqvruMuj3Nnb
x70CkSXrCOvF9J3ctmMu565BRd0rwMCpWVqrVVMKQ7VbT+RCKVsIBY+lkVQ+ssUa
GV4crYCO+rn8fmEK4/mpyuxFpjQAAuotqW5esGbEhtPF2IyLYhdeGcsHrhOfEJQO
t4RsaIC/w+V5YLwtcQSkMq4wv/LqCK8aSv0NwVLZL+L/8qazpPuabUIyJtQKXeC8
kIY1akX+YpcEknpZ2l1JHNhm2VsNzkC18Q+Y3P0Lx+Zi1gMO6OZupfUgvMPh6y1R
/FKCUvw3AgMBAAECggEBAIIf+jPxavaGYgzcOFRcAOlf6nHlgoeWKD7NKW6YG/gF
L80vDFu7yH4QyuLVhlz9xD9ROtu6gv5sjclSgS3IZrrHUeShrovnOq5b5ARQdkDt
c1BcXKKVrhgct47aMJp47qtpYL62QX05SdlmQdRtk8FK6fhu6gL3IO5TK9hYDl7u
OXmg2JAKpXKteWB0Z8QvKhIHDxwUxL2rQ/G3qDkSQ896JGgU2JAr6UAZYEaRYaaJ
IJIDlJIOHGBC3b+3lZSmaxEZ93Eeyn3IcbzOfvWg3NqupQTN8aPy6hG0f2qAZ1bj
WG4Sd7WDXrOegN7IVmNeQMaI9qEB7zXe8fQzpTiM29kCgYEA/b1hbfxs7CZmjShG
Nd8xGDhRRYRiRxYxB2CSurlb83O72MejRG3lOgtB9wb7F0fadLgMtSp/Lk00cXzy
YYkiZC3iBpD2nhOUb5I8yASXs+ANHk3+tCLceQFv0/TKAeGiO4jidUXf5vmR5aPU
drgtbwy5aT40dUv9Zri/MwpOjxsCgYEA4+Op8gQ/7e607rS/wpFSDNfQET7lBvks
PYK0D76SmrlosZUY6azQn/DcC7lQ09mZfuuyn9x1qlFjxpqRaa2CsfPq4fFOSr/S
+NQ9U/cz2t7FAAzTgi3t3n+t7pJLw6HS1VfWFp5lCZoNIFrqtA31gArwIB1pN3Vf
0bX/cbBErRUCgYEA090J+ez4AaH6pMhI/3hRpNh5O6NS7+oem/tN6K0WkstCwLnI
oD1mVbXKqXlhtEmhpS18JtTKBp4eONhMBZacaatJ+5OU596PZS1kpNn41Q8xxOj4
z+3/yuWhOwg8l5+Pd0hPVf42+sPNx0GpCEu2W+/y7GYtJPeDKP7/Xp2vhJsCgYEA
u42WQ3GN79Nio4asv4P8REelnVvnACs4ZtNYQBD29VIcwPJVk5PAC7IeV6PHyuMu
eg+fbgPx7x+W/1Ac1x2PD8gQiq2fYtOm3VVHuAAedEadWaI0vNHyEAmC63MJ2dMo
Ap+MugbYXuOjY/qPaWqHnz7hS30JKDR4jM69kiKhEtUCgYAvoBllUz5kUfMUXyN2
E0hwS8gp7mRMralSnWZDsNzQJVW14gW9eR6WJ2PA3Kc6aw9u42TT6FSqJAJ5rgW4
RjOduoRFwUzdDFa2V1TjppeFG8URQY1/NEEgBs0BK4TPHuJQWgTjr0IZ5PD8m85g
9p7VbZ+d8JxFC/8xccjsfVcX0A==
-----END PRIVATE KEY-----";

use crate::AppError;
use rsa::{pkcs8::{DecodePrivateKey},Pkcs1v15Encrypt, RsaPublicKey, RsaPrivateKey};
use tracing::error;
lazy_static::lazy_static!{
    static ref PRIV_KEY : RsaPrivateKey = RsaPrivateKey::from_pkcs8_pem(PRIV_KEY_STR).expect("failed to get private_encryption_key");
    static ref PUB_KEY : RsaPublicKey = PRIV_KEY.to_public_key(); 
}

pub async fn encrypt(value : &str) -> Result<Vec<u8>, AppError>{
    let mut rand = rand::thread_rng();

    match RsaPublicKey::encrypt(&PUB_KEY, &mut rand,Pkcs1v15Encrypt, value.as_bytes()){
        Ok(r) => Ok(r),
        Err(err) =>{ 
            error!("❌Failed to encrypt pass: {}", err);
            Err(AppError::InternalServerError("failed to encrypt password".to_string()))
        },
    }
}

pub async fn decrypt(value : Vec<u8>) -> Result<String,AppError>{
    match PRIV_KEY.decrypt(Pkcs1v15Encrypt, &value){
        Ok(res) => {
            // res is in Vec<u8>
            // convert it to string before returning
            let res = match String::from_utf8(res){
                Ok(r) => r,
                Err(err) => {
                    error!("❌ Failed to convert decrypted password to String: {}", err);
                    return Err(AppError::InternalServerError("decrypt password error".to_string()))}
            };
            Ok(res)
        },
        Err(err) => {
            error!("❌ Failed to decrypt the stored_password: {}", err);
            Err(AppError::InternalServerError("password couldn't be decrypted".to_string()))
        }
    }
}