use poem_openapi::{Object, Union};
use serde_derive::{Deserialize, Serialize};

use super::filter::{EqExpr, ExistsExpr, GteExpr, GtExpr, LtExpr, LteExpr, NeqExpr, RangeExpr};
use super::morelikethis::MoreLikeThisExpr;
use super::text::{FullTextExpr, FuzzyExpr, PhraseExpr, PrefixExpr, RegexExpr};
use super::textparser::TextParserExpr;

#[derive(Debug, Union, Serialize, Deserialize)]
pub enum WhereClause {
    All(AllExpr),
    Any(AnyExpr),
    AtLeast(AtLeastExpr),
    Not(NotExpr),
    Exists(ExistsExpr),
    Fuzzy(FuzzyExpr),
    FullText(FullTextExpr),
    Phrase(PhraseExpr),
    Prefix(PrefixExpr),
    MoreLikeThis(MoreLikeThisExpr),
    Regex(RegexExpr),
    TextParser(TextParserExpr),
    Eq(EqExpr),
    Neq(NeqExpr),
    Range(RangeExpr),
    Lt(LtExpr),
    Lte(LteExpr),
    Gt(GtExpr),
    Gte(GteExpr),
}

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct AllExpr {
    #[serde(rename = "$all")]
    #[oai(rename = "$all")]
    pub ctx: Vec<WhereClause>,
}

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct AnyExpr {
    #[serde(rename = "$any")]
    #[oai(rename = "$any")]
    pub ctx: Vec<WhereClause>,
}

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct AtLeastExpr {
    #[serde(rename = "$atleast")]
    #[oai(rename = "$atleast")]
    pub ctx: Vec<WhereClause>,
    #[serde(rename = "$threshold")]
    #[oai(rename = "$threshold")]
    pub threshold: usize,
}

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct NotExpr {
    #[serde(rename = "$not")]
    #[oai(rename = "$not")]
    pub ctx: Box<WhereClause>,
}
