use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use poem_openapi::types::multipart::Upload;
use poem_openapi::Multipart;
use std::collections::HashMap;
use std::sync::Arc;
use std::process::Command;
use std::path::PathBuf;
use tempfile::TempDir;

use crate::storage::Storage;
use super::Tag;

pub struct LnxDocumentApi {
    storage: Arc<Storage>,
}

impl LnxDocumentApi {
    pub fn new(storage: Arc<Storage>) -> Self {
        Self { storage }
    }
}

#[derive(poem_openapi::Object, serde::Deserialize)]
pub struct IndexDocumentRequest {
    pub index: String,
    pub id: String,
    #[serde(default)]
    pub filename: Option<String>,
    pub document: HashMap<String, serde_json::Value>,
}

#[derive(poem_openapi::Object)]
pub struct IndexDocumentResponse {
    pub success: bool,
    pub indexed: usize,
    pub error: Option<String>,
}

#[derive(Multipart)]
pub struct ConvertDocumentRequest {
    pub file: Upload,
    pub index: String,
}

#[derive(poem_openapi::Object)]
pub struct ConvertDocumentResponse {
    pub success: bool,
    pub pages_indexed: usize,
    pub filename: String,
    pub error: Option<String>,
}

#[OpenApi(tag = Tag::DocumentEndpoints)]
impl LnxDocumentApi {
    #[oai(path = "/documents", method = "post")]
    /// Index a document
    async fn index_document(
        &self,
        Json(payload): Json<IndexDocumentRequest>,
    ) -> Json<IndexDocumentResponse> {
        let filename = payload.filename.as_deref();
        match self.storage.add_document(&payload.index, &payload.id, filename, payload.document) {
            Ok(_) => Json(IndexDocumentResponse {
                success: true,
                indexed: 1,
                error: None,
            }),
            Err(e) => Json(IndexDocumentResponse {
                success: false,
                indexed: 0,
                error: Some(e.to_string()),
            }),
        }
    }

    #[oai(path = "/documents/convert", method = "post")]
    /// Convert and index a document (DOCX/DOC to PDF, then extract with accurate pages)
    async fn convert_and_index(
        &self,
        form: ConvertDocumentRequest,
    ) -> Json<ConvertDocumentResponse> {
        let filename = form.file.file_name().unwrap_or("unknown").to_string();
        let index = form.index;
        
        tracing::info!("Converting document: {} for index: {}", filename, index);
        
        // Read file content
        let content = match form.file.into_vec().await {
            Ok(c) => c,
            Err(e) => return Json(ConvertDocumentResponse {
                success: false,
                pages_indexed: 0,
                filename: filename.clone(),
                error: Some(format!("Failed to read file: {}", e)),
            }),
        };
        
        // Convert DOCX to PDF using LibreOffice
        let pdf_data = match convert_to_pdf(&content, &filename).await {
            Ok(data) => data,
            Err(e) => return Json(ConvertDocumentResponse {
                success: false,
                pages_indexed: 0,
                filename: filename.clone(),
                error: Some(format!("Conversion failed: {}. Make sure LibreOffice is installed.", e)),
            }),
        };
        
        tracing::info!("PDF conversion successful, extracting text...");
        
        // Extract text from PDF with accurate page numbers
        let pages = match extract_pdf_pages(&pdf_data) {
            Ok(p) => p,
            Err(e) => return Json(ConvertDocumentResponse {
                success: false,
                pages_indexed: 0,
                filename: filename.clone(),
                error: Some(format!("PDF extraction failed: {}", e)),
            }),
        };
        
        tracing::info!("Extracted {} pages, indexing...", pages.len());
        
        if pages.is_empty() {
            return Json(ConvertDocumentResponse {
                success: false,
                pages_indexed: 0,
                filename: filename.clone(),
                error: Some("No text could be extracted from the document. Is LibreOffice installed?".to_string()),
            });
        }
        
        // Index each page
        let mut indexed = 0;
        for extracted_page in pages.iter() {
            let mut doc_fields = HashMap::new();
            doc_fields.insert("content".to_string(), serde_json::Value::String(extracted_page.content.clone()));
            doc_fields.insert("page".to_string(), serde_json::Value::Number(extracted_page.page_number.into()));
            
            let doc_id = format!("{}_{}", filename, extracted_page.page_number);
            
            tracing::debug!("Indexing page {} for file '{}' - {} chars", extracted_page.page_number, filename, extracted_page.content.len());
            
            match self.storage.add_document(&index, &doc_id, Some(&filename), doc_fields) {
                Ok(_) => indexed += 1,
                Err(e) => {
                    tracing::error!("Failed to index page {}: {}", extracted_page.page_number, e);
                }
            }
        }
        
        tracing::info!("Successfully indexed {} pages", indexed);
        
        Json(ConvertDocumentResponse {
            success: true,
            pages_indexed: indexed,
            filename,
            error: None,
        })
    }
}

