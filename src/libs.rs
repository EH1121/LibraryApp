use std::collections::HashSet;
use actix_web::{http::StatusCode};
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

pub async fn get_book(genre: &str, book_id: &str, retrieve_fields: Option<String>, db: &Database) -> Result<(StatusCode, Value), (StatusCode, Errors)>{
    let response = db.get_single_document(genre, book_id, retrieve_fields).await.unwrap();
    
    if !response.status_code().is_success() {
        let e = match response.status_code(){
            StatusCode::NOT_FOUND => Errors::BookNotFound(book_id.to_string()),
            _ => Errors::Unknown
        };
        return Err((response.status_code(), e));
    }

    Ok((response.status_code(), response.json::<Value>().await.unwrap()))
}

pub fn search_body(terms: &Option<String>, search_fields: &Option<Vec<String>>, get_fields: &Option<String>) -> Value {

    let fields = match get_fields {
        Some(val) => val.split(',').map(|x| x.trim().to_string()).collect(),
        None => vec!["*".to_string()],
    };

    if let Some(term) = terms {
        json!({
            "_source": {
                "includes": fields
            },
            "query": {
                "query_string": {
                    "query": term,
                    "type": "cross_fields",
                    "fields": search_fields.to_owned().unwrap_or(vec!["*".to_string()])
                }
            }
        })
    } else {
        json!({
            "_source": {
                "includes": fields
            },
            "query": {
                "match_all": {} 
            },
        })
    }
}

pub async fn create_new_genre(user_id: Option<String>, genre: &str, db: &Database) {
    let genre_index = match user_id {
        Some(x) => format!("{}.{}", x.to_lowercase(), &genre.to_lowercase()),
        None => genre.to_string()
    };

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

pub async fn genre_exists(user_id: &str, genre: &str, db: &Database) -> Result<HashSet<String>, (StatusCode, Errors, HashSet<String>)> {
    match get_user_genre_list(user_id, db).await {
        Ok(l) => {
            match l.contains(genre) {
                true => Ok(l),
                false => Err((StatusCode::NOT_FOUND, Errors::GenreNotFound(genre.to_string()), l))
            }
        },
        Err((s, e)) => Err((s, e, HashSet::new()))
    }
}

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

pub async fn check_server(db: &Database) -> bool {
    db.get_indices(Some("".to_string())).await.is_ok()
}

pub async fn check_userid_genre(user_id: &str, genre: &str, db: &Database) -> Result<(), (StatusCode, Errors)>{
    if check_server(db).await {
        match genre_exists(user_id, genre, db).await {
            Ok(_) => return Ok(()),
            Err((s, e, _)) => return Err((s, e))
        }
    }
    Err((StatusCode::SERVICE_UNAVAILABLE, Errors::ServerDown))
}