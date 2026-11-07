use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use std::sync::Arc;

use crate::models::AppState;

pub async fn get_stats(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.get_stats() {
        Ok(stats) => (StatusCode::OK, Json(json!({ "stats": stats }))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to compute stats: {}", e) })),
        )
            .into_response(),
    }
}
