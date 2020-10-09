use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DependencyDeclaration {
    pub at: String,
    #[serde(rename = "type")]
    pub dep_type: DependencyDeclarationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DependencyDeclarationType {
    Local,
    External,
} 
