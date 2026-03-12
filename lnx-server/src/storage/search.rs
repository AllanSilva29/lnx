use std::collections::HashMap;

use anyhow::Result;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Field, Value};
use tantivy::TantivyDocument;

pub fn search_index(
    index: &tantivy::Index,
    reader: &tantivy::IndexReader,
    fields: &HashMap<String, Field>,
    query_str: &str,
    limit: usize,
) -> Result<Vec<serde_json::Value>> {
    let searcher = reader.searcher();

    let text_fields: Vec<Field> = fields.values().copied().collect();

    tracing::debug!("Searching with fields: {:?}", fields.keys().collect::<Vec<_>>());

    if text_fields.is_empty() {
        tracing::warn!("No text fields in index!");
        return Ok(vec![]);
    }

    let query_parser = QueryParser::for_index(index, text_fields.clone());

    let query_str = if query_str.is_empty() || query_str == "*" {
        "*"
    } else {
        query_str
    };

    tracing::debug!("Parsed query string: {}", query_str);

    let query = query_parser.parse_query(query_str)?;
    tracing::debug!("Query parsed successfully: {:?}", query);

    let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

    let schema = index.schema();
    let text_fields: Vec<Field> = fields.values().copied().collect();
    let filename_field = schema.get_field("filename").ok();
    let page_field = schema.get_field("page").ok();
    let section_field = schema.get_field("section").ok();

    let mut results_map: HashMap<String, HashMap<(Option<i64>, Option<i64>), Vec<serde_json::Value>>> = HashMap::new();

    for (_score, doc_address) in top_docs {
        let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;

        let filename = filename_field
            .and_then(|f| retrieved_doc.get_first(f))
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "Unknown".to_string());

        let page_num = page_field
            .and_then(|f| retrieved_doc.get_first(f))
            .and_then(|v| v.as_i64());

        let section_num = section_field
            .and_then(|f| retrieved_doc.get_first(f))
            .and_then(|v| v.as_i64());

        let mut occurrences = Vec::new();

        for field in &text_fields {
            let field_name = schema.get_field_name(*field).to_string();
            if let Some(value) = retrieved_doc.get_first(*field) {
                if let Some(text) = value.as_str() {
                    // Skip binary or invalid content
                    if !is_valid_text(text) {
                        continue;
                    }
                    
                    // Enhanced matching: check for both exact and partial matches
                    let lower_text = text.to_lowercase();
                    let lower_query = query_str.to_lowercase();
                    
                    // Simplified search terms - only use exact match and basic variations
                    let mut search_terms = vec![lower_query.clone()];
                    
                    // Add singular/plural variation only if it makes sense
                    if lower_query.ends_with('s') && lower_query.len() > 1 {
                        search_terms.push(lower_query[..lower_query.len()-1].to_string());
                    } else if !lower_query.ends_with('s') {
                        search_terms.push(lower_query.clone() + "s");
                    }
                    
                    // Remove duplicates
                    let unique_terms: std::collections::HashSet<_> = search_terms.into_iter().collect();
                    let search_terms: Vec<_> = unique_terms.into_iter().collect();
                    
                    // Find all unique occurrences with better deduplication
                    let mut all_matches = std::collections::BTreeMap::new();
                    
                    for term in &search_terms {
                        if lower_text.contains(term) {
                            let mut start_byte = 0;
                            while let Some(pos) = lower_text[start_byte..].find(term) {
                                let match_byte_pos = start_byte + pos;
                                
                                // Check if this position overlaps with any existing match
                                let mut overlaps = false;
                                for &existing_pos in all_matches.keys() {
                                    let existing_end = existing_pos + search_terms.iter()
                                        .find(|t| lower_text[existing_pos..].starts_with(*t))
                                        .map(|t| t.len())
                                        .unwrap_or(term.len());
                                    
                                    let current_end = match_byte_pos + term.len();
                                    
                                    // Check if ranges overlap (with small tolerance for adjacent matches)
                                    if !(match_byte_pos >= existing_end || current_end <= existing_pos) {
                                        overlaps = true;
                                        break;
                                    }
                                }
                                
                                if overlaps {
                                    start_byte = match_byte_pos + term.len();
                                    continue;
                                }

                                let text_before_match = &text[..match_byte_pos];
                                let line = text_before_match.matches('\n').count() + 1;

                                let line_start = text_before_match
                                    .rfind('\n')
                                    .map(|p| p + 1)
                                    .unwrap_or(0);
                                let char_pos = text[line_start..match_byte_pos].chars().count();

                                let snippet_start = find_char_boundary(text, match_byte_pos.saturating_sub(40));
                                let snippet_end = find_char_boundary(
                                    text,
                                    (match_byte_pos + term.len() + 40).min(text.len()),
                                );
                                let snippet = &text[snippet_start..snippet_end];

                                all_matches.insert(match_byte_pos, serde_json::json!({
                                    "field": field_name,
                                    "line": line,
                                    "char": char_pos,
                                    "snippet": snippet.trim(),
                                    "matched_term": term
                                }));

                                start_byte = match_byte_pos + term.len();
                            }
                        }
                    }
                    
                    // Add matches in order
                    for (_, occurrence) in &all_matches {
                        occurrences.push(occurrence.clone());
                    }
                    
                    // Debug logging
                    if !all_matches.is_empty() {
                        tracing::debug!("Found {} occurrences for '{}' in page {} of {} (searched: {:?})", all_matches.len(), query_str, page_num.unwrap_or(0), filename, search_terms);
                    }
                }
            }
        }

        results_map
            .entry(filename.clone())
            .or_insert_with(HashMap::new)
            .entry((page_num, section_num))
            .or_insert_with(Vec::new)
            .extend(occurrences);
    }

    let mut results = Vec::new();
    for (filename, blocks) in results_map {
        let mut page_blocks = Vec::new();
        let mut total_occurrences = 0;

        for ((page_num, section_num), matches) in blocks {
            let occurrence_count = matches.len();
            if occurrence_count > 0 {
                total_occurrences += occurrence_count;
                page_blocks.push(serde_json::json!({
                    "page": page_num,
                    "section": section_num,
                    "occurrences": occurrence_count,
                    "matches": matches
                }));
            }
        }

        page_blocks.sort_by_key(|b| {
            let page = b.get("page").and_then(|p| p.as_i64()).unwrap_or(0);
            let section = b.get("section").and_then(|s| s.as_i64()).unwrap_or(0);
            (page, section)
        });

        results.push(serde_json::json!({
            "filename": filename,
            "total_occurrences": total_occurrences,
            "page_blocks": page_blocks
        }));
    }
    
    // Filter out results with zero occurrences
    results.retain(|result| {
        result.get("total_occurrences")
            .and_then(|o| o.as_u64())
            .map(|count| count > 0)
            .unwrap_or(false)
    });

    Ok(results)
}

fn is_valid_text(s: &str) -> bool {
    // Check if string is valid UTF-8 and doesn't have too many control characters
    if !s.is_ascii() {
        // For non-ASCII, check if it's valid UTF-8
        std::str::from_utf8(s.as_bytes()).is_ok()
    } else {
        // For ASCII, allow printable characters and common whitespace
        let non_printable = s.bytes().filter(|&b| b < 32 && b != 9 && b != 10 && b != 13).count();
        // Allow up to 5% non-printable characters
        non_printable * 20 < s.len()
    }
}

fn find_char_boundary(s: &str, byte_index: usize) -> usize {
    if byte_index >= s.len() {
        return s.len();
    }

    let mut index = byte_index;
    while index > 0 && !s.is_char_boundary(index) {
        index -= 1;
    }
    index
}
