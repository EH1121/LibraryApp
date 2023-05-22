use serde::{Deserialize, Serialize};

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
    pub search_fields: Option<String>,
    pub return_fields: Option<String>,
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