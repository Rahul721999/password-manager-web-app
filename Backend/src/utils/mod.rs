pub mod hashing;
pub use hashing::hash_pass;

pub mod error_module;
pub use error_module::*;

pub mod validator_module;
pub use validator_module::{valid_email, valid_password};