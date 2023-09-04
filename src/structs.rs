use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserName{
    pub user_name: String
}

#[derive(Deserialize)]
pub struct GetUserList{
    pub user_name: Option<String>,
    pub from: Option<i64>,
    pub count: Option<i64>
}

#[derive(Deserialize)]
pub struct UserID{
    pub user_id: String
}

#[derive(Deserialize)]
pub struct UpdateUser{
    pub user_id: String,
    pub user_name: String
}

#[derive(MultipartForm)]
pub struct GetFile{
    pub file: TempFile
}

#[derive(Deserialize)]
pub struct OptionalGenre{
    pub genre: Option<String>
}

#[derive(Deserialize)]
pub struct UserGenre{
    pub user_id: String,
    pub genre: String
}

#[derive(Deserialize)]
pub struct Genre{
    pub genre: String
}

#[derive(Deserialize, Serialize)]
pub struct IndexResponse {
    pub index: String,
    #[serde(rename(deserialize = "docs.count"))]
    pub books_count: String,
    #[serde(rename(deserialize = "docs.deleted"))]
    pub books_deleted: String,
    #[serde(rename(deserialize = "pri.store.size"))]
    pub primary_size: String
}

#[derive(Deserialize)]
pub struct OptionalReturnFields{
    pub return_fields: Option<String>
}

#[derive(Serialize)]
pub struct Failures {
    pub doc_num: usize,
    pub reason: String,
    pub code: i64
}

#[derive(Deserialize)]
pub struct UserBookID {
    pub user_id: String,
    pub genre: String,
    pub book_id: String
}

#[derive(Deserialize)]
pub struct BookSearchQuery {
    pub genre: Option<String>,
    pub search_term: Option<String>,
    pub from: Option<i64>,
    pub count: Option<i64>
}

#[derive(Serialize, Deserialize)]
pub struct BookInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isbn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub judul: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penulis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penerbit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bahasa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jumlah_halaman: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tanggal_terbit: Option<String>
}