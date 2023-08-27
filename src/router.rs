mod notion_contact;
mod notion_porto;

use axum::Json;
use notion_contact::{convert_to_json, push_notion};
use notion_porto::get_porto;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;

use crate::router::notion_porto::extract_payload;

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

pub async fn api_porto() -> Result<Json<Vec<Value>>, (StatusCode, String)> {
    let bearer = env::var("BEARER").expect("BEARER not found");

    let bearer = format!("Bearer {bearer}");
    dbg!(&bearer);
    let porto_id = env::var("NOTION_PORTO_ID").expect("NOTION_PORTO_ID key not found");
    let notion_url = format!("https://api.notion.com/v1/databases/{}/query", porto_id);
    dbg!(&porto_id);
    dbg!(&notion_url);

    let payload = json!({
        "filter": {
            "property": "Status",
            "status": {
                "equals":"Done",
            },
        },
    });

    let res = get_porto(&payload, &bearer, &notion_url).await;

    match res {
        Ok(data) => {
            let value_response = extract_payload(&data);
            match value_response {
                Some(formated_resp) => Ok(Json(formated_resp)),
                None => Err((
                    StatusCode::NOT_FOUND,
                    "Error when extracting data".to_owned(),
                )),
            }
        }
        Err(err) => Err((StatusCode::NOT_FOUND, err.to_string())),
    }
}
