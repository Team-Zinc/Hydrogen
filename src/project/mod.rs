pub mod parse;
pub mod kinds;
pub mod project_error;

use crate::meta::Meta;
use crate::fetchfile::Fetchfile;
use crate::actual::{static_actual::StaticActual};
use crate::project::project_error::ProjectError;
use crate::project::parse::Parse;

use std::fs;
use std::path::Path;
use std::env;
use std::error::Error;

// TODO: Make file name "prettier"
/// Where to look for the meta file.
const META_FILE: &str = "Hydrogen.yml";
/// Where to look for the fetch file.
const FETCH_FILE: &str = "Fetch.yml";
/// Where to look for the static actual file.
const STATIC_BUILD_FILE: &str = "Build.yml";
/// Where to look for the dynamic actual file.
const DYNAMIC_BUILD_FILE: &str = "Build.py";

/// A project is simply a container for a
/// metadata file (Hydrogen.yml),
/// a dependency declaration file (Fetch.yml),
/// a static actual file (Build.yml)
/// and a dynamic actual file (Build.py).
/// 
/// # NOTE:
/// If a file is found, the corresponding
/// source field is set to Some from None
/// 
/// Please note the dynamic actual files are not yet implemented.
#[derive(Debug)]
pub struct Project {
    meta_src: Option<String>,
    fetch_src: Option<String>,
    static_src: Option<String>,
    dynamic_src: Option<String>,

    pub meta: Meta,
    pub fetchfile: Fetchfile,
    pub static_actual: StaticActual,
}

impl Project {
    pub fn new() -> Self {
        Self {
            meta_src: None,
            fetch_src: None,
            static_src: None,
            dynamic_src: None,

            meta: Meta::new(),
            fetchfile: Fetchfile::new(),
            static_actual: StaticActual::new(),
        }
    }

    /// Looks for a project in the current directory.
    pub fn look_for(&mut self) {
        if Path::new(META_FILE).exists() {
            self.meta_src = Some(String::new());
        }

        if Path::new(FETCH_FILE).exists() {
            self.fetch_src = Some(String::new());
        }

        if Path::new(STATIC_BUILD_FILE).exists() {
            self.static_src = Some(String::new());
        }

        if Path::new(DYNAMIC_BUILD_FILE).exists() {
            self.dynamic_src = Some(String::new());
        }
    }

    /// Reads all the project specific files. Gets data from layout.
    pub fn read_all(&mut self) -> Result<(), std::io::Error> {
        if self.meta_src.is_some() {
            self.meta_src = Some(fs::read_to_string(META_FILE)?);
        }

        if self.fetch_src.is_some() {
            self.fetch_src = Some(fs::read_to_string(FETCH_FILE)?);
        }

        if self.static_src.is_some() {
            self.static_src = Some(fs::read_to_string(STATIC_BUILD_FILE)?)
        }

        if self.dynamic_src.is_some() {
            self.dynamic_src = Some(fs::read_to_string(DYNAMIC_BUILD_FILE)?);
        }

        Ok(())
    }

    /// This function simply calls the parse functions
    /// for meta and fetchfile, and (maybe, TODO) runs the dynamic.
    pub fn parse_all(&mut self) -> Result<(), ProjectError> {
        if self.meta_src.is_some() {
            // Parse the meta source
            self.meta.from_string(self.meta_src.as_ref().unwrap())?;
        }

        if self.fetch_src.is_some() {
            // Parse the fetch source
            self.fetchfile.from_string(self.meta_src.as_ref().unwrap())?;
        }

        if self.static_src.is_some() {
            // Parse the static actual source
            self.static_actual.from_string(self.static_src.as_ref().unwrap())?;
        }

        // TODO: Run dynamic configuration

        Ok(())
    }
}
