use snafu::{ResultExt};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency {
    name: String,
    at: String,
}