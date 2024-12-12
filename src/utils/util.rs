use crate::utils::enums::{MediaStatus, MediaType};
use crate::utils::models::QUERY;
use reqwest::Client;
use serde_json::json;

#[derive(Debug)]
pub struct QueryOptions {
    pub media_type: MediaType,
    pub page: i32,
    pub per_page: i32,
    pub status: Option<MediaStatus>,
}

pub async fn get_result(json: String) -> serde_json::Value {
    let client = Client::new();
    let response = client
        .post("https://graphql.anilist.co/")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(json)
        .send()
        .await
        .unwrap()
        .text()
        .await;

    serde_json::from_str(&response.unwrap()).unwrap()
}

pub async fn fetch(options: QueryOptions) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let variables = json!({
        "page": options.page,
        "perPage": options.per_page,
        "type": options.media_type,
        "status": options.status,
    });

    let body = json!({
        "query": QUERY,
        "variables": variables,
    });

    let result = get_result(body.to_string()).await;
    Ok(result)
}
