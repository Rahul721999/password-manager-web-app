pub mod config;
pub use config::{Config, run};

pub mod user_routes;
pub use user_routes::{
    health_check,sign_up, login, del_acc,
};

pub mod feature_route;
pub use feature_route::{
    
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
    pass_strength::{analyze_pass},
};

pub mod log_config;
pub use log_config::{get_subscriber, init_subscriber, DomainSpanBuilder};

pub mod middleware;
pub use middleware::MyMiddleware;