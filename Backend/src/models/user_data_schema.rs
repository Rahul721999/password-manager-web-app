use chrono::{DateTime, Utc};
use sqlx::{types::Uuid, Encode, FromRow, Postgres, Type};
#[derive(Debug, Clone, FromRow, Encode)]
pub struct UserData {
    pub id: Uuid,
    pub user_id: Uuid,
    pub website_name: String,
    pub website_url: String,
    pub username: String,
    pub password_hash: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl Type<Postgres> for UserData {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("website_credentials")
    }
}
impl UserData {
    pub fn new(
        user_id: Uuid,
        website_name: String,
        website_url: String,
        username: String,
        password_hash: Vec<u8>,
    ) -> Self {
        UserData {
            id: Uuid::new_v4(),
            user_id,
            website_name,
            website_url,
            username,
            password_hash,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
// use sqlx::postgres::{PgArguments, PgTypeInfo, PgArgumentBuffer};
// use sqlx::{Type, Postgres};
// impl<'q> Encode<'q, Postgres> for UserData {
//     fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
//         buf.extend(&self.id);
//         buf.extend(&self.user_id);
//         buf.extend(&self.website_name);
//         buf.extend(&self.website_url);
//         buf.extend(&self.username);
//         buf.extend(&self.password_hash);
//         buf.extend(&self.created_at);
//         buf.extend(&self.updated_at);
//         sqlx::encode::IsNull::No
//     }
// }
