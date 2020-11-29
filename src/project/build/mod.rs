pub mod build;
pub mod configure;
pub mod langs;
pub mod language;
pub mod link;

use super::Project;
use crate::project;
use crate::project::build::configure::ConfigurePool;

use snafu::Snafu;

use std::path::PathBuf;

#[derive(Debug, Snafu)]
pub enum BuildError {
    #[snafu(visibility(pub(crate)))]
    #[snafu(display("Unable to discern language from {}.", path.display()))]
    LanguageDiscernError { path: PathBuf },

    #[snafu(visibility(pub(crate)))]
    #[snafu(display("Unable to create target directory: {}.", source))]
    TargetDirError { source: std::io::Error },

    #[snafu(visibility(pub(crate)))]
    #[snafu(display("Compiler returned as failure"))]
    CompilerError { },

    #[snafu(visibility(pub(crate)))]
    #[snafu(display("Linker/archiver returned as failure"))]
    LinkerError { },
}

/// This struct holds a future of
/// an already building or configuration
/// task. It also contains a BuildContext
#[derive(Debug)]
pub struct BuiltJob<'a> {
    pub context: Option<configure::BuildContext>,
    pub project: Box<&'a Project>,
    pub build_result: Result<(), Box<dyn std::error::Error>>,
    pub link_result: Result<(), Box<dyn std::error::Error>>,
    pub time: u128,
}

/// Holds data about projects already
/// compiled, like where to tell the
/// linker to look for a library.
/// Makes build separation a lot
/// easier.
pub struct BuildPool<'a> {
    pub num_threads: u32,
    pub built: Vec<Box<BuiltJob<'a>>>,
    pub projects: Vec<Vec<&'a Project>>,
    pub configure_pool: configure::ConfigurePool,
}

impl<'a> BuiltJob<'a> {
    pub fn get_success_description(&self) -> String {
        [
            self.project.to_string(),
            "was built in".into(),
            self.time.to_string(),
            "milliseconds".into(),
        ]
        .join(" ")
    }

    pub fn get_failure_description(&self) -> String {
        if self.build_result.is_err() {
            return [
                self.project.to_string(),
                "had a build failure after".into(),
                self.time.to_string(),
                "milliseconds".into(),
            ]
            .join(" ");
        } else if self.link_result.is_err() {
            return [
                self.project.to_string(),
                "had a linking failure after".into(),
                self.time.to_string(),
                "milliseconds".into(),
            ]
            .join(" ");
        }

        "".into()
    }
}

impl<'a> BuildPool<'a> {
    pub fn new(ps: Vec<Vec<&'a Project>>) -> Self {
        Self {
            num_threads: 4,
            built: Vec::new(),
            projects: ps,
            configure_pool: configure::ConfigurePool::new(),
        }
    }
}

impl<'a> BuiltJob<'a> {
    pub fn new(p: &'a Project) -> Self {
        Self {
            context: None,
            project: Box::from(p),
            build_result: Ok(()),
            link_result: Ok(()),
            time: 0,
        }
    }
}
