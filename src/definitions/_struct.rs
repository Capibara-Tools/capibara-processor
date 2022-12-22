use super::header::HeaderSummary;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct YamlStruct {
    pub summary: Box<String>,
    pub fields: Vec<Field>,
    pub description: Box<String>,
    pub os_affinity: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub(crate) struct Struct {
    pub name: Box<String>,
    pub header: HeaderSummary,
    pub summary: Box<String>,
    pub fields: Vec<Field>,
    pub description: Box<String>,
    pub os_affinity: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub(crate) struct Field {
    pub name: Box<String>,
    #[serde(rename = "type")]
    pub _type: Box<String>,
    pub description: Box<String>,
}
