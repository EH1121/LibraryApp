// use elasticsearch::http::response::Response;
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::{actions::EClientTesting, handlers::{errors::ErrorTypes, libs::document::search_body_builder}, APPLICATION_LIST_NAME};

use super::document::get_document;

/// Inserts a new app name to the application list
pub async fn insert_new_app_name(app_name: &str, client: &EClientTesting) -> StatusCode {
    let exists = exists_app_name(app_name, client).await;

    // If exists, return conflict
    if exists {
        return StatusCode::CONFLICT;
    }

    let body = json!({
        "name": app_name,
        "indexes": []
    });

    // Inserts name into app_id
    client.insert_document(APPLICATION_LIST_NAME, &body).await.unwrap().status_code()
}

pub async fn get_app_indexes_list(app_id: &str, client: &EClientTesting) -> Result<Vec<String>, (StatusCode, ErrorTypes)> {
    let (_, value) = match get_document(APPLICATION_LIST_NAME, app_id, &Some("indexes".to_string()), client).await{
        Ok(x) => x,
        Err((status, _)) => return match status {
            StatusCode::NOT_FOUND => Err((status, ErrorTypes::ApplicationNotFound(app_id.to_string()))),
            _ => Err((status, ErrorTypes::Unknown))
        },
    };

    let list: Vec<String> = match value.get("indexes") {
        Some(x) => serde_json::from_value(x.clone()).unwrap(),
        None => Vec::new()
    };
    Ok(list)
}

pub async fn exists_app_name(app_name: &str, client: &EClientTesting) -> bool{
    let app_name_exact = format!("\"{app_name}\"");
    let body = search_body_builder(&Some(app_name_exact), &Some(vec!["name".to_string()]), &None);
    let resp = client.search_index(APPLICATION_LIST_NAME, &body, &None, &Some(1)).await.unwrap();
    let resp_json = resp.json::<Value>().await.unwrap();
    let num = resp_json["hits"]["total"]["value"].as_i64().unwrap();
    num > 0
}
