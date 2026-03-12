pub mod index;
pub mod search;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use parking_lot::RwLock;
use tantivy::TantivyDocument;

use crate::storage::index::IndexInstance;
use crate::storage::search::search_index;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Index '{0}' not found")]
    IndexNotFound(String),
    #[error("Invalid document: {0}")]
    InvalidDocument(String),
}

pub struct Storage {
    indexes: RwLock<HashMap<String, Arc<IndexInstance>>>,
    data_path: PathBuf,
}

impl Storage {
    pub fn new(data_path: PathBuf) -> Result<Self> {
        let storage = Self {
            indexes: RwLock::new(HashMap::new()),
            data_path,
        };
        
        // Load existing indexes from disk
        storage.load_existing_indexes()?;
        
        Ok(storage)
    }
    
    fn load_existing_indexes(&self) -> Result<()> {
        if !self.data_path.exists() {
            std::fs::create_dir_all(&self.data_path)?;
            return Ok(());
        }
        
        let entries = std::fs::read_dir(&self.data_path)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let index_name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                // Try to load the index
                match IndexInstance::load(&path) {
                    Ok(instance) => {
                        tracing::info!("Loaded existing index: {}", index_name);
                        self.indexes.write().insert(index_name, Arc::new(instance));
                    }
                    Err(e) => {
                        tracing::warn!("Failed to load index {}: {}", index_name, e);
                    }
                }
            }
        }
        
        Ok(())
    }

    pub fn create_index(&self, name: &str, schema_fields: Vec<(&str, &str)>) -> Result<Arc<IndexInstance>> {
        let index_path = self.data_path.join(name);
        tracing::info!("Creating index '{}' at path: {:?}", name, index_path);
        let instance = IndexInstance::create(schema_fields, &index_path)?;
        let instance = Arc::new(instance);

        self.indexes.write().insert(name.to_string(), instance.clone());
        tracing::info!("Index '{}' created and stored in memory", name);

        Ok(instance)
    }

    pub fn list_indexes(&self) -> Vec<String> {
        self.indexes.read().keys().cloned().collect()
    }

    pub fn delete_index(&self, name: &str) -> Result<()> {
        tracing::info!("Deleting index: {}", name);
        
        // Extract the instance from memory first, releasing the lock immediately
        let instance = {
            let mut indexes = self.indexes.write();
            indexes.remove(name)
        };
        
        let was_found = instance.is_some();
        
        // If we had an instance, clean it up outside the lock
        if let Some(instance) = instance {
            tracing::info!("Cleaning up resources for index: {}", name);
            
            // Force commit and drop writer
            {
                let mut writer_guard = instance.writer.write();
                if let Some(mut writer) = writer_guard.take() {
                    if let Err(e) = writer.commit() {
                        tracing::warn!("Failed to commit changes for index {}: {}", name, e);
                    }
                    tracing::info!("Committed and dropped writer for index: {}", name);
                }
            }
            
            // The reader and index will be automatically dropped when the instance is dropped
            // No need for explicit manipulation since they're part of the Arc
            
            // Explicitly drop the instance to force resource cleanup
            drop(instance);
            
            // Force garbage collection to help release resources
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
        
        // Now try to delete the directory
        let index_path = self.data_path.join(name);
        if index_path.exists() {
            tracing::info!("Attempting to delete index directory: {:?}", index_path);
            
            for attempt in 1..=5 {
                match std::fs::remove_dir_all(&index_path) {
                    Ok(_) => {
                        tracing::info!("Successfully deleted index directory: {:?}", index_path);
                        return Ok(());
                    }
                    Err(e) if attempt == 5 => {
                        tracing::error!("Failed to delete index after 5 attempts: {}", e);
                        
                        // Aggressive cleanup: try to force close all file handles
                        #[cfg(unix)]
                        {
                            use std::os::unix::fs::MetadataExt;
                            if let Ok(metadata) = std::fs::metadata(&index_path) {
                                tracing::debug!("Directory inode: {}, mode: {:?}", metadata.ino(), metadata.mode());
                            }
                        }
                        
                        // Delete files individually with more aggressive approach
                        if let Ok(entries) = std::fs::read_dir(&index_path) {
                            for entry in entries.flatten() {
                                let path = entry.path();
                                if path.is_file() {
                                    // Try multiple times to delete each file
                                    for file_attempt in 1..=3 {
                                        match std::fs::remove_file(&path) {
                                            Ok(_) => {
                                                tracing::debug!("Removed file: {:?}", path);
                                                break;
                                            }
                                            Err(e) if file_attempt == 3 => {
                                                tracing::warn!("Failed to remove file after 3 attempts: {:?} - {}", path, e);
                                            }
                                            Err(_) => {
                                                std::thread::sleep(std::time::Duration::from_millis(50));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Wait longer before final attempt
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                        
                        // Final attempt with force on Unix systems
                        #[cfg(unix)]
                        {
                            use std::process::Command;
                            let path_str = index_path.to_string_lossy();
                            if let Ok(_) = Command::new("rm").arg("-rf").arg(&*path_str).status() {
                                tracing::info!("Force deleted index directory with rm -rf: {:?}", index_path);
                                return Ok(());
                            }
                        }
                        
                        // Last resort Windows-style
                        #[cfg(not(unix))]
                        {
                            std::thread::sleep(std::time::Duration::from_millis(2000));
                        }
                        
                        if std::fs::remove_dir_all(&index_path).is_ok() {
                            tracing::info!("Force deleted index directory: {:?}", index_path);
                            return Ok(());
                        }
                        
                        return Err(anyhow::anyhow!("Failed to delete index directory after all attempts: {}", e));
                    }
                    Err(e) => {
                        tracing::warn!("Attempt {} failed to delete index: {}", attempt, e);
                        // Progressive backoff: 100ms, 200ms, 400ms, 800ms, 1600ms
                        let delay = 100 * (1 << (attempt - 1));
                        std::thread::sleep(std::time::Duration::from_millis(delay));
                    }
                }
            }
        } else if !was_found {
            tracing::warn!("Index {} not found for deletion", name);
        }
        
        tracing::info!("Index {} deletion completed", name);
        Ok(())
    }

    pub fn add_document(
        &self,
        index_name: &str,
        id: &str,
        filename: Option<&str>,
        fields: HashMap<String, serde_json::Value>,
    ) -> Result<()> {
        let indexes = self.indexes.read();
        let instance = indexes
            .get(index_name)
            .ok_or_else(|| StorageError::IndexNotFound(index_name.to_string()))?;

        tracing::info!(
            "Adding doc to index {}: id={}, filename={:?}, index_fields={:?}, doc_fields={:?}",
            index_name,
            id,
            filename,
            instance.fields.keys().collect::<Vec<_>>(),
            fields.keys().collect::<Vec<_>>()
        );

        let id_field = instance
            .schema
            .get_field("id")
            .map_err(|_| StorageError::InvalidDocument("id field not found".to_string()))?;
        let filename_field = instance.schema.get_field("filename").ok();
        let page_field = instance.schema.get_field("page").ok();
        let section_field = instance.schema.get_field("section").ok();

        let mut doc = TantivyDocument::new();
        doc.add_text(id_field, id);

        if let (Some(fn_field), Some(fn_val)) = (filename_field, filename) {
            doc.add_text(fn_field, fn_val);
        }

        let mut page_num: Option<i64> = None;
        let mut section_num: Option<i64> = None;

        for (key, value) in fields {
            if key == "page" {
                if let Some(p) = value.as_i64() {
                    page_num = Some(p);
                }
                continue;
            }

            if key == "section" {
                if let Some(s) = value.as_i64() {
                    section_num = Some(s);
                }
                continue;
            }

            if let Some(field) = instance.fields.get(&key) {
                match value {
                    serde_json::Value::String(s) => {
                        // Only store if it looks like valid text
                        if is_valid_text(&s) {
                            doc.add_text(*field, s);
                        }
                    }
                    serde_json::Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            doc.add_i64(*field, i);
                        } else if let Some(f) = n.as_f64() {
                            doc.add_f64(*field, f);
                        }
                    }
                    serde_json::Value::Bool(b) => {
                        doc.add_i64(*field, if b { 1 } else { 0 });
                    }
                    _ => {}
                }
            }
        }

        if let (Some(pf), Some(pn)) = (page_field, page_num) {
            doc.add_i64(pf, pn);
        }

        if let (Some(sf), Some(sn)) = (section_field, section_num) {
            doc.add_i64(sf, sn);
        }

        let mut writer_guard = instance.writer.write();
        if let Some(writer) = writer_guard.as_mut() {
            writer.add_document(doc)?;
            writer.commit()?;
            tracing::info!("Document committed to index: {}", index_name);
        }

        Ok(())
    }

    pub fn search(
        &self,
        index_name: &str,
        query_str: &str,
        limit: usize,
    ) -> Result<Vec<serde_json::Value>> {
        let indexes = self.indexes.read();
        let instance = indexes
            .get(index_name)
            .ok_or_else(|| StorageError::IndexNotFound(index_name.to_string()))?;

        tracing::info!(
            "Searching index '{}' for query: '{}'",
            index_name,
            query_str
        );

        let search_results = search_index(
            &instance.index,
            &instance.reader,
            &instance.fields,
            query_str,
            limit,
        )?;

        Ok(search_results)
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new(PathBuf::from("/var/lib/lnx/data")).unwrap()
    }
}

fn is_valid_text(s: &str) -> bool {
    // Check if string contains too many control characters (likely binary)
    if s.is_empty() {
        return false;
    }
    
    if !s.is_ascii() {
        // For non-ASCII, check if it's valid UTF-8
        if std::str::from_utf8(s.as_bytes()).is_err() {
            return false;
        }
    }
    
    // Count non-printable characters (excluding common whitespace)
    let non_printable = s.bytes().filter(|&b| b < 32 && b != 9 && b != 10 && b != 13).count();
    
    // Reject if more than 5% non-printable
    if s.len() > 10 && non_printable * 20 > s.len() {
        return false;
    }
    
    true
}
