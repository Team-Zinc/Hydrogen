use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum Kind {
    #[serde(rename = "console")]
    Console,
    #[serde(rename = "static")]
    StaticLib,
    #[serde(rename = "dynamic")]
    DynamicLib,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Vendor {
    GitHub,
    GitLab,
    BitBucket,
    Git,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Build {
    Hydrogen,
    CMake,
    Custom,
}
