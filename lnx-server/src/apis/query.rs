use lnx_query::syntax::SelectQuery;
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use std::sync::Arc;

use crate::storage::Storage;
use super::Tag;

pub struct LnxQueryApi {
    storage: Arc<Storage>,
}

impl LnxQueryApi {
    pub fn new(storage: Arc<Storage>) -> Self {
        Self { storage }
    }
}

#[derive(poem_openapi::Object)]
pub struct SearchResult {
    pub results: Vec<serde_json::Value>,
    pub total: usize,
    pub took_ms: u64,
}

#[derive(poem_openapi::Object, serde::Deserialize)]
pub struct SimpleSearchRequest {
    pub index: String,
    pub query: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default)]
    pub semantic: bool,
}

fn default_limit() -> usize { 10 }

#[OpenApi(tag = Tag::QueryEndpoints)]
impl LnxQueryApi {
    #[oai(path = "/query/select", method = "post")]
    /// Execute Select Query
    async fn execute(
        &self,
        Json(payload): Json<SelectQuery>,
    ) -> poem::Result<Json<SearchResult>> {
        let start = std::time::Instant::now();
        
        let index_name = payload.from_tables.clone();
        let limit = payload.limit;
        
        let results = self.storage.search(&index_name, "*", limit).unwrap_or_default();
        
        let took = start.elapsed().as_millis() as u64;
        
        Ok(Json(SearchResult {
            results: results.clone(),
            total: results.len(),
            took_ms: took,
        }))
    }

    #[oai(path = "/query/simple", method = "post")]
    /// Simple search - just pass query string directly
    async fn simple_search(
        &self,
        Json(payload): Json<SimpleSearchRequest>,
    ) -> poem::Result<Json<SearchResult>> {
        let start = std::time::Instant::now();
        
        // If semantic search is enabled, combine keyword + semantic results
        let results = if payload.semantic {
            // Get keyword results
            let keyword_results = match self.storage.search(&payload.index, &payload.query, payload.limit) {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!("Keyword search error: {}", e);
                    vec![]
                }
            };
            
            // Get semantic results (would need embedding service)
            // For now, just return keyword results with a note
            tracing::info!("Semantic search requested but not yet fully implemented");
            keyword_results
        } else {
            match self.storage.search(&payload.index, &payload.query, payload.limit) {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!("Search error: {}", e);
                    return Err(poem::Error::from_string(
                        format!("Search failed: {}", e),
                        poem::http::StatusCode::INTERNAL_SERVER_ERROR
                    ));
                }
            }
        };
        
        let took = start.elapsed().as_millis() as u64;
        
        Ok(Json(SearchResult {
            results: results.clone(),
            total: results.len(),
            took_ms: took,
        }))
    }

    #[oai(path = "/query/explain", method = "post")]
    /// Explain Query
    async fn explain(
        &self,
        Json(payload): Json<SelectQuery>,
    ) -> Json<serde_json::Value> {
        Json(serde_json::json!({
            "index": payload.from_tables,
            "limit": payload.limit,
            "offset": payload.offset,
        }))
    }
}
