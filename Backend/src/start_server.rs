use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware::{Compat},
    web, App, HttpServer,
};
use lib::*;
use tracing::info;
use tracing_actix_web::TracingLogger;

pub async fn start(config: Settings) -> std::io::Result<()> {
    //get the db
    let app = config.application.clone();
    let db = config.run();
    let frontend_url = config.frontend.url.clone();
    let configuration = web::Data::new(config);
    //start the app
    info!("🚀 Starting server at {}:{}", app.host, app.port);
    info!("⚠️ Log-Level : {}", app.log_level.clone());
    HttpServer::new(move || {
        // set cors
        
        let cors = Cors::default()
            .allowed_origin(&frontend_url)
            .allowed_methods(vec!["GET", "POST","DELETE", "PATCH","PUT"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(TracingLogger::default())
            .route("/", web::get().to(greet))
            .service(
                web::scope("/Auth")
                    .app_data(web::Data::new(db.clone()))
                    .app_data(configuration.clone())
                    .wrap(Compat::new(TracingLogger::default()))
                    .route("/SignUp", web::post().to(sign_up))
                    .route("/LogIn", web::post().to(login))
                    .route("/Delete-acc", web::delete().to(del_acc)),
            )
            .service(
                web::scope("/User")
                    .app_data(web::Data::new(db.clone()))
                    .app_data(configuration.clone())
                    .wrap(Compat::new(TracingLogger::default()))
                    .route("/store", web::put().to(store))
                    .route("/get", web::get().to(fetch))
                    .route("/get_all", web::get().to(fetch_all))
                    .route("/update", web::patch().to(update))
                    .route("/delete", web::delete().to(delete))
                    .route("/generate_password", web::get().to(generate))
            )
            .wrap(TracingLogger::<DomainSpanBuilder>::new())
            .app_data(web::Data::new(db.clone()))
            .app_data(configuration.clone())
    })
    .bind(format!("{}:{}", app.host, app.port))?
    .run()
    .await?;
    Ok(())
}
