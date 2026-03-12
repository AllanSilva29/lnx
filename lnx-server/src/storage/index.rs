use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use parking_lot::RwLock;
use tantivy::schema::*;
use tantivy::{Index, IndexReader, IndexWriter, directory::ManagedDirectory, directory::MmapDirectory, IndexSettings, store::Compressor};

pub struct IndexInstance {
    pub index: Index,
    pub schema: Schema,
    pub writer: RwLock<Option<IndexWriter>>,
    pub reader: IndexReader,
    pub fields: HashMap<String, Field>,
}

impl IndexInstance {
    pub fn create(
        schema_fields: Vec<(&str, &str)>,
        index_path: &Path,
    ) -> Result<Self> {
        let mut schema_builder = Schema::builder();

        schema_builder.add_text_field("id", STRING | STORED);
        schema_builder.add_text_field("filename", STRING | STORED);
        schema_builder.add_i64_field("page", INDEXED | STORED);
        schema_builder.add_i64_field("section", INDEXED | STORED);

        let mut field_map = HashMap::new();

        for (field_name, field_type) in schema_fields {
            let field = match field_type {
                "text" => schema_builder.add_text_field(field_name, TEXT | STORED),
                "string" => schema_builder.add_text_field(field_name, STRING | STORED),
                "i64" => schema_builder.add_i64_field(field_name, INDEXED | STORED),
                "f64" => schema_builder.add_f64_field(field_name, INDEXED | STORED),
                _ => schema_builder.add_text_field(field_name, TEXT | STORED),
            };
            field_map.insert(field_name.to_string(), field);
        }

        let schema = schema_builder.build();
        
        // Create directory if it doesn't exist
        std::fs::create_dir_all(index_path)?;
        
        let mmap_dir = MmapDirectory::open(index_path)?;
        let directory = ManagedDirectory::wrap(Box::new(mmap_dir))?;
        let mut settings = IndexSettings::default();
        settings.docstore_compression = Compressor::None;
        let index = Index::create(directory, schema.clone(), settings)?;

        let writer = index.writer(50_000_000)?;
        let reader = index
            .reader_builder()
            .try_into()?;

        Ok(Self {
            index,
            schema,
            writer: RwLock::new(Some(writer)),
            reader,
            fields: field_map,
        })
    }

    pub fn load(index_path: &Path) -> Result<Self> {
        let mmap_dir = MmapDirectory::open(index_path)?;
        let directory = ManagedDirectory::wrap(Box::new(mmap_dir))?;
        let mut settings = IndexSettings::default();
        settings.docstore_compression = Compressor::None;
        let index = Index::open(directory)?;
        let schema = index.schema();
        
        // Rebuild field map from schema
        let mut field_map = HashMap::new();
        for (field_id, _field_entry) in schema.fields() {
            let field_name = schema.get_field_name(field_id);
            field_map.insert(field_name.to_string(), field_id);
        }
        
        let writer = index.writer(50_000_000)?;
        let reader = index
            .reader_builder()
            .try_into()?;

        Ok(Self {
            index,
            schema,
            writer: RwLock::new(Some(writer)),
            reader,
            fields: field_map,
        })
    }
}
