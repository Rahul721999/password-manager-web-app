use actix_web::{Responder, HttpResponse};
use tracing::info;

#[tracing::instrument]
pub async fn greet() -> impl Responder {
    info!("Health-Check successfull");
    HttpResponse::Ok()
}
