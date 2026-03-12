use poem_openapi::types::Example;
use poem_openapi::{Enum, Object, Union};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Union, Serialize, Deserialize)]
pub enum OneOrManySortBy {
    One(SortBy),
    Many(Vec<SortBy>),
}

#[derive(Debug, Object, Serialize, Deserialize)]
#[oai(example = true)]
pub struct SortBy {
    #[serde(rename = "$by")]
    #[oai(rename = "$by")]
    pub by: String,
    #[serde(default, rename = "$order")]
    #[oai(default, rename = "$order")]
    pub order: Order,
}

impl Example for SortBy {
    fn example() -> Self {
        Self {
            by: "$score".to_string(),
            order: Order::Desc,
        }
    }
}

#[derive(Debug, Default, Enum, Serialize, Deserialize)]
#[oai(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Order {
    #[default]
    Desc,
    Asc,
}
