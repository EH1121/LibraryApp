use serde::{Deserialize, Serialize};

/// Used for GET/POST: Search
#[derive(Deserialize)]
pub struct DocumentSearchQuery {
    pub index: Option<String>,
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

/// Used for POST/PUT: Document
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