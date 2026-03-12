use std::collections::BTreeMap;

use poem_openapi::{Object, Union};
use serde_derive::{Deserialize, Serialize};

use super::super::query::SelectQuery;

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct MoreLikeThisExpr {
    #[serde(rename = "$morelikethis")]
    #[oai(rename = "$morelikethis")]
    pub ctx: MoreLikeThisBounds,
    #[serde(rename = "$threshold")]
    #[oai(rename = "$threshold")]
    pub threshold: f32,
    #[serde(rename = "$boost")]
    #[oai(rename = "$boost")]
    pub boost: f32,
}

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct MoreLikeThisBounds {
    #[serde(rename = "$from")]
    #[oai(rename = "$from")]
    pub from: MoreLikeThisFrom,
    #[serde(default, rename = "$stopwords")]
    #[oai(default, rename = "$stopwords")]
    pub stop_words: Vec<String>,
    #[serde(rename = "$docfrequency")]
    #[oai(rename = "$docfrequency")]
    pub doc_frequency: Option<MoreLikeThisDocFreq>,
}

#[derive(Debug, Union, Serialize, Deserialize)]
pub enum MoreLikeThisFrom {
    Query(MoreLikeThisFromQuery),
    Docs(MoreLikeThisFromDocs),
}

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct MoreLikeThisFromQuery {
    #[serde(rename = "$query")]
    #[oai(rename = "$query")]
    pub query: Box<SelectQuery>,
}

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct MoreLikeThisFromDocs {
    #[serde(rename = "$docs")]
    #[oai(rename = "$docs")]
    pub docs: Vec<BTreeMap<String, serde_json::Value>>,
}

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct MoreLikeThisDocFreq {
    #[serde(rename = "$min")]
    #[oai(rename = "$min")]
    pub min: Option<usize>,
    #[serde(rename = "$max")]
    #[oai(rename = "$max")]
    pub max: Option<usize>,
}
