pub mod parse;
pub mod kinds;
pub mod project_error;

use crate::meta::Meta;
use crate::fetchfile::Fetchfile;
use crate::actual::{static_actual::StaticActual, real_actual::RealActual};
// use crate::project::project_error::ProjectError;
// use crate::project::parse::Parse;

use std::fs;
use std::path::Path;

use serde::{Serialize, Deserialize}; 

// TODO: Make file names "prettier"
/// Where to look for the meta file.
const META_FILE: &str = "Hydrogen.yml";
/// Where to look for the fetch file.
const FETCH_FILE: &str = "Fetch.yml";
/// Where to look for the static actual file.
const STATIC_BUILD_FILE: &str = "Build.yml";
/// Where to look for the dynamic actual file.
// const DYNAMIC_BUILD_FILE: &str = "Build.py";

/// A templated generic struct for
/// holding sources and the actual
/// struct on an element.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectElement<T> {
    /// An element is simple a struct like Meta or StaticActual.
    pub element: T,
    /// The source of the element.
    pub src: String,
}

/// A project is simply a container for a
/// metadata file (Hydrogen.yml),
/// a dependency declaration file (Fetch.yml),
/// a static actual file (Build.yml)
/// and a dynamic actual file (Build.py).
/// 
/// Please note the dynamic actual files are not yet implemented.
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub meta: Option<Box<ProjectElement<Meta>>>,
    pub fetchfile: Option<Box<ProjectElement<Fetchfile>>>,
    pub static_actual: Option<Box<ProjectElement<StaticActual>>>,
    pub real_actual: Option<Box<RealActual>>,
}

impl Project {
    pub fn new() -> Self {
        Self {
            meta: None,
            fetchfile: None,
            static_actual: None,
            real_actual: None,
        }
    }

    /// Looks for a project in the current directory, and
    /// read it if it exists.
    pub fn read_all(&mut self) -> Result<(), std::io::Error> {
        if Path::new(META_FILE).exists() {
            self.meta = Some(Box::from(ProjectElement{
                element: Meta::new(),
                src: fs::read_to_string(META_FILE)?,
            }));
        }

        if Path::new(FETCH_FILE).exists() {
            self.fetchfile = Some(Box::from(ProjectElement{
                element: Fetchfile::new(),
                src: fs::read_to_string(FETCH_FILE)?,
            }));
        }

        if Path::new(STATIC_BUILD_FILE).exists() {
            self.static_actual = Some(Box::from(ProjectElement{
                element: StaticActual::new(),
                src: fs::read_to_string(STATIC_BUILD_FILE)?,
            }));
        }

        Ok(())
    }

    /// # NOT IMPLEMENTED
    /// Constructs a RealActual from a DynamicActual.
    /// Consumes the dynamic actual.
    pub fn apply_dynamic_to_real(&mut self) {
        panic!("NOT IMPLEMENTED: apply_dynamic: RealActual");
    }

    /// Constructs a RealActual from a StaticActual.
    /// Consumes the static actual.
    pub fn static_to_real(&mut self) {
        let mut r = RealActual::new();
        r.from_static(self.static_actual.take().unwrap().element); 
        self.real_actual = Some(Box::from(r));
    }
}
