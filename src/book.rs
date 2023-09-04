use std::io::Read;

use crate::{database::Database, structs::*, libs::*};
use actix_multipart::form::MultipartForm;
use actix_web::{web::{self, Data}, HttpResponse, http::StatusCode};
use serde_json::{json, Value};

/// Ambil data buku dari indeks
pub async fn get_book(path: web::Path<UserBookID>, query: web::Query<OptionalReturnFields>, db: Data::<Database>) -> HttpResponse {  

    // Jadikan Lowercase lalu diformat jadi bentuk userid.genre 
    let genre = path.genre.to_lowercase();
    let genre_index = &format!("{}.{}", &path.user_id.to_lowercase(), &genre);

    // Cek kalo userid sama genre emang ada, kalo iya lanjut
    match check_userid_genre(&path.user_id, &genre, &db).await{
        Ok(_) => (),
        Err((s, e)) => return HttpResponse::build(s).json(json!({"error": e.to_string()}))
    };

    // Sekarang coba ambil dokumennya, kalo gagal kirim eror, kalo berhasil kirim bukunya
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

/// Cari buku di indeks dengan metode post
pub async fn search_books(path: web::Path<UserID>, genre: web::Query<OptionalGenre>, query: web::Json<BookSearchQuery>, db: Data::<Database>) -> HttpResponse {

    // Berapa lama waktu jalannya?
    let took = std::time::Instant::now();

    // Antara cari di semua, atau di satu genre spesifik
    let mut to_search = genre.genre.as_ref().unwrap_or(&"*".to_string()).to_lowercase();
    to_search = if to_search.eq(""){
        "*".to_string()
    } else {
        to_search
    };
    let genre_index = format!("{}.{}", &path.user_id.to_lowercase(), &to_search);

    // Cek kalo user atau genre ada, lalu kalo ketemu, genre engga ada, tetap lanjut tapi ambil dari semua genre
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

    // Hapus wildcard kalo ada lalu hapus spasi kalo berlebihan
    let terms = if query.search_term.is_some() {
        let mut z = query.search_term.as_ref().unwrap().replace("*", " ").replace(r" *", "* ");
        z.push('*');
        Some(z)
    } else {
        None
    };

    // Buat bodynya untuk search
    let body = if let Some(term) = terms {
        // Kalo ada yang mau dicari
        json!({
            "_source": {
                "includes": "*"
            },
            "query": {
                "query_string": {
                    "query": term,
                    "type": "cross_fields"
                }
            }
        })
    } else {
        // Kalo engga ada
        json!({
            "_source": {
                "includes": "*"
            },
            "query": {
                "match_all": {} 
            },
        })
    };

    // Kirim permintaan cari
    let response = db.search(&genre_index, 
            body, 
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

/// Cari buku di indeks dengan metode get
pub async fn search_books_get(path: web::Path<UserID>, query: web::Query<BookSearchQuery>, db: Data::<Database>) -> HttpResponse {
    // Berapa lama waktu jalannya?
    let took = std::time::Instant::now();

    // Antara cari di semua, atau di satu genre spesifik
    let mut to_search = query.genre.as_ref().unwrap_or(&"*".to_string()).to_lowercase();
    to_search = if to_search.eq(""){
        "*".to_string()
    } else {
        to_search
    };
    let genre_index = format!("{}.{}", &path.user_id.to_lowercase(), &to_search);

    // Cek kalo user atau genre ada, lalu kalo ketemu, genre engga ada, tetap lanjut tapi ambil dari semua genre
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

    // Hapus wildcard kalo ada lalu hapus spasi kalo berlebihan
    let terms = if query.search_term.is_some() {
        let mut z = query.search_term.as_ref().unwrap().replace("*", " ").replace(r" *", "* ");
        z.push('*');
        Some(z)
    } else {
        None
    };

    // Buat bodynya untuk search
    let body = if let Some(term) = terms {
        // Kalo ada yang mau dicari
        json!({
            "_source": {
                "includes": "*"
            },
            "query": {
                "query_string": {
                    "query": term,
                    "type": "cross_fields"
                }
            }
        })
    } else {
        // Kalo engga ada
        json!({
            "_source": {
                "includes": "*"
            },
            "query": {
                "match_all": {} 
            },
        })
    };

    // Kirim permintaan cari
    let response = db.search(&genre_index, body, query.from, query.count)
        .await.unwrap()
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

// Buat buku baru
pub async fn create_books(path: web::Path<UserGenre>, data: web::Json<Vec<BookInput>>, db: Data::<Database>) -> HttpResponse {

    // Bikin lowercase lalu cek kalo user sama genre ada
    let genre = path.genre.to_lowercase();

    match check_userid_genre(&path.user_id, &genre, &db).await{
        Ok(_) => (),
        Err((s, e)) => return HttpResponse::build(s).json(json!({"error": e.to_string()})),
    }

    // Kirim permintaan bikin
    let response = db.index_documents(
        &format!("{}.{}", &path.user_id.to_lowercase(), &genre), 
        data.as_ref())
        .await.unwrap()
        .json::<Value>()
        .await.unwrap();

    // Untuk respons cuma yang gagal yang dikirim
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

/// Update buku
pub async fn update_book(path: web::Path<UserBookID>, data: web::Json<BookInput>, db: Data::<Database>) -> HttpResponse {

    // Cek kalo user sama genre ada
    let genre = path.genre.to_lowercase();
    match check_userid_genre(&path.user_id, &genre, &db).await{
        Ok(_) => (),
        Err((s, e)) => return HttpResponse::build(s).json(json!({"error": e.to_string()}))
    };

    // Kirim permintaan update
    match db.update_single_document(&format!("{}.{}", &path.user_id.to_lowercase(), &genre), &path.book_id, &data).await.unwrap().status_code(){
        // Kalo ga ketemu
        StatusCode::NOT_FOUND => HttpResponse::NotFound().json(json!({"error": Errors::BookNotFound(path.book_id.to_string()).to_string()})),
        
        // Kalo permintaan gagal karena request konflik atau apa
        StatusCode::BAD_REQUEST => HttpResponse::BadRequest().json(json!({"error": Errors::BadRequest.to_string()})),
        
        // lain-lain
        x => {
            if x.is_success() {
                // kalo sukses
                HttpResponse::build(x).finish()
            } else {
                // kalo bukan sukses (engga tau eror apa)
                HttpResponse::build(x).json(json!({"error": Errors::Unknown.to_string()}))
            }
        }
    }

}

/// Hapus buku
pub async fn delete_book(path: web::Path<UserBookID>, db: Data::<Database>) -> HttpResponse { 
    // Cek kalo user sama genre ada
    let genre = path.genre.to_lowercase();

    match check_userid_genre(&path.user_id, &genre, &db).await{
        Ok(_) => (),
        Err((s, e)) => return HttpResponse::build(s).json(json!({"error": e.to_string()}))
    };

    // Hapus satu buku
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

/// Untuk upload file json supaya
pub async fn upload_json(path: web::Path<UserGenre>, f: MultipartForm<GetFile>, db: web::Data<Database>) -> HttpResponse {
    // Cek kalo user sama genre ada
    let genre = &path.genre.to_lowercase();
    match check_userid_genre(&path.user_id, genre, &db).await{
        Ok(_) => (),
        Err((s, e)) => return HttpResponse::build(s).json(json!({"error": e.to_string()}))
    };

    // Namanya di split supaya dari nama.json jadi [nama, json]
    let name: Vec<&str> = f.file.file_name.as_ref().unwrap().split('.').collect();
    let mut file = f.file.file.reopen().unwrap();

    // Cek kalo namanya json
    if !name.last().unwrap().to_ascii_lowercase().eq("json"){
        return HttpResponse::BadRequest().json(json!({"error": "Only JSON is Accepted"}))
    };

    // Buffer kontennya
    let mut contents = String::new();
    
    // Baca semua data di jsonnya
    file.read_to_string(&mut contents).unwrap();
    
    // Dari variabel contents jadiin bentuk serde value
    let data: Result<Vec<Value>, _> = serde_json::from_str(&contents);
    match data{
        // Kalau berhasil
        Ok(dat) => {
            // Kirim ke elastic
            let response = db.index_documents(
                &format!("{}.{}", &path.user_id.to_lowercase(), &path.genre.to_lowercase()), 
                dat.as_ref())
                .await.unwrap()
                .json::<Value>()
                .await.unwrap();
            
            // Untuk respons cuma yang gagal yang dikirim
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
                // Kalo erornya gatau
                return HttpResponse::Ok().json(json!({"error": Errors::Unknown.to_string()}));
            }
            HttpResponse::Ok().json(fail)
        },
        // Kalo jsonnya invalid
    Err(_) => HttpResponse::BadRequest().json(json!({"error": "Invalid JSON"}))
    }
}