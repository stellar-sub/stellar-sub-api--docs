use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use std::sync::Arc;

use crate::models::AppState;

pub async fn get_merchant_plans(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> impl IntoResponse {
    let plans = match state.plans.read() {
        Ok(plans) => plans,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to read plans" })),
            )
                .into_response()
        }
    };

    let merchant_plans: Vec<_> = plans
        .values()
        .filter(|p| p.merchant == address)
        .cloned()
        .collect();

    (StatusCode::OK, Json(json!({ "plans": merchant_plans }))).into_response()
}

pub async fn get_merchant_subscriptions(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
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

    let merchant_subscriptions: Vec<_> = subscriptions
        .values()
        .filter(|s| s.merchant == address)
        .cloned()
        .collect();

    (StatusCode::OK, Json(json!({ "subscriptions": merchant_subscriptions }))).into_response()
}
