use elasticsearch::{
    http::{transport::Transport, response::Response}, 
    indices::{IndicesCreateParts, IndicesDeleteParts}, 
    cat::CatIndicesParts,
    *
};
use serde::Serialize;
use serde_json::json;

pub struct Database {
    pub es: Elasticsearch
}

impl Database {
    pub fn new(url: &str) -> Self {
        Self{
            es: Elasticsearch::new(Transport::single_node(url).unwrap())
        }
    }

    /// Buat Dokumen Baru secara banyak
    pub async fn index_documents(&self, index: &str, data: &[impl Serialize]) -> Result<Response, Error> {
        
        // Konversi ke bentuk yang diminta elastic
        let body: Vec<BulkOperation<_>> = data
            .iter()
            .map(|p| {
                BulkOperation::index(p).into()
            })
            .collect();

        // Kirim ke elastic
        self.es
            .bulk(BulkParts::Index(index))
            .body(body)
            .send()
            .await
    }

    /// Cari dokumen di indeks
    pub async fn search(&self, index: &str, body: impl Serialize, from: Option<i64>, count: Option<i64>) -> Result<Response, Error>{

        // Cari di indeks dengan paginasi dan data pencarian
        self.es
            .search(SearchParts::Index(&[index]))
            .from(from.unwrap_or(0))
            .size(count.unwrap_or(20))
            .body(body)
            .send()
            .await
    }

    /// Ambil satu dokumen dari indeks
    pub async fn get_single_document(&self, index: &str, doc_id: &str, retrieve_fields: Option<String>) -> Result<Response, Error>{
        
        // Apa aja yang mau diambil dari dokumennya
        let fields_to_return = retrieve_fields.unwrap_or("*".to_string());

        // Minta ke elastic untuk dokumennya
        self.es
            .get_source(GetSourceParts::IndexId(index, doc_id))
            ._source_includes(&[&fields_to_return])
            .send()
            .await
    }

    // Update satu dokumen
    pub async fn update_single_document(&self, index: &str, document_id: &str, data: impl Serialize) -> Result<Response, Error> {
        self.es
            .update(UpdateParts::IndexId(index, document_id))
            .body(json!({"doc": data}))
            .send()
            .await
    }

    // Hapus satu dokumen
    pub async fn delete_single_document(&self, index: &str, document_id: &str) -> Result<Response, Error>{
        self.es
            .delete(DeleteParts::IndexId(index, document_id))
            .send()
            .await
    }

    // Buat satu indeks baru
    pub async fn create_single_index(&self, index: &str, body: &impl Serialize) -> Result<Response, Error>{
        self.es
            .indices()
            .create(IndicesCreateParts::Index(index))
            .body(body)
            .send()
            .await
    }

    // Ambil data statistik satu atau lebih indeks
    pub async fn get_indices(&self, index: Option<String>) -> Result<Response, Error>{
        self.es
            .cat()
            .indices(CatIndicesParts::Index(&[&index.unwrap_or("*".to_string())]))
            .format("json")
            .send()
            .await
    }
    
    // Hapus satu indeks
    pub async fn delete_single_index(&self, index: String) -> Result<Response, Error>{
        self.es
            .indices()
            .delete(IndicesDeleteParts::Index(&[&index]))
            .send()
            .await
    }
}