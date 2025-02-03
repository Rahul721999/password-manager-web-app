use tracing::{subscriber::set_global_default, Subscriber};
use tracing_actix_web::{DefaultRootSpanBuilder, RootSpanBuilder};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

use crate::ApplicationSettings;

pub fn get_subscriber<Sink>(app: &ApplicationSettings, sink: Sink) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Sync + Send + 'static,
{
    let name = app.name.clone();
    let env_filter = EnvFilter::new(app.log_level.clone());
    let formatter_layer = BunyanFormattingLayer::new(
        name, //output the formatted spans to stdout
        sink,
    );

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatter_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set the subscriber");
}

pub struct DomainSpanBuilder;

impl RootSpanBuilder for DomainSpanBuilder {
    fn on_request_start(request: &actix_web::dev::ServiceRequest) -> tracing::Span {
        tracing_actix_web::root_span!(
            request,
            client_id = tracing::field::Empty,
            target = tracing::field::Empty
        )
    }
    fn on_request_end<B: actix_web::body::MessageBody>(
        span: tracing::Span,
        outcome: &Result<actix_web::dev::ServiceResponse<B>, actix_web::Error>,
    ) {
        DefaultRootSpanBuilder::on_request_end(span, outcome);
    }
}
