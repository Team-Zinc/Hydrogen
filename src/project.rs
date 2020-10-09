pub mod hy;
pub mod dep;
pub mod py;
pub mod project_error;
pub mod types;
pub mod dependency;

use hy::Hy;
use dep::Dep;
use console::style;
use dependency::DependencyDeclaration;
use project_error::ProjectError;
use std::path::Path;
use std::fs;
use std::borrow::Borrow;
use serde::Deserialize;

// A project is a container that contains a hy.yml, hy.py, and a dep.yml
#[derive(Deserialize, Debug)]
pub struct Project {
    pub hy: Hy,
    pub dep: Option<Dep>,
    // TODO: PYTHON INTEGRATION
}

impl Project {
    pub fn new_root() -> Self {
        Self {
            hy: Hy::new(),
            dep: None,
        }
    }

    pub fn new() -> Self {
        Project::new_root()
    }
    
    pub fn walk_dependencies(&self) -> Result<Vec<Self>, ProjectError> {
        let mut walked_dependencies: Vec<Self> = Vec::new();
        let dependencies = match &self.hy.dependencies {
            Some(d) => d,
            None => return Ok(vec![]),
        };
        
        for dependency in dependencies {
             // Here we look for and parse where the current Project Dependency Declaration is pointing to.
            let mut dependency_as_project = Project::new();
            let at: String = dependency.at.clone() + &"/hy.yml".to_owned();
            let walked: Vec<Self>;
            
            if ! Path::new(&at).exists() {
                /* println!("{} Failed to find dependency at {} (required by {})!",
                    style("[!!!]").bold().red(),
                    &at, self.hy.get_pretty()
                ); */

                return Err(ProjectError::FindError {
                    reason: Box::from(format!("failed to find dependency at {} (required by {})!",
                    &at, self.hy.get_pretty()))
                });
            }

            dependency_as_project = dependency_as_project.parse_from_str(match &fs::read_to_string(&at) {
                Ok(s) => s,
                Err(e) => return Err(ProjectError::ReadingError {
                    source: e,
                    reason: Box::from(format!("failed to read dependency file at {} (required by {})!",
                    &at, self.hy.get_pretty()))
                }),
            })?;
            walked_dependencies.append(&mut dependency_as_project.walk_dependencies()?);

            // Now we walk that and append to our walked_dependencies
        }

        Ok(walked_dependencies)
    }

    pub fn parse_from_str<'a>(&self, s: &str) -> Result<Project, ProjectError> {
        let p: Project = match serde_yaml::from_str(s) {
            Ok(s) => s,
            Err(e) => {
                return Err(ProjectError::ParsingError {
                    source: e,
                });
            }
        };

        Ok(p)
    }
}

pub fn find_root<'a>() -> Option<&'a str> {
    if ! Path::new("./hy.yml").exists() {
        return None;
    }

    Some("hy.yml")
}
