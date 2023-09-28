use super::header::HeaderSummary;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct YamlFunction {
    pub summary: Box<String>,
    pub returns: Return,
    pub parameters: Vec<Parameter>,
    pub description: Box<String>,
    pub examples: Vec<Example>,
    pub associated: Vec<String>,
    pub os_affinity: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Function {
    pub name: Box<String>,
    pub header: HeaderSummary,
    pub summary: Box<String>,
    pub returns: Return,
    pub parameters: Vec<Parameter>,
    pub description: Box<String>,
    pub examples: Vec<Example>,
    pub associated: Vec<String>,
    pub os_affinity: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Example {
    pub title: Box<String>,
    pub code: Box<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Parameter {
    pub name: Box<String>,
    #[serde(rename = "type")]
    pub _type: Box<String>,
    pub description: Box<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Return {
    #[serde(rename = "type")]
    pub _type: Box<String>,
    pub description: Box<String>,
}
