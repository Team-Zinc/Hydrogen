use serde::{Deserialize, Serialize};
use super::types::{Vendor, Build};

/// Contains information on how to download an external dependency from a remote source.
#[derive(Deserialize, Serialize, Debug)]
pub struct Dep {
    pub name: String,
    pub description: String,
    pub vendor: Vendor,
    pub from: String,
    pub version: String,
    pub build: Build,
}
