use super::*;
use version::Version;

use log::*;
use serde::{Deserialize};
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub authors: Vec<String>,
    pub kind: Option<Kind>,
    pub language: Option<Language>,
    pub version: Version,
    pub files: Option<Vec<String>>,

    pub local: Option<Vec<dependency::Local>>,
    pub external: Option<Vec<dependency::External>>,

    #[serde(skip)]
    contents: String,
}

impl Project {
    pub fn new() -> Self {
        Self {
            name: "".to_owned(),
            description: "".to_owned(),
            authors: Vec::default(),
            kind: Option::None,
            language: Option::None,
            version: Version::new(),
            files: Option::None,
            contents: "".to_owned(),

            local: None,
            external: None,
        }
    }

    /// Get the full "qualifier" name of a Project.
    pub fn get_full_name(&self) -> String {
        /* I find this method really quite coolio beanz. */
        let full_name = &[
            &self.name, ":",
            &self.version.major.to_string(), ".",
            &self.version.minor.to_string(), ".",
            &self.version.patch.to_string()
        ].join("");

        full_name.to_string()
    }

    /// Just read the hy.yml file.
    fn read_hy(&mut self, path: &String) -> Result<(), ProjectError> {
        Ok(self.contents = match fs::read_to_string(path) {
            Ok(v) => v,
            Err(e) => {
                return Err(ProjectError::ReadingError{
                    source: e,
                    path: Box::from(path.to_owned()),
                })  
            },
        })
    }

    /// Deserialize hy.yml into structs with *serde*
    pub fn deserialize_hy(&mut self, path: &String) -> Result<Project, ProjectError> {
        self.read_hy(path)?;
        debug!("File contents:\n{:?}", self.contents);

        let project: Project = match serde_yaml::from_str(&self.contents) {
            Ok(p) => p,
            Err(e) => {
                return Err(ProjectError::ParsingError{
                    source: e,
                    path: Box::from(path.to_owned()),
                })  
            },
        };

        debug!("Root project:\n{:?}", &project);
        
        Ok(project)
    }
}
