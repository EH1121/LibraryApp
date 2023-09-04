use actix_web::{HttpResponse, web::{self, Data}, http::StatusCode};
use serde_json::{json, Value};
use crate::{USER_LIST, database::Database, libs::*};
use super::structs::*;

// Ambil list usernya
pub async fn get_user_list(path: web::Path<GetUserList>, db: Data::<Database>) -> HttpResponse{
    // Cek kalo elastic hidup
    if !check_server(&db).await { return HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(json!({"error": Errors::ServerDown.to_string()}))}

    // Harus selalu ada list usernya
    create_new_genre(None, USER_LIST, &db).await;

    // Ambil semua usernya
    let body =
        json!({
            "_source": {
                "includes": "*"
            },
            "query": {
                "match_all": {} 
            },
        });
    HttpResponse::Ok().json(db.search(USER_LIST, &body, path.from, path.count).await.unwrap().json::<Value>().await.unwrap()["hits"]["hits"].clone())
}

/// Ambil data satu user
pub async fn get_a_user(path: web::Path<UserID>, db: Data::<Database>) -> HttpResponse{
    // Cek kalo elastic hidup
    if !check_server(&db).await { return HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(json!({"error": Errors::ServerDown.to_string()}))}

    // Harus selalu ada list usernya
    create_new_genre(None, USER_LIST, &db).await;

    // Ambil data dari satu user
    match get_book(USER_LIST, &path.user_id, Some("_id,name,genres".to_string()), &db).await {
        Ok((s, v)) => HttpResponse::build(s).json(v),
        Err((s, e)) => match e {
            Errors::BookNotFound(_) => HttpResponse::build(s).json(json!({"error": Errors::UserNotFound(path.user_id.to_string()).to_string()})),
            _ => HttpResponse::build(s).json(json!({"error": e.to_string()}))
        }
    }
}

/// Buat user baru
pub async fn create_new_user(data: web::Json<UserName>, db: Data::<Database>) -> HttpResponse{
    // Cek kalo elastic hidup
    if !check_server(&db).await { return HttpResponse::ServiceUnavailable().json(json!({"error": Errors::ServerDown.to_string()})) }

    // Harus selalu ada list usernya
    create_new_genre(None, USER_LIST, &db).await;

    // Bikin user baru
    HttpResponse::build(db.index_documents(USER_LIST, &vec![json!({"name": data.user_name})]).await.unwrap().status_code()).finish()
}

// Update data satu user
pub async fn update_user(data: web::Json<UpdateUser>, db: Data::<Database>) -> HttpResponse{
    // Cek kalo elastic hidup
    if !check_server(&db).await { return HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(json!({"error": Errors::ServerDown.to_string()}))}

    // Update data user
    HttpResponse::build(db.update_single_document(USER_LIST, &data.user_id,json!({"name": &data.user_name})).await.unwrap().status_code()).finish()
}

// Hapus satu user
pub async fn delete_user(path: web::Path<UserID>, db: Data::<Database>) -> HttpResponse{
    // Cek kalo elastic hidup
    if !check_server(&db).await { return HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(json!({"error": Errors::ServerDown.to_string()}))}

    // Ambil list genre dari usernya
    match get_user_genre_list(&path.user_id, &db).await {

        // Kalo user ada, ambil genrenya
        Ok(l) => {
            // Loop untuk hapus semua genrenya 
            for i in l {
                let _ = db.delete_single_index(format!("{}.{}", &path.user_id.to_lowercase(), &i)).await;
            }

            // Lalu hapus usernya
            HttpResponse::build(db.delete_single_document(USER_LIST, &path.user_id).await.unwrap().status_code()).finish()
        },

        // Kalo ga ketemu
        Err((s, e)) => HttpResponse::build(s).json(json!({"error": e.to_string()})),
    }
}