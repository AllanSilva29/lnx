use poem_openapi::types::MaybeUndefined;
use poem_openapi::{Object, Union};
use serde_derive::{Deserialize, Serialize};

use super::sort::OneOrManySortBy;
use super::where_clause::WhereClause;

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct SelectQuery {
    #[serde(rename = "$select")]
    #[oai(rename = "$select")]
    pub select_fields: Vec<String>,
    #[serde(default, rename = "$distinct")]
    #[oai(default, rename = "$distinct")]
    pub distinct: bool,
    #[serde(rename = "$from")]
    #[oai(rename = "$from")]
    pub from_tables: String,
    #[serde(default, rename = "$where")]
    #[oai(default, rename = "$where")]
    pub where_clause: MaybeUndefined<WhereClause>,
    #[serde(default, rename = "$parallel")]
    #[oai(default, rename = "$parallel")]
    pub parallel: bool,
    #[serde(default, rename = "$sort")]
    #[oai(default, rename = "$sort")]
    pub sort_by: Option<OneOrManySortBy>,
    #[serde(default = "default_limit")]
    #[oai(default = "default_limit")]
    pub limit: usize,
    #[serde(default)]
    #[oai(default)]
    pub offset: usize,
}

fn default_limit() -> usize {
    100
}

#[derive(Debug, Union, Serialize, Deserialize)]
enum OneOrManyTables {
    One(String),
    Many(Vec<String>),
}
