use super::ContactPayload;
use reqwest::Client;
use serde_json::{json, Value};
use std::env;

pub async fn push_notion(value: Value) -> Result<Value, reqwest::Error> {
    let notion_url = env::var("NOTION_URL").expect("NOTION_URL key not found");
    let bearer = env::var("BEARER").expect("BEARER not found");

    let bearer = format!("Bearer {bearer}");

    let client = Client::new();

    let resp: Value = client
        .post(notion_url)
        .header("Authorization", bearer)
        .header("Content-Type", "application/json")
        .header("Notion-Version", "2022-06-28")
        .json(&value)
        .send()
        .await?
        .json()
        .await?;

    Ok(resp)
}

pub fn convert_to_json(contact: &ContactPayload) -> Value {
    let notion_db = env::var("NOTION_DB_ID").expect("NOTION_DB_ID key not found");

    let payload = json!({
          "parent": {
            "database_id": notion_db,
          },
          "properties": {
            "Name": {
              "title": [
                {
                  "text": {
                    "content": contact.name,
                  },
                },
              ],
            },
            "email": {
              "email": contact.email,
            },
            "Message": {
              "rich_text": [
                {
                  "text": {
                    "content": contact.message,
                  },
                },
              ],
            },
            "Contacted": {
               "checkbox": false
              },
       },
    });
    payload
}
