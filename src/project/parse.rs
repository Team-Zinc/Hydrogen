use crate::project::Project;

use path_clean::PathClean;
use snafu::{ResultExt, Snafu};

use std::path::PathBuf;
use std::{env, io, mem};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ParsingError {
    #[snafu(visibility(pub(crate)))]
    #[snafu(display("Unable to parse configuration from {}: {}", filetype, source))]
    ParseError {
        source: serde_yaml::Error,
        filetype: String,
    },

    #[snafu(visibility(pub(crate)))]
    #[snafu(display("Failed to get current working directory: {}", source))]
    GetDirError { source: io::Error },

    #[snafu(visibility(pub(crate)))]
    #[snafu(display("Failed to change directory to {}: {}", to.display(), source))]
    ChangeDirError { to: PathBuf, source: io::Error },
}

pub trait Parse {
    fn from_string(&mut self, src: &str) -> Result<(), ParsingError>;
}

impl Project {
    /// This function simply calls the parse functions
    /// for meta and fetchfile, and (maybe, TODO) runs the dynamic.
    pub fn parse_all(&mut self) -> Result<(), ParsingError> {
        if self.meta.is_some() {
            // Parse the meta source
            let mut e = self.meta.take().unwrap();
            e.element.from_string(e.src.as_ref())?;

            mem::swap(&mut self.meta, &mut Some(e));
            // self.meta = Some(e);
        }

        if self.static_actual.is_some() {
            // Parse the static actual source
            let mut e = self.static_actual.take().unwrap();
            e.element.from_string(e.src.as_ref())?;

            mem::swap(&mut self.static_actual, &mut Some(e));
            // self.static_actual = Some(e);
        }

        Ok(())
    }

    /// Parses all of its children. Not just the
    /// direct ones, but ALL of them.
    pub fn parse_all_children(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref mut real) = self.real_actual {
            if let Some(ref mut children) = real.dependencies {
                for child in &mut children.iter_mut() {
                    let old_directory = env::current_dir()
                        .context(GetDirError {})?
                        .into_os_string()
                        .into_string()
                        .unwrap();

                    env::set_current_dir(&child.at).context(ChangeDirError {
                        to: PathBuf::from(&child.at).clean(),
                    })?;

                    let mut child_project = Project::new();
                    child_project.read_all()?;
                    child_project.parse_all()?;

                    child_project.parse_all_children()?;
                    child_project.construct_real_actual();

                    child.project = Some(child_project);

                    env::set_current_dir(&old_directory).context(ChangeDirError {
                        to: PathBuf::from(&old_directory).clean(),
                    })?;
                }
            }
        }
        
        Ok(())
    }
}