async fn convert_to_pdf(content: &[u8], filename: &str) -> anyhow::Result<Vec<u8>> {
    // Check if LibreOffice is available
    let check = Command::new("libreoffice").arg("--version").output();
    if check.is_err() || !check.unwrap().status.success() {
        anyhow::bail!("LibreOffice is not installed or not in PATH");
    }
    
    // Create temporary directory
    let temp_dir = TempDir::new()?;
    let input_path = temp_dir.path().join(filename);
    let output_dir = temp_dir.path();
    
    // Write input file
    std::fs::write(&input_path, content)?;
    
    // Run LibreOffice conversion
    let output = Command::new("libreoffice")
        .args(&[
            "--headless",
            "--convert-to",
            "pdf",
            "--outdir",
            output_dir.to_str().unwrap(),
            input_path.to_str().unwrap(),
        ])
        .output()?;
    
    if !output.status.success() {
        anyhow::bail!("LibreOffice conversion failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    // Read the generated PDF
    let pdf_filename = PathBuf::from(filename).with_extension("pdf");
    let pdf_path = output_dir.join(pdf_filename);
    
    let pdf_data = std::fs::read(&pdf_path)?;
    
    Ok(pdf_data)
}

#[derive(Debug, Clone)]
struct ExtractedPage {
    page_number: i64,
    content: String,
}

fn extract_pdf_pages(pdf_data: &[u8]) -> anyhow::Result<Vec<ExtractedPage>> {
    // Check if pdftotext is available
    let check = Command::new("pdftotext").arg("-v").output();
    if check.is_err() || !check.as_ref().unwrap().status.success() {
        anyhow::bail!("pdftotext is not installed or not in PATH. Install with: apt-get install poppler-utils");
    }
    
    // Save PDF to temp file
    let temp_dir = TempDir::new()?;
    let pdf_path = temp_dir.path().join("temp.pdf");
    std::fs::write(&pdf_path, pdf_data)?;
    
    // Try using pdftotext with -layout option for better page separation
    tracing::info!("Attempting PDF extraction with pdftotext -layout");
    let output = Command::new("pdftotext")
        .args(&[
            "-layout",           // Maintain layout
            "-nopgbrk",         // Don't insert page breaks
            pdf_path.to_str().unwrap(),
            "-",                // Output to stdout
        ])
        .output();
    
    if let Ok(result) = output {
        if result.status.success() {
            let full_text = String::from_utf8_lossy(&result.stdout).to_string();
            tracing::info!("pdftotext -layout extracted {} characters", full_text.len());
            
            // Split by form feed or estimate pages
            let pages: Vec<ExtractedPage> = full_text
                .split('\x0C')
                .enumerate()
                .map(|(i, s)| ExtractedPage {
                    page_number: (i + 1) as i64,
                    content: s.trim().to_string(),
                })
                .filter(|p| !p.content.is_empty())
                .collect();
            
            if pages.len() > 1 {
                tracing::info!("Extracted {} pages using pdftotext", pages.len());
                return Ok(pages);
            } else {
                tracing::info!("pdftotext -layout returned only 1 page, trying page-by-page extraction");
            }
        } else {
            tracing::warn!("pdftotext -layout failed with status: {}", result.status);
            if !result.stderr.is_empty() {
                tracing::warn!("pdftotext stderr: {}", String::from_utf8_lossy(&result.stderr));
            }
        }
    } else {
        tracing::warn!("Failed to execute pdftotext -layout");
    }
    
    // Fallback: Try extracting page by page with pdftotext
    tracing::info!("Trying page-by-page extraction with pdftotext");
    let mut page_num = 1;
    let mut pages = Vec::new();
    let mut consecutive_empty = 0;
    
    loop {
        tracing::debug!("Extracting page {}", page_num);
        let output = Command::new("pdftotext")
            .args(&[
                "-f", &page_num.to_string(),  // First page
                "-l", &page_num.to_string(),  // Last page
                "-layout",
                pdf_path.to_str().unwrap(),
                "-",
            ])
            .output();
        
        match output {
            Ok(result) if result.status.success() => {
                let page_text = String::from_utf8_lossy(&result.stdout).to_string();
                let trimmed_text = page_text.trim();
                
                if trimmed_text.is_empty() {
                    tracing::debug!("Page {} is empty", page_num);
                    consecutive_empty += 1;
                    
                    // Only stop after 10 consecutive empty pages (likely end of document)
                    if consecutive_empty >= 10 {
                        tracing::info!("Found {} consecutive empty pages, stopping extraction at page {}", consecutive_empty, page_num - 1);
                        break;
                    }
                } else {
                    consecutive_empty = 0; // Reset counter on non-empty page
                    tracing::debug!("Page {} extracted: {} characters", page_num, trimmed_text.len());
                    pages.push(ExtractedPage {
                        page_number: page_num,
                        content: trimmed_text.to_string(),
                    });
                }
                
                page_num += 1;
                
                // Safety limit to prevent infinite loops
                if page_num > 500 {
                    tracing::warn!("Stopping extraction after 500 pages (safety limit)");
                    break;
                }
            }
            Ok(result) => {
                tracing::warn!("Failed to extract page {}: status {}", page_num, result.status);
                if !result.stderr.is_empty() {
                    tracing::debug!("Page {} stderr: {}", page_num, String::from_utf8_lossy(&result.stderr));
                }
                break;
            }
            Err(e) => {
                tracing::error!("Error executing pdftotext for page {}: {}", page_num, e);
                break;
            }
        }
    }
    
    if !pages.is_empty() {
        tracing::info!("Extracted {} pages using page-by-page method", pages.len());
        return Ok(pages);
    }
    
    // Final fallback: use pdf-extract
    tracing::info!("Falling back to pdf-extract");
    let full_text = pdf_extract::extract_text(&pdf_path)
        .map_err(|e| anyhow::anyhow!("PDF extraction error: {}", e))?;
    
    // Try multiple methods to split pages
    let pages: Vec<ExtractedPage> = full_text
        .split('\x0C')  // Form feed character
        .enumerate()
        .map(|(i, s)| ExtractedPage {
            page_number: (i + 1) as i64,
            content: s.trim().to_string(),
        })
        .filter(|p| !p.content.is_empty())
        .collect();
    
    if !pages.is_empty() && pages.len() > 1 {
        tracing::info!("Extracted {} pages using pdf-extract with form feed", pages.len());
        return Ok(pages);
    }
    
    // If no form feeds, try to estimate pages by looking for page indicators
    // or split by double newlines as a rough estimate
    let estimated_pages: Vec<ExtractedPage> = full_text
        .split("\n\n\n")  // Triple newlines often indicate page breaks
        .enumerate()
        .map(|(i, s)| ExtractedPage {
            page_number: (i + 1) as i64,
            content: s.trim().to_string(),
        })
        .filter(|p| !p.content.is_empty() && p.content.len() > 50) // Filter out very short fragments
        .collect();
    
    if !estimated_pages.is_empty() && estimated_pages.len() > 1 {
        tracing::info!("Estimated {} pages using paragraph breaks", estimated_pages.len());
        return Ok(estimated_pages);
    }
    
    // Last resort: return the entire text as one page but warn about potential issue
    tracing::warn!("Could not determine page boundaries, treating entire PDF as one page. Total text length: {} characters", full_text.len());
    Ok(vec![ExtractedPage {
        page_number: 1,
        content: full_text.trim().to_string(),
    }])
}
