use std::fmt::Display;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct UserCred {
    pub id: Uuid,
    pub auth_provider: AuthProvider,
    pub email: String,
    pub google_id: Option<String>,
    pub password_hash: Option<String>,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type, PartialEq)]
#[sqlx(type_name = "auth_provider_enum", rename_all = "lowercase")]
pub enum AuthProvider {
    #[sqlx(rename = "email")]
    Email,
    #[sqlx(rename = "google")]
    Google,
}

impl Display for AuthProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Email => write!(f, "email"),
            Self::Google => write!(f, "google"),
        }
    }
}
impl Default for AuthProvider {
    fn default() -> Self {
        Self::Email
    }
}
