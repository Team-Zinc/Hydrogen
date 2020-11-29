use super::project_dependency::Dependency;
use crate::project::parse;
use crate::project::parse::Parse;
use crate::project::parse::ParsingError;

use serde::{Deserialize, Serialize};
use snafu::ResultExt;
// The fetchfile (located in fetch.yml) describes
// where and how to download a dependency.

#[derive(Debug, Serialize, Deserialize)]
pub struct StaticActual {
    pub files: Option<Vec<String>>,
    pub dependencies: Option<Vec<Dependency>>,
}

impl StaticActual {
    pub fn new() -> Self {
        Self {
            files: None,
            dependencies: None,
        }
    }
}

impl Parse for StaticActual {
    fn from_string(&mut self, src: &str) -> Result<(), ParsingError> {
        *self = serde_yaml::from_str(src).context(parse::ParseError {
            filetype: "Build.yml",
            at: std::env::current_dir().unwrap(),
        })?;

        Ok(())
    }
}
