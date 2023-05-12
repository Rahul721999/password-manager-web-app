pub mod configs;
pub use configs::{Settings, DatabaseSettings, ApplicationSettings};


pub mod user_routes;
pub use user_routes::{
    health_check::greet,sign_up, login, del_acc,
};

pub mod feature_route;
pub use feature_route::{
    store_details::store,
    fetch_details::fetch,
    fetch_all::fetch_all,
    update_details::update,
    delete_details::delete,
    gen_pass::generate,
};

pub mod models;
pub use models::{
    user_schema::UserCred,
    user_data::UserData,
};


pub mod utils;
pub use utils::{
    hashing::{hash_pass, verify_pass},
    error_module::{AppError},
    validator_module::{valid_email, valid_password},
    gen_token::{TokenClaims},
    pass_strength::{analyze_pass, generate_pass},
};

pub mod log_config;
pub use log_config::{get_subscriber, init_subscriber, DomainSpanBuilder};

pub mod middleware;
pub use middleware::MyMiddleware;


lazy_static::lazy_static!{
    static ref EMAIL_REGEX : regex::Regex = match regex::Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})"){
        Ok(reg) => reg,
        Err(_) =>{
            tracing::error!("failed to set the email regex");
            panic!("Failed to set the email_regex");
        }
    };
}