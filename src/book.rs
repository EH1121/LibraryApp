use crate::{database::Database, structs::*, libs::*};
use actix_web::{web::{self, Data}, HttpResponse, http::StatusCode};
use serde_json::{json, Value};

pub async fn get_book(path: web::Path<UserBookID>, query: web::Path<OptionalReturnFields>, db: Data::<Database>) -> HttpResponse {  

    let genre = path.genre.to_lowercase();
    let genre_index = &format!("{}.{}", &path.user_id.to_lowercase(), &genre);

    match check_userid_genre(&path.user_id, &genre, &db).await{
        Ok(_) => (),
        Err((s, e)) => return HttpResponse::build(s).json(json!({"error": e.to_string()}))
    };

    let response = db.get_single_document(genre_index, &path.book_id, query.return_fields.clone()).await.unwrap();
    
    if !response.status_code().is_success() {
        let e = match response.status_code(){
            StatusCode::NOT_FOUND => Errors::BookNotFound(path.book_id.to_owned()).to_string(),
            _ => Errors::Unknown.to_string()
        };
        return HttpResponse::build(response.status_code()).json(json!({"error": e}));
    }

    HttpResponse::build(response.status_code()).json(response.json::<Value>().await.unwrap())
}

pub async fn search_books(path: web::Path<UserID>, genre: web::Query<OptionalGenre>, query: web::Json<BookSearchQuery>, db: Data::<Database>) -> HttpResponse {

    let took = std::time::Instant::now();

    let mut to_search = genre.genre.as_ref().unwrap_or(&"*".to_string()).to_lowercase();
    to_search = if to_search.eq(""){
        "*".to_string()
    } else {
        to_search
    };
    let genre_index = format!("{}.{}", &path.user_id.to_lowercase(), &to_search);

    match check_userid_genre(&path.user_id, &to_search, &db).await{
        Ok(_) => (),
        Err((s, e)) => 
            match e {
                Errors::GenreNotFound(_) => 
                    if query.genre.is_some() && !(query.genre.as_ref().unwrap().eq("*") || query.genre.as_ref().unwrap().eq("")){
                        return HttpResponse::build(s).json(json!({"error": e.to_string()}))
                    },
                _ => return HttpResponse::build(s).json(json!({"error": e.to_string()}))
            }
    };
    
    let fields = query.search_fields.as_ref().map(|s| s.split(',').map(|x| x.trim().to_owned()).collect());

    let term = if query.search_term.is_some() {
        let mut z = query.search_term.as_ref().unwrap().replace(r" *", "* ");
        z.push('*');
        Some(z)
    } else {
        None
    };

    let response = db.search(&genre_index, 
        &search_body(&term, &fields, &query.return_fields), 
            query.from, 
            query.count
        ).await.unwrap()
        .json::<Value>()
        .await.unwrap();

    HttpResponse::Ok().json(json!({
        "took": &took.elapsed().as_millis(),
        "data": &response["hits"]["hits"],
        "total": &response["hits"]["total"]["value"],
        "from": &query.from.unwrap_or(0),
        "count": &query.count.unwrap_or(20)
    }))
}

pub async fn search_books_get(path: web::Path<UserID>, query: web::Query<BookSearchQuery>, db: Data::<Database>) -> HttpResponse {

    let took = std::time::Instant::now();

    let mut to_search = query.genre.as_ref().unwrap_or(&"*".to_string()).to_lowercase();
    to_search = if to_search.eq(""){
        "*".to_string()
    } else {
        to_search
    };
    let genre_index = format!("{}.{}", &path.user_id.to_lowercase(), &to_search);

    match check_userid_genre(&path.user_id, &to_search, &db).await{
        Ok(_) => (),
        Err((s, e)) => 
            match e {
                Errors::GenreNotFound(_) => 
                    if query.genre.is_some() && !(query.genre.as_ref().unwrap().eq("*") || query.genre.as_ref().unwrap().eq("")){
                        return HttpResponse::build(s).json(json!({"error": e.to_string()}))
                    },
                _ => return HttpResponse::build(s).json(json!({"error": e.to_string()}))
            }
    };

    let fields = query.search_fields.as_ref().map(|s| s.split(',').map(|x| x.trim().to_owned()).collect());

    let term = if query.search_term.is_some() {
        let mut z = query.search_term.as_ref().unwrap().replace(r" *", "* ");
        z.push('*');
        Some(z)
    } else {
        None
    };

    let response = db.search(&genre_index, 
        &search_body(&term, &fields, &query.return_fields), 
            query.from, 
            query.count
        ).await.unwrap()
        .json::<Value>()
        .await.unwrap();

    HttpResponse::Ok().json(json!({
        "took": &took.elapsed().as_millis(),
        "data": &response["hits"]["hits"],
        "total": &response["hits"]["total"]["value"],
        "from": &query.from.unwrap_or(0),
        "count": &query.count.unwrap_or(20)
    }))
}

