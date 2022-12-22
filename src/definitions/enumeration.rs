use super::header::HeaderSummary;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct YamlEnumeration {
    pub summary: Box<String>,
    pub variants: Vec<Variant>,
    pub description: Box<String>,
    pub os_affinity: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub(crate) struct Enumeration {
    pub name: Box<String>,
    pub header: HeaderSummary,
    pub summary: Box<String>,
    pub variants: Vec<Variant>,
    pub description: Box<String>,
    pub os_affinity: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub(crate) struct Variant {
    pub name: Box<String>,
    pub description: Box<String>,
}
