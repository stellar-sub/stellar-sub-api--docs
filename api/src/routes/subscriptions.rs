use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use std::sync::Arc;

use crate::models::AppState;

pub async fn get_subscriptions(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let subscriptions = match state.subscriptions.read() {
        Ok(subscriptions) => subscriptions,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to read subscriptions" })),
            )
                .into_response()
        }
    };

    let all_subscriptions: Vec<_> = subscriptions.values().cloned().collect();

    (StatusCode::OK, Json(json!({ "subscriptions": all_subscriptions }))).into_response()
}

pub async fn get_subscription_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let subscriptions = match state.subscriptions.read() {
        Ok(subscriptions) => subscriptions,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to read subscriptions" })),
            )
                .into_response()
        }
    };

    match subscriptions.get(&id) {
        Some(subscription) => {
            (StatusCode::OK, Json(json!({ "subscription": subscription }))).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Subscription not found" })),
        )
            .into_response(),
    }
}
