// #![allow(unused)]
use serde::Deserialize;
use sqlx::types::Uuid;
#[derive(Debug, Deserialize, Clone, sqlx::FromRow)]
pub struct UserCred {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
}
