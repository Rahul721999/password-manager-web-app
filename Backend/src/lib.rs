pub mod configs;
pub use configs::{ApplicationSettings, DatabaseSettings, Settings};

pub mod user_routes;
pub use user_routes::{del_acc, health_check::greet, login, sign_up, google_oauth::google_auth};

pub mod feature_route;
pub use feature_route::{
    delete_details::delete, fetch_all::fetch_all, fetch_details::fetch, gen_pass::generate,
    store_details::store, update_details::update,
};

pub mod models;
pub use models::{user_data_schema::UserData, user_schema::UserCred};

pub mod utils;
pub use utils::{
    error_module::AppError,
    gen_token::TokenClaims,
    hashing::{hash_pass, verify_pass},
    pass_strength::{analyze_pass, generate_pass},
    validator_module::{valid_email, valid_password},
};

pub mod log_config;
pub use log_config::{get_subscriber, init_subscriber, DomainSpanBuilder};

pub mod middleware;
pub use middleware::MyMiddleware;

lazy_static::lazy_static! {
    static ref EMAIL_REGEX : regex::Regex = match regex::Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})"){
        Ok(reg) => reg,
        Err(_) =>{
            tracing::error!("failed to set the email regex");
            panic!("Failed to set the email_regex");
        }
    };
}
