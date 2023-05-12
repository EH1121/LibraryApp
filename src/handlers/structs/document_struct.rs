use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Used for GET/POST: Search
#[derive(Deserialize)]
pub struct DocumentSearchQuery {
    pub search_term: Option<String>,
    pub search_in: Option<String>,
    pub return_fields: Option<String>,
    pub from: Option<i64>,
    pub count: Option<i64>,
    pub wildcards: Option<bool>
}

/// Used for Get: Document
#[derive(Deserialize)]
pub struct ReturnFields{
    pub return_fields: Option<String>
}

/// For Bulk Errors Output
#[derive(Serialize)]
pub struct BulkFailures {
    pub document_number: usize,
    pub error: String,
    pub status: i64
}

/// Used for Delete: Document
#[derive(Deserialize)]
pub struct RequiredDocumentID {
    pub app_id: String,
    pub index: String,
    pub document_id: String
}

#[derive(Serialize, Deserialize)]
pub struct BookInput {
    pub isbn: Option<String>,
    pub judul: Option<String>,
    pub penulis: Option<String>,
    pub penerbit: Option<String>,
    pub genre: Option<Vec<String>>,
    pub bahasa: Option<String>,
    pub jumlah_halaman: Option<usize>,
    pub tanggal_terbit: NaiveDate
}