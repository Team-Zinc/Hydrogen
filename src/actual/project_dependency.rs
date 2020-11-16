use serde::{Deserialize, Serialize}; 

use crate::project::Project;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub at: String,
    pub project: Option<Project>,
}
