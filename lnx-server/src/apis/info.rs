use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use std::sync::Arc;

use crate::storage::Storage;
use super::Tag;

pub struct LnxInfoApi {
    storage: Arc<Storage>,
}

impl LnxInfoApi {
    pub fn new(storage: Arc<Storage>) -> Self {
        Self { storage }
    }
}

#[derive(poem_openapi::Object)]
pub struct ServerSummary {
    pub version: String,
    pub indexes: Vec<String>,
    pub status: String,
}

#[OpenApi(tag = Tag::InfoEndpoints)]
impl LnxInfoApi {
    #[oai(path = "/info/summary", method = "get")]
    /// Get Server Summary
    ///
    /// Returns summary information about the state of the system.
    async fn get_summary(&self) -> Json<ServerSummary> {
        let indexes = self.storage.list_indexes();
        
        Json(ServerSummary {
            version: env!("CARGO_PKG_VERSION").to_string(),
            indexes,
            status: "running".to_string(),
        })
    }
}
