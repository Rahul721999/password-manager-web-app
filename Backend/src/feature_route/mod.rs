pub mod store_details;
pub use store_details::store;

pub mod fetch_details;
pub use fetch_details::fetch;

pub mod fetch_all;
pub use fetch_all::fetch_all;

pub mod delete_details;
pub use delete_details::delete;

pub mod update_details;
pub use update_details::update;

pub mod gen_pass;
pub use gen_pass::generate;
