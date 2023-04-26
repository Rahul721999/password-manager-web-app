
use serde::Deserialize;
use sqlx::types::Uuid;
#[derive(Debug, Deserialize, Clone, sqlx::FromRow)]
pub struct UserData{
    pub id : Uuid,
    pub user_id : Uuid,
    pub website_name : String,
    pub website_url : String,
    pub username : String,
    pub password_hash  : String,
    pub created_at : usize,
    pub updated_at : usize,
}
