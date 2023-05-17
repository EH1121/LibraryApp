use std::iter::zip;

use elasticsearch::{IndexParts, UpdateParts, SearchParts, GetSourceParts, DeleteParts, http::response::Response, Error, BulkOperation, BulkParts}; //, BulkOperations};
use serde_json::{Value};

use super::EClientTesting;

impl EClientTesting {
    pub async fn insert_document(&self, index: &str, data: &Value) -> Result<Response, Error>{
        self.elastic
            .index(IndexParts::Index(index))
            .body(data)
            .send()
            .await
    }

    pub async fn bulk_create_documents(&self, index: &str, data: &[Value], ids: &[String]) -> Result<Response, Error> {

        let mut body: Vec<BulkOperation<_>> = vec![];

        for (val, id) in zip(data.iter(), ids){
            body.push(BulkOperation::create(id, val).index(index).into())
        }

        self.elastic
            .bulk(BulkParts::Index(index))
            .body(body)
            .send()
            .await
    }

    pub async fn search_index(&self, index: &str, body: &Value, from: &Option<i64>, count: &Option<i64>) -> Result<Response, Error>{

        let from = from.unwrap_or(0);
        let count = count.unwrap_or(20);

        self.elastic
            .search(SearchParts::Index(&[index]))
            .from(from)
            .size(count)
            .body(body)
            .send()
            .await
    }

    /// Returns a single document
    pub async fn get_document(&self, index: &str, doc_id: &str, retrieve_fields: &Option<String>) -> Result<Response, Error>{
        
        let fields_to_return = retrieve_fields.as_deref().unwrap_or("*");

        self.elastic
            .get_source(GetSourceParts::IndexId(index, doc_id))
            ._source_includes(&[fields_to_return])
            .send()
            .await
    }
    
    /// Updates existing document on an index
    pub async fn update_document(&self, index: &str, document_id: &str, data: &Value) -> Result<Response, Error> {//(StatusCode, Value){
        self.elastic
            .update(UpdateParts::IndexId(index, document_id))
            .body(data)
            .send()
            .await
    }

    /// Deletes document on an index
    pub async fn delete_document(&self, index: &str, document_id: &str) -> Result<Response, Error>{
        self.elastic
            .delete(DeleteParts::IndexId(index, document_id))
            .send()
            .await
    }
}