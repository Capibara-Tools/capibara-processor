use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct YamlHeader {
    pub summary: Box<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Header {
    #[serde(rename = "ref")]
    pub _ref: Box<String>,
    pub name: Box<String>,
    pub summary: Box<String>,
    pub os_affinity: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub(crate) struct HeaderSummary {
    #[serde(rename = "ref")]
    pub _ref: Box<String>,
    pub name: Box<String>,
}
