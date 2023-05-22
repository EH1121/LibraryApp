use serde::{Deserialize, Serialize};

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