use std::collections::HashSet;
use actix_web::http::StatusCode;
use serde_json::{json, Value};
use crate::{database::Database, USER_LIST};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("Cannot find user with ID: {0}")]
    UserNotFound(String),
    #[error("Cannot find genre: {0}")]
    GenreNotFound(String),
    #[error("Genre already exist: {0}")]
    GenreExists(String),
    #[error("Cannot find book with ID: {0}")]
    BookNotFound(String),
    #[error("Bad Data Given")]
    BadRequest,
    #[error("Database server is offline")]
    ServerDown,
    #[error("Unknown error has occured")]
    Unknown
}

// Ambil buku
pub async fn get_book(genre: &str, book_id: &str, retrieve_fields: Option<String>, db: &Database) -> Result<(StatusCode, Value), (StatusCode, Errors)>{

    // Kirim permintaan ke elastic
    let response = db.get_single_document(genre, book_id, retrieve_fields).await.unwrap();
    
    // Kalo ga ketemu kasih eror
    if !response.status_code().is_success() {
        let e = match response.status_code(){
            StatusCode::NOT_FOUND => Errors::BookNotFound(book_id.to_string()),
            _ => Errors::Unknown
        };
        return Err((response.status_code(), e));
    }

    Ok((response.status_code(), response.json::<Value>().await.unwrap()))
}

// Buat genre baru
pub async fn create_new_genre(user_id: Option<String>, genre: &str, db: &Database) {
    // Kalo ada user idnya berarti ini mau masukin ke user, kalo engga, ini berarti mau bikin user
    let genre_index = match user_id {
        Some(x) => format!("{}.{}", x.to_lowercase(), &genre.to_lowercase()),
        None => genre.to_string()
    };

    // Bikin format data yang mau dikirim ke server untuk buat genre baru
    if db.get_indices(Some(genre_index.clone())).await.unwrap().status_code() == StatusCode::NOT_FOUND {
        let body = 
            json!(
                {
                    "mappings": { 	
                        "dynamic":"true",
                        "properties": {
                            "tanggal_terbit": {
                                "type": "date",
                                "format": "dd-MM-yyyy"
                            }
                        }
                    }
                }
            );
        db.create_single_index(&genre_index, &body).await.unwrap();
    }
}

// Cek kalo genre ada
pub async fn genre_exists(user_id: &str, genre: &str, db: &Database) -> Result<HashSet<String>, (StatusCode, Errors, HashSet<String>)> {
    match get_user_genre_list(user_id, db).await {
        Ok(l) => {
            // Cek kalo user punya genrenya
            match l.contains(genre) {
                true => Ok(l),
                false => Err((StatusCode::NOT_FOUND, Errors::GenreNotFound(genre.to_string()), l))
            }
        },
        Err((s, e)) => Err((s, e, HashSet::new()))
    }
}

/// Ambil list genre dari user
pub async fn get_user_genre_list(user_id: &str, db: &Database) -> Result<HashSet<String>, (StatusCode, Errors)> {
    match get_book(USER_LIST, user_id, Some("genres".to_string()), db).await{
        Ok((_, v)) => {
            match v.get("genres") {
                Some(x) => Ok(serde_json::from_value(json!(x)).unwrap()),
                None => Ok(HashSet::new())
            }
        },
        Err((code, _)) => match code {
            StatusCode::NOT_FOUND => Err((code, Errors::UserNotFound(user_id.to_string()))),
            _ => Err((code, Errors::Unknown))
        },
    }
}

// Cek kalo server hidup
pub async fn check_server(db: &Database) -> bool {
    db.get_indices(Some("".to_string())).await.is_ok()
}

// Cek kalo user sama genre ada
pub async fn check_userid_genre(user_id: &str, genre: &str, db: &Database) -> Result<(), (StatusCode, Errors)>{
    if check_server(db).await {
        match genre_exists(user_id, genre, db).await {
            Ok(_) => return Ok(()),
            Err((s, e, _)) => return Err((s, e))
        }
    }
    Err((StatusCode::SERVICE_UNAVAILABLE, Errors::ServerDown))
}