pub async fn create_books(path: web::Path<UserGenre>, data: web::Json<Vec<BookInput>>, db: Data::<Database>) -> HttpResponse {

    let genre = path.genre.to_lowercase();

    match check_userid_genre(&path.user_id, &genre, &db).await{
        Ok(_) => (),
        Err((s, e)) => return HttpResponse::build(s).json(json!({"error": e.to_string()})),
    }

    let response = db.index_documents(
        &format!("{}.{}", &path.user_id.to_lowercase(), &genre), 
        data.as_ref())
        .await.unwrap()
        .json::<Value>()
        .await.unwrap();

    let mut fail: Vec<Failures> = vec![];

    if !response["errors"].is_null() {
        if response["errors"].as_bool().unwrap(){
            for (num, dat) in response["items"].as_array().unwrap().iter().enumerate(){
                if !dat["index"]["error"].is_null(){
                    fail.push(
                        Failures {
                            doc_num: num,
                            reason: dat["index"]["error"]["reason"].as_str().unwrap().to_string(),
                            code: dat["index"]["status"].as_i64().unwrap()
                        }
                    );
                }
            }
        }
    } else {
        return HttpResponse::Ok().json(json!({"error": Errors::Unknown.to_string()}));
    }
    HttpResponse::Ok().json(fail)
}

pub async fn update_book(path: web::Path<UserBookID>, data: web::Json<BookInput>, db: Data::<Database>) -> HttpResponse {

    let genre = path.genre.to_lowercase();
    match check_userid_genre(&path.user_id, &genre, &db).await{
        Ok(_) => (),
        Err((s, e)) => return HttpResponse::build(s).json(json!({"error": e.to_string()}))
    };
    match db.update_single_document(&format!("{}.{}", &path.user_id.to_lowercase(), &genre), &path.book_id, &data).await.unwrap().status_code(){
        StatusCode::NOT_FOUND => HttpResponse::NotFound().json(json!({"error": Errors::BookNotFound(path.book_id.to_string()).to_string()})),
        StatusCode::BAD_REQUEST => HttpResponse::BadRequest().json(json!({"error": Errors::BadRequest.to_string()})),
        x => {
            if x.is_success() {
                HttpResponse::build(x).finish()
            } else {
                HttpResponse::build(x).json(json!({"error": Errors::Unknown.to_string()}))
            }
        }
    }

}

pub async fn delete_book(path: web::Path<UserBookID>, db: Data::<Database>) -> HttpResponse { 
    let genre = path.genre.to_lowercase();

    match check_userid_genre(&path.user_id, &genre, &db).await{
        Ok(_) => (),
        Err((s, e)) => return HttpResponse::build(s).json(json!({"error": e.to_string()}))
    };

    match db.delete_single_document(&format!("{}.{}", &path.user_id.to_lowercase(), &genre), &path.book_id).await.unwrap().status_code() {
        StatusCode::NOT_FOUND => HttpResponse::NotFound().json(json!({"error": Errors::BookNotFound(path.book_id.to_string()).to_string()})),
        x => 
            if x.is_success() {
                HttpResponse::build(x).finish()
            } else {
                HttpResponse::build(x).json(json!({"error": Errors::Unknown.to_string()}))
            }
    }
}