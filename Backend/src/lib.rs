pub mod config;
pub use config::{Config, run};

pub mod user_routes;
pub use user_routes::*;

pub mod models;
pub use models::user_schema::*;


pub mod utils;
pub use utils::{
    hashing::{hash_pass, verify_pass},
    error_module::{AppError},
    validator_module::{valid_email, valid_password},
    gen_token::{TokenClaims},
};

pub mod log_config;
pub use log_config::{get_subscriber, init_subscriber, DomainSpanBuilder};

pub mod middleware;
pub use middleware::MyMiddleware;