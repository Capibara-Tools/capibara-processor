use super::{function::Return, header::HeaderSummary};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct YamlMacro {
    pub summary: Box<String>,
    pub kind: MacroKind,
    pub description: Box<String>,
    pub os_affinity: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Macro {
    pub name: Box<String>,
    pub header: HeaderSummary,
    pub summary: Box<String>,
    pub kind: MacroKind,
    pub description: Box<String>,
    pub os_affinity: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MacroFunction {
    pub returns: Return,
    pub parameters: Vec<TypelessParameter>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct TypelessParameter {
    pub name: Box<String>,
    pub description: Box<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MacroObject {}

#[derive(Serialize, Deserialize)]
pub(crate) enum MacroKind {
    #[serde(rename = "object")]
    Object(MacroObject),
    #[serde(rename = "function")]
    Function(MacroFunction),
}
