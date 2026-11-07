use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use std::sync::Arc;

use crate::models::AppState;

pub async fn get_plans(State(state): State<Arc<AppState>>) -> impl IntoResponse {
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

    let active_plans: Vec<_> = plans
        .values()
        .filter(|p| p.is_active)
        .cloned()
        .collect();

    (StatusCode::OK, Json(json!({ "plans": active_plans }))).into_response()
}

pub async fn get_plan_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
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

    match plans.get(&id) {
        Some(plan) => (StatusCode::OK, Json(json!({ "plan": plan }))).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Plan not found" })),
        )
            .into_response(),
    }
}
