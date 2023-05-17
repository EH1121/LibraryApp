use actix_web::{HttpResponse, web::{self, Data}};
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::{actions::client::EClientTesting, handlers::errors::ErrorTypes, APPLICATION_LIST_NAME};

use super::libs::{{create_or_exists_index, is_server_up, insert_new_app_name, search_body_builder, get_document, index_name_builder}, get_app_indexes_list};
use super::structs::applications_struct::*;

// Since there must always be an application list, this will always create one if it doesnt exist
pub async fn initialize_new_app_id(data: web::Json<RequiredAppName>, client: Data::<EClientTesting>) -> HttpResponse{

    if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }

    let _ = create_or_exists_index(None, APPLICATION_LIST_NAME, None, None, &client).await;

    let app_id_status = insert_new_app_name(&data.app_name, &client).await;

    match app_id_status {
        StatusCode::CREATED => {
            HttpResponse::Created().finish()
        },
        StatusCode::CONFLICT => {
            HttpResponse::Conflict().finish()
        },
        _ => {
            HttpResponse::build(app_id_status).json(json!({"error": ErrorTypes::Unknown.to_string()}))
        }
    }
}

pub async fn get_application_list(data: web::Path<OptionalAppName>, client: Data::<EClientTesting>) -> HttpResponse{

    if !is_server_up(&client).await { return HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(json!({"error": ErrorTypes::ServerDown.to_string()}))}

    let body = search_body_builder(&data.app_name.clone(), &None, &Some("_id,name,indexes".to_string()));
    let json_resp = client.search_index(APPLICATION_LIST_NAME, &body, &None, &None).await.unwrap().json::<Value>().await.unwrap();
    HttpResponse::Ok().json(json!(
        json_resp["hits"]["hits"]
    ))
}


pub async fn get_application(data: web::Path<RequiredAppID>, client: Data::<EClientTesting>) -> HttpResponse{

    if !is_server_up(&client).await { return HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(json!({"error": ErrorTypes::ServerDown.to_string()}))}

    let resp = get_document(APPLICATION_LIST_NAME, &data.app_id, &Some("_id,name,indexes".to_string()), &client).await;

    match resp {
        Ok((code, value)) => HttpResponse::build(code).json(value),
        Err((code, error)) => HttpResponse::build(code).json(json!({"error": error.to_string()})) 
    }
}

pub async fn update_application(data: web::Json<UpdateApp>, client: Data::<EClientTesting>) -> HttpResponse{

    if !is_server_up(&client).await { return HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(json!({"error": ErrorTypes::ServerDown.to_string()}))}

    let body = json!({
        "doc": {
            "name": &data.app_name
        }
    });

    let resp = client.update_document(APPLICATION_LIST_NAME, &data.app_id, &body).await.unwrap();

    HttpResponse::build(resp.status_code()).finish()
}

pub async fn delete_application(data: web::Path<RequiredAppID>, client: Data::<EClientTesting>) -> HttpResponse{

    if !is_server_up(&client).await { return HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(json!({"error": ErrorTypes::ServerDown.to_string()}))}

    match get_app_indexes_list(&data.app_id, &client).await {
        Ok(x) => {
            for i in x {
                let _ = client.delete_index(&index_name_builder(&data.app_id, &i)).await;
            }
        },
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()})),
    }

    let resp = client.delete_document(APPLICATION_LIST_NAME, &data.app_id).await.unwrap();

    HttpResponse::build(resp.status_code()).finish()
}