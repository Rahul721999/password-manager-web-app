use actix_web::{Responder, HttpResponse};
use serde_json::json;
use tracing::info;

#[tracing::instrument(
    name ="🚩Health-Check"
    skip_all
)]
pub async fn greet() -> impl Responder {
    info!("✅ Health-check fn called");
    HttpResponse::Ok()
        .json(json!({"message" : "Health-Check Successful"}))
}
