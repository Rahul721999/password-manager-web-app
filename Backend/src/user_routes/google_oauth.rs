use crate::{models::AuthProvider, AppError, Settings, TokenClaims, UserCred};
use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleAuthRequest {
    pub token: String,
}

#[derive(Deserialize, Clone)]
struct GoogleClaims {
    sub: String, // Google user ID
    email: String,
    given_name: Option<String>,
    family_name: Option<String>,
}

#[tracing::instrument(
    name = "🚩 OAuth request"
    skip (db, config)
)]
pub async fn google_auth(
    payload: web::Json<GoogleAuthRequest>,
    db: web::Data<PgPool>,
    config: web::Data<Settings>,
) -> Result<HttpResponse, AppError> {
    // Fetch the token from the payload
    let google_token = &payload.token;

    // Step 1: Verify Google ID Token
    let google_user = match verify_google_token(&google_token).await {
        Ok(user) => user,
        Err(_err) => return Err(AppError::AuthError("UnAuthorized".to_string())),
    };

    // Step 2: Check if the user exists
    let record = match sqlx::query_as::<_, UserCred>(
        "SELECT id, auth_provider, email, google_id, password_hash, first_name, last_name 
        FROM user_cred WHERE google_id = $1",
    )
    .bind(google_user.sub.clone())
    .fetch_optional(db.get_ref())
    .await
    {
        Ok(row) => row,
        Err(err) => {
            return Err(AppError::InternalServerError(format!("Error : {}", err)));
        }
    };

    // Step 2.1: Add user to the database if user doesn't exist
    let user_id = if let Some(user) = record {
        user.id
    } else {
        handle_oauth_signup(db, &google_user).await?
    };

    // 3. generate & return JWT
    let claim = TokenClaims {
        id: user_id,
        email: google_user.email.clone(),
        exp: (Utc::now() + Duration::seconds(config.application.jwt_exp as i64)).timestamp()
            as usize,
    };

    let token = match claim.generate(&config) {
        Ok(token) => token,
        Err(_err) => {
            tracing::error!("❌ failed to generate token");
            return Err(AppError::InternalServerError(
                "Failed to gen JWT".to_string(),
            ));
        }
    };

    Ok(
        HttpResponse::Ok()
            .json(serde_json::json!({"message" : "Success", "Authorization" : token})),
    )
}

// * ----------------------Helper function: Verify Google ID Token-------------------------
async fn verify_google_token(token: &str) -> Result<GoogleClaims, String> {
    let google_url = format!("https://oauth2.googleapis.com/tokeninfo?id_token={}", token);

    let response = reqwest::get(&google_url).await;
    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let claims: GoogleClaims = resp.json().await.map_err(|_| "Failed to parse token response".to_string())?;
                Ok(claims)
            } else {
                Err("Invalid Google token".to_string())
            }
        }
        Err(_) => Err("Failed to verify token".to_string()),
    }
}

// * ------------------------------Helper fn to handle oAuth SignUp------------------------------
async fn handle_oauth_signup(
    db: web::Data<PgPool>,
    google_user: &GoogleClaims,
) -> Result<Uuid, AppError> {
    let user_id = Uuid::new_v4(); // generate new Uuid

    let result = sqlx::query!(
        "INSERT INTO user_cred (id, auth_provider, google_id, email, first_name, last_name) 
         VALUES ($1, $2, $3, $4, $5, $6)",
        user_id,
        AuthProvider::Google as AuthProvider, // Use string instead of Enum
        google_user.sub,
        google_user.email,
        google_user.given_name.as_deref().unwrap_or(""),
        google_user.family_name.as_deref().unwrap_or(""),
    )
    .execute(db.as_ref())
    .await;

    match result {
        Ok(_) => {
            tracing::info!("✅ User added successfully");
            Ok(user_id) // Correct return type
        }
        Err(err) => {
            tracing::error!("❌ Failed to add User: {}", err);
            Err(AppError::InternalServerError(
                "Database insertion failed".to_string(),
            ))
        }
    }
}
