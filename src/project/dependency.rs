use crate::project::{Language, Build};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub enum Vendor {
    Github,
    None,
}

#[derive(Deserialize, Debug)]
pub struct Local {
    location: String,
}

#[derive(Deserialize, Debug)]
pub struct External {
    name: String,
    language: Language,
    location: String,
    build: Build,
    from: String,
    vendor: Vendor,
}
