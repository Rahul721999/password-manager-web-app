// #![allow(unused)]
use serde::Deserialize;
use sqlx::types::Uuid;
#[derive(Debug, Deserialize, Clone, sqlx::FromRow)]
pub struct UserCred{
    pub id : Uuid,
    pub email : String,
    pub password_hash : String,
    pub first_name : String,
    pub last_name : String,

}

// impl UserCred{
//     pub fn new(
//         email : String, 
//         password_hash : String, 
//         first_name: String, 
//         last_name : String
//     ) -> Self{
//         let id = bson::Uuid::new();
//         UserCred { id, email, password_hash, first_name, last_name}
//     }
// }

// impl fmt::Display for UserCred{
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f,"{}, {}, {}, {}, ",self.email,self.password_hash,self.first_name, self.last_name)
//     }
// }