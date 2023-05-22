use actix_web::{HttpResponse, web::{self, Data}, http::StatusCode};
use serde_json::{json, Value};
use crate::{USER_LIST, database::Database, libs::*};
use super::structs::*;

pub async fn get_user_list(path: web::Path<GetUserList>, db: Data::<Database>) -> HttpResponse{
    if !check_server(&db).await { return HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(json!({"error": Errors::ServerDown.to_string()}))}
    create_new_genre(None, USER_LIST, &db).await;
    let body = search_body(&path.user_name.clone(), &None, &Some("*".to_string()));
    HttpResponse::Ok().json(db.search(USER_LIST, &body, path.from, path.count).await.unwrap().json::<Value>().await.unwrap()["hits"]["hits"].clone())
}

pub async fn get_a_user(path: web::Path<UserID>, db: Data::<Database>) -> HttpResponse{
    if !check_server(&db).await { return HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(json!({"error": Errors::ServerDown.to_string()}))}
    create_new_genre(None, USER_LIST, &db).await;
    match get_book(USER_LIST, &path.user_id, Some("_id,name,genres".to_string()), &db).await {
        Ok((s, v)) => HttpResponse::build(s).json(v),
        Err((s, e)) => match e {
            Errors::BookNotFound(_) => HttpResponse::build(s).json(json!({"error": Errors::UserNotFound(path.user_id.to_string()).to_string()})),
            _ => HttpResponse::build(s).json(json!({"error": e.to_string()}))
        }
    }
}

pub async fn create_new_user(data: web::Json<UserName>, db: Data::<Database>) -> HttpResponse{
    if !check_server(&db).await { return HttpResponse::ServiceUnavailable().json(json!({"error": Errors::ServerDown.to_string()})) }
    create_new_genre(None, USER_LIST, &db).await;
    HttpResponse::build(db.index_documents(USER_LIST, &vec![json!({"name": data.user_name})]).await.unwrap().status_code()).finish()
}

pub async fn update_user(data: web::Json<UpdateUser>, db: Data::<Database>) -> HttpResponse{
    if !check_server(&db).await { return HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(json!({"error": Errors::ServerDown.to_string()}))}
    HttpResponse::build(db.update_single_document(USER_LIST, &data.user_id,json!({"name": &data.user_name})).await.unwrap().status_code()).finish()
}

pub async fn delete_user(path: web::Path<UserID>, db: Data::<Database>) -> HttpResponse{
    if !check_server(&db).await { return HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(json!({"error": Errors::ServerDown.to_string()}))}
    match get_user_genre_list(&path.user_id, &db).await {
        Ok(l) => {
            for i in l {
                let _ = db.delete_single_index(format!("{}.{}", &path.user_id.to_lowercase(), &i)).await;
            }
            HttpResponse::build(db.delete_single_document(USER_LIST, &path.user_id).await.unwrap().status_code()).finish()
        },
        Err((s, e)) => HttpResponse::build(s).json(json!({"error": e.to_string()})),
    }
}