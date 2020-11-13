use snafu::{ResultExt, Snafu};
use serde::{Serialize, Deserialize};
use path_clean::{clean, PathClean};

use std::{env, fs, io};
use std::path::{Path, PathBuf};

use crate::project;
use crate::project::Project;
use crate::project::ProjectError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub at: String,
    pub project: Option<Project>,
}

impl Dependency {
    pub fn read_dependency(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // First, create a project (this will be our working dependency)
        let mut project = Project::new();

        // Then we will look and read.
        project.read_all()?;

        self.project.replace(project);

        Ok(())
    }

    pub fn parse_dependency(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut project = self.project.take().unwrap();

        // Then we will look and read.
        project.parse_all()?;

        self.project.replace(project);

        Ok(())
    }
}