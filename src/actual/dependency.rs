use snafu::{ResultExt};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub at: String,
}