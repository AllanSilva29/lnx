use poem_openapi::Object;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct FuzzyExpr {
    #[serde(rename = "$fuzzy")]
    #[oai(rename = "$fuzzy")]
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
    #[serde(default = "default_true", rename = "$prefixlast")]
    #[oai(default = "default_true", rename = "$prefixlast")]
    pub prefix_last_term: bool,
    #[serde(default = "FuzzyExpr::default_one_typo_threshold", rename = "$onetypo")]
    #[oai(default = "FuzzyExpr::default_one_typo_threshold", rename = "$onetypo")]
    pub one_typo_threshold: usize,
    #[serde(default = "FuzzyExpr::default_two_typo_threshold", rename = "$twotypo")]
    #[oai(default = "FuzzyExpr::default_two_typo_threshold", rename = "$twotypo")]
    pub two_typo_threshold: usize,
}

impl FuzzyExpr {
    fn default_one_typo_threshold() -> usize {
        5
    }

    fn default_two_typo_threshold() -> usize {
        8
    }
}

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct FullTextExpr {
    #[serde(rename = "$fulltext")]
    #[oai(rename = "$fulltext")]
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
}

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct PhraseExpr {
    #[serde(rename = "$phrase")]
    #[oai(rename = "$phrase")]
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
    #[serde(rename = "$slop")]
    #[oai(rename = "$slop")]
    pub slop: usize,
}

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct PrefixExpr {
    #[serde(rename = "$prefix")]
    #[oai(rename = "$prefix")]
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
}

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct RegexExpr {
    #[serde(rename = "$regex")]
    #[oai(rename = "$regex")]
    pub ctx: String,
    #[serde(rename = "$fields")]
    #[oai(rename = "$fields")]
    pub fields: Vec<String>,
}

fn default_true() -> bool {
    true
}
