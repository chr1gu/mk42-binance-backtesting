use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SymbolResult {
    pub name: String,
    pub next_marker: Option<String>,
    pub common_prefixes: Vec<CommonPrefixes>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct KlineResult {
    pub next_marker: Option<String>,
    pub contents: Option<Vec<Contents>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CommonPrefixes {
    pub prefix: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Contents {
    pub key: String,
}
