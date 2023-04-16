use actix_web::{Responder, HttpResponse};
use serde_json::json;
use tracing::info;
#[tracing::instrument(
    name ="Health-Check"
    skip_all
)]
pub async fn greet() -> impl Responder {
    info!("health-check fn called");
    HttpResponse::Ok()
        .json(json!({"status" : "Success"}))
}
