use crate::project::{parse::Parse, kinds::{Language, Type}};
use crate::project::parse::ParsingError;
use crate::project::parse;

use snafu::{ResultExt};
use serde::{Serialize, Deserialize};

/// Metadata (located in hy.yml) describes
/// certain data about the project (like names and authors).
/// It can contain static actuals, but no
/// dependency fetching rules.
#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub name: String,
    pub description: String,
    pub authors: Vec<String>,
    pub version: String,

    #[serde(rename = "type")]
    #[serde(default)]
    pub type_of: Type,
    #[serde(default)]
    pub language: Language,
}

impl Meta {
    pub fn new() -> Self {
        Self {
            name: "".into(),
            description: "".into(),
            authors: vec![],
            version: "".into(),

            type_of: Type::default(),
            language: Language::default(),
        }
    }
}

impl Parse for Meta {
    fn from_string(&mut self, src: &str) -> Result<(), ParsingError> {
        *self = serde_yaml::from_str(src).context(
            parse::ParseError {
                filetype: "meta",
            }
        )?; 
        
        Ok(())
    }
}
