use axum::Router;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

pub trait LoggingRouterExt {
    fn add_logging(self) -> Self;
}

impl LoggingRouterExt for Router {
    fn add_logging(self) -> Self {
        self.layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
    }
}
