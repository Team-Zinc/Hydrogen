use snafu::{ResultExt, Snafu};
use serde::{Serialize, Deserialize};
use path_clean::{clean, PathClean};

use std::{env, fs, io};
use std::path::{Path, PathBuf};

use crate::project;
use crate::project::Project;
use crate::project::ProjectError__;

#[derive(Debug, Snafu)]
pub enum ProjectDependencyError {
    #[snafu(visibility(pub(crate)))]
    #[snafu(display("Failed to get current working directory: {}", source))]
    GetDirError { 
        source: io::Error,
    },

    #[snafu(visibility(pub(crate)))]
    #[snafu(display("Failed to change directory to {}: {}", to.display(), source))]
    ChangeDirError {
        to: PathBuf,
        source: io::Error,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub at: String,
    pub project: Option<Project>,
}

impl Dependency {
    pub fn read_dependency(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let old_directory = env::current_dir()
            .context(GetDirError {})?
            .into_os_string().into_string().unwrap();
        env::set_current_dir(&self.at)
            .context(ChangeDirError {
                to: PathBuf::from(&self.at).clean(),
            })?;

        // First, create a project (this will be our working dependency)
        let mut project = Project::new();

        // Then we will look and read.
        project.read_all()?;

        env::set_current_dir(&old_directory).context(ChangeDirError {
            to: fs::canonicalize(&old_directory).unwrap()
        })?;

        self.project.replace(project);

        Ok(())
    }

    pub fn parse_dependency(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let old_directory = env::current_dir()
            .context(GetDirError {})?
            .into_os_string().into_string().unwrap();

        env::set_current_dir(&self.at)
            .context(ChangeDirError {
                to: PathBuf::from(&self.at).clean(),
            })?;

        let mut project = self.project.take().unwrap();

        // Then we will look and read.
        project.parse_all()?;

        env::set_current_dir(&old_directory).context(ChangeDirError {
            to: fs::canonicalize(&old_directory).unwrap()
        })?;

        self.project.replace(project);

        Ok(())
    }
}