use actix_web::{HttpServer, App, web, middleware::{self, Logger}};
use tracing::info;
use crate::Config;
use lib::{health_check::greet, sign_up::sign_up};
use tracing_actix_web::TracingLogger;

pub async fn start(config : Config) -> std::io::Result<()>{
    //get the db
    let db = Config::run(config.db_url).await;
    
    //start the app
    info!("Starting server at {}:{}",config.host, config.port);
    HttpServer::new( move ||{
        App::new()
            .app_data(web::Data::new(db.clone()))
            // .wrap(Logger::default())
            .wrap(TracingLogger::default())
            .route("/health-check", web::get().to(greet))
            .service(
                web::scope("/Auth")
                    // .wrap(Logger::default())
                    .wrap(TracingLogger::default())    
                    .route("/SignUp", web::post().to(sign_up))
            )
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;
    Ok(())
}