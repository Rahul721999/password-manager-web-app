use actix_limitation::Limiter;
use actix_web::dev::ServiceRequest;
use std::time::Duration;

pub fn initialize_limiter(redis_url: &str) -> Limiter {
    let limiter = Limiter::builder(redis_url)
        .key_by(|req: &ServiceRequest| {
            req.connection_info()
                .realip_remote_addr()
                .map(|ip| ip.to_string())
        })
        .limit(1)
        .period(Duration::from_secs(1))
        .build()
        .expect("failed to set rate limiter");
    limiter
}
