mod indexer;
mod models;
mod routes;

use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::indexer::Indexer;
use crate::models::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "stellar_subscriptions_api=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = Arc::new(AppState::new());

    let indexer = Indexer::new(Arc::clone(&state));
    tokio::spawn(async move {
        indexer.start().await;
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/plans", get(routes::plans::get_plans))
        .route("/plans/:id", get(routes::plans::get_plan_by_id))
        .route("/subscriptions", get(routes::subscriptions::get_subscriptions))
        .route("/subscriptions/:id", get(routes::subscriptions::get_subscription_by_id))
        .route("/merchants/:address/plans", get(routes::merchants::get_merchant_plans))
        .route("/merchants/:address/subscriptions", get(routes::merchants::get_merchant_subscriptions))
        .route("/stats", get(routes::stats::get_stats))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("Failed to bind to port 3001");

    tracing::info!("API server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
