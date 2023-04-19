use actix_web::{HttpServer, App, web, middleware::{self, Logger, Compat}};
use tracing::info;
use lib::{health_check::greet, 
    sign_up::sign_up, 
    login,
    del_acc,
    DomainSpanBuilder, 
    config::{Config, run},
    MyMiddleware
};
use tracing_actix_web::TracingLogger;

pub async fn start(config : Config) -> std::io::Result<()>{
    //get the db
    let configuration = web::Data::new(config.clone());
    let db = run(&config.db_url).await;
    let middleware = MyMiddleware{};
    //start the app
    info!("🚀 Starting server at {}:{}",config.host, config.port);
    HttpServer::new( move ||{
        App::new()
            .wrap(TracingLogger::default())
            .wrap(middleware)
            .route("/health-check", web::get().to(greet))
            .service(
                web::scope("/Auth")
                    .app_data(web::Data::new(db.clone()))
                    .app_data(configuration.clone())
                    .wrap(Compat::new(TracingLogger::default()))    
                    .route("/SignUp", web::post().to(sign_up))
                    .route("/LogIn", web::post().to(login))
                    .route("/Delete-acc", web::post().to(del_acc))
            )
            .wrap(TracingLogger::<DomainSpanBuilder>::new())
            .app_data(web::Data::new(db.clone()))
            .app_data(configuration.clone())
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;
    Ok(())
}