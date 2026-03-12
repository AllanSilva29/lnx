use poem_openapi::types::MaybeUndefined;
use poem_openapi::Object;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct ExistsExpr {
    #[serde(rename = "$exists")]
    #[oai(rename = "$exists")]
    pub fields: Vec<String>,
}

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct RangeExpr {
    #[serde(rename = "$range")]
    #[oai(rename = "$range")]
    pub ctx: RangeBounds,
    #[serde(rename = "$fields")]
    #[oai(rename = "$fields")]
    pub fields: Vec<String>,
}

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct RangeBounds {
    #[serde(rename = "$lt")]
    #[oai(rename = "$lt")]
    pub lt: MaybeUndefined<String>,
    #[serde(rename = "$lte")]
    #[oai(rename = "$lte")]
    pub lte: MaybeUndefined<String>,
    #[serde(rename = "$gt")]
    #[oai(rename = "$gt")]
    pub gt: MaybeUndefined<String>,
    #[serde(rename = "$gte")]
    #[oai(rename = "$gte")]
    pub gte: MaybeUndefined<String>,
}

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct EqExpr {
    #[serde(rename = "$eq")]
    #[oai(rename = "$eq")]
    pub ctx: String,
    #[serde(rename = "$fields")]
    #[oai(rename = "$fields")]
    pub fields: Vec<String>,
}

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct NeqExpr {
    #[serde(rename = "$neq")]
    #[oai(rename = "$neq")]
    pub ctx: String,
    #[serde(rename = "$fields")]
    #[oai(rename = "$fields")]
    pub fields: Vec<String>,
}

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct LtExpr {
    #[serde(rename = "$lt")]
    #[oai(rename = "$lt")]
    pub ctx: String,
    #[serde(rename = "$fields")]
    #[oai(rename = "$fields")]
    pub fields: Vec<String>,
}

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct LteExpr {
    #[serde(rename = "$lte")]
    #[oai(rename = "$lte")]
    pub ctx: String,
    #[serde(rename = "$fields")]
    #[oai(rename = "$fields")]
    pub fields: Vec<String>,
}

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct GtExpr {
    #[serde(rename = "$gt")]
    #[oai(rename = "$gt")]
    pub ctx: String,
    #[serde(rename = "$fields")]
    #[oai(rename = "$fields")]
    pub fields: Vec<String>,
}

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct GteExpr {
    #[serde(rename = "$gte")]
    #[oai(rename = "$gte")]
    pub ctx: String,
    #[serde(rename = "$fields")]
    #[oai(rename = "$fields")]
    pub fields: Vec<String>,
}
