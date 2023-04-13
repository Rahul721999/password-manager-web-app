use dotenv::dotenv;
use tracing::{Subscriber, subscriber::{set_global_default}};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry, fmt::MakeWriter};

pub fn get_subscriber<Sink>(
    sink : Sink
)-> impl Subscriber + Send + Sync
    where Sink: for<'a> MakeWriter<'a> + Sync + Send + 'static,
{
    dotenv().ok();
    let name = std::env::var("PROJECT_NAME").expect("Failed to load project name");
    let log_lvl = std::env::var("RUST_LOG").expect("Failed to set RUST_LOG");
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(log_lvl));
    let formatter_layer = BunyanFormattingLayer::new(name, 
        //output the formatted spans to stdout
        sink
    );

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatter_layer)
    }
    
    pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send){
        LogTracer::init().expect("Failed to set logger");
        set_global_default(subscriber).expect("Failed to set the subscriber");
    
}