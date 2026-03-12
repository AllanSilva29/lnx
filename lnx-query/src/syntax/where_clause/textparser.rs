use poem_openapi::Object;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct TextParserExpr {
    #[serde(rename = "$parse")]
    #[oai(rename = "$parse")]
    pub ctx: String,
    #[serde(rename = "$fields")]
    #[oai(rename = "$fields")]
    pub fields: Vec<String>,
    #[serde(rename = "$threshold")]
    #[oai(rename = "$threshold")]
    pub threshold: f32,
    #[serde(rename = "$boost")]
    #[oai(rename = "$boost")]
    pub boost: f32,
    #[serde(rename = "$options")]
    #[oai(rename = "$options")]
    pub parse_options: TextParserConfig,
}

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct TextParserConfig {
    #[serde(default = "default_true", rename = "$boolean")]
    #[oai(default = "default_true", rename = "$boolean")]
    pub boolean_ops_enabled: bool,
    #[serde(default = "default_true", rename = "$negative")]
    #[oai(default = "default_true", rename = "$negative")]
    pub negative_ops_enabled: bool,
    #[serde(default = "default_true", rename = "$must")]
    #[oai(default = "default_true", rename = "$must")]
    pub must_ops_enabled: bool,
    #[serde(default = "default_true", rename = "$phrase")]
    #[oai(default = "default_true", rename = "$phrase")]
    pub phrase_ops_enabled: bool,
    #[serde(default, rename = "$slop")]
    #[oai(default, rename = "$slop")]
    pub slop_ops_enabled: bool,
    #[serde(default = "default_true", rename = "$range")]
    #[oai(default = "default_true", rename = "$range")]
    pub range_ops_enabled: bool,
    #[serde(default, rename = "$in")]
    #[oai(default, rename = "$in")]
    pub set_ops_enabled: bool,
    #[serde(default = "default_true", rename = "$parsewildcard")]
    #[oai(default = "default_true", rename = "$parsewildcard")]
    pub parse_wildcards: bool,
    #[serde(default = "default_true", rename = "$boost")]
    #[oai(default = "default_true", rename = "$boost")]
    pub boost_ops_enabled: bool,
    #[serde(default, rename = "$fuzzyterms")]
    #[oai(default, rename = "$fuzzyterms")]
    pub fuzzy_terms: bool,
    #[serde(default = "default_true", rename = "$strict")]
    #[oai(default = "default_true", rename = "$strict")]
    pub strict: bool,
}

fn default_true() -> bool {
    true
}
