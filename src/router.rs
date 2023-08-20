mod notion_contact;

use axum::Json;
use notion_contact::{convert_to_json, push_notion};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct ContactPayload {
    name: String,
    email: String,
    message: String,
}

pub async fn push_contact(
    Json(payload): Json<ContactPayload>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let extracted_val = convert_to_json(&payload);
    let response_json = push_notion(extracted_val).await;

    match response_json {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err((StatusCode::NOT_FOUND, err.to_string())),
    }
}
