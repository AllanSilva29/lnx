use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use std::collections::HashMap;
use std::sync::Arc;

use crate::storage::Storage;
use super::Tag;

pub struct LnxIndexApi {
    storage: Arc<Storage>,
}

impl LnxIndexApi {
    pub fn new(storage: Arc<Storage>) -> Self {
        Self { storage }
    }
}

#[derive(poem_openapi::Object, serde::Deserialize)]
pub struct CreateIndexRequest {
    pub name: String,
    #[serde(default)]
    pub fields: HashMap<String, String>,
}

#[derive(poem_openapi::Object)]
pub struct IndexInfo {
    pub name: String,
    pub document_count: u64,
}

#[derive(poem_openapi::Object)]
pub struct CreateIndexResponse {
    pub success: bool,
    pub name: String,
}

#[OpenApi(tag = Tag::IndexEndpoints)]
impl LnxIndexApi {
    #[oai(path = "/index", method = "post")]
    /// Create a new index
    async fn create_index(
        &self,
        Json(payload): Json<CreateIndexRequest>,
    ) -> Json<CreateIndexResponse> {
        let fields: Vec<(&str, &str)> = payload.fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        
        match self.storage.create_index(&payload.name, fields) {
            Ok(_) => Json(CreateIndexResponse {
                success: true,
                name: payload.name,
            }),
            Err(e) => Json(CreateIndexResponse {
                success: false,
                name: e.to_string(),
            }),
        }
    }

    #[oai(path = "/index", method = "get")]
    /// List all indexes
    async fn list_indexes(&self) -> Json<Vec<String>> {
        Json(self.storage.list_indexes())
    }

    #[oai(path = "/index/:name", method = "delete")]
    /// Delete an index
    async fn delete_index(
        &self,
        name: String,
    ) -> Json<serde_json::Value> {
        match self.storage.delete_index(&name) {
            Ok(_) => Json(serde_json::json!({ "success": true })),
            Err(e) => Json(serde_json::json!({ "success": false, "error": e.to_string() })),
        }
    }
}
