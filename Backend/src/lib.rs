pub mod config;

pub mod user_routes;
pub use user_routes::*;

pub mod models;
pub use models::user_schema::*;


pub mod utils;
pub use utils::{
    hashing::hash_pass,
    error_module::{AppError},
    validator_module::{valid_email, valid_password},
};

pub mod log_config;
pub use log_config::{get_subscriber, init_subscriber};