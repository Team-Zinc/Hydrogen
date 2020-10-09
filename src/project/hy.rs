use serde::Deserialize;
use super::types::Kind;
use super::dependency::DependencyDeclaration;
use super::ProjectError;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Contains project metadata, such as naming and such. It may also contain a static actual.
#[derive(Deserialize, Debug)]
pub struct Hy {
    pub name: String,
    pub kind: Kind,
    pub description: String,
    pub authors: Vec<String>,
    pub version: String,

    pub dependencies: Option<Vec<DependencyDeclaration>>,
}

impl Hy {
    pub fn new() -> Self {
        /*
         * It seems bad setting the default kind to Console,
         * but in the end, the configuration file MUST SPECIFY
         * this. Serde ensures.
         */

        Self {
            name: "".to_owned(),
            kind: Kind::Console,
            description: "".to_owned(),
            authors: vec![],
            version: "0.0.0".to_owned(),

            dependencies: None,
        }
    }
    
    pub fn get_pretty(&self) -> String {
        format!("{} v{}", &self.name, &self.version)
    }

    pub fn parse_from_file(path: String) -> Result<Hy, ProjectError> {
        // TODO: Split into two functions maybe?
        
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                return Err(ProjectError::ReadingError {
                    reason: format!("failed to read file at {}", path),
                    source: Box::from(e),
                });
            }
        };

        let reader = BufReader::new(file);

        let h = match serde_yaml::from_reader(reader) {
            Ok(f) => f,
            Err(e) => {
                return Err(ProjectError::ParsingError{
                    source: e,
                });
            }
        };

        Ok(h)
    }
}
