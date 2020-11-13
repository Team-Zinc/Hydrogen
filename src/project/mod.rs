pub mod kinds;
pub mod parse;

use crate::actual::{real_actual::RealActual, static_actual::StaticActual};
use crate::fetchfile::Fetchfile;
use crate::meta::Meta;

use std::path::Path;
use std::{fs, io};

use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};

// TODO: Make file names "prettier"
/// Where to look for the meta file.
const META_FILE: &str = "Hydrogen.yml";
/// Where to look for the fetch file.
const FETCH_FILE: &str = "Fetch.yml";
/// Where to look for the static actual file.
const STATIC_BUILD_FILE: &str = "Build.yml";
/// Where to look for the dynamic actual file.
// const DYNAMIC_BUILD_FILE: &str = "Build.py";

#[derive(Debug, Snafu)]
pub enum ProjectError {
    #[snafu(visibility(pub(crate)))]
    #[snafu(display("Unable to read configuration from {}", path))]
    ConfigReadError {
        path: &'static str,
        source: io::Error,
    },
}

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
    pub fn read_all(&mut self) -> Result<(), ProjectError> {
        if Path::new(META_FILE).exists() {
            /* let src = fs::read_to_string(META_FILE)
            .context(IOConfigError {
                path: META_FILE,
            })?; */

            let src = fs::read_to_string(META_FILE).context(ConfigReadError { path: META_FILE })?;

            self.meta = Some(Box::from(ProjectElement {
                element: Meta::new(),
                src: src,
            }));
        }

        if Path::new(FETCH_FILE).exists() {
            let src =
                fs::read_to_string(FETCH_FILE).context(ConfigReadError { path: FETCH_FILE })?;

            self.fetchfile = Some(Box::from(ProjectElement {
                element: Fetchfile::new(),
                src: src,
            }));
        }

        if Path::new(STATIC_BUILD_FILE).exists() {
            let src = fs::read_to_string(STATIC_BUILD_FILE).context(ConfigReadError {
                path: STATIC_BUILD_FILE,
            })?;

            self.static_actual = Some(Box::from(ProjectElement {
                element: StaticActual::new(),
                src: src,
            }));
        }

        Ok(())
    }

    /// Constructs a RealActual from a StaticActual.
    /// Consumes the static actual.
    pub fn construct_real_actual(&mut self) {
        if self.static_actual.is_none() {
            return;
        }
        let mut r = RealActual::new();
        r.from_static(self.static_actual.take().unwrap().element);
        self.real_actual = Some(Box::from(r));
    }
}
