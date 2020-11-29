pub mod actual;
pub mod build;
pub mod kinds;
pub mod meta;
pub mod parse;

use crate::project::meta::Meta;
use actual::{real_actual::RealActual, static_actual::StaticActual};

use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use std::{fmt, fs, io};

use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};

// TODO: Make file names "prettier"
/// Where to look for the meta file.
const META_FILE: &str = "Hydrogen.yml";
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

    #[snafu(visibility(pub(crate)))]
    #[snafu(display("Unable to find mandatory Hydrogen.yml file in {}", which.display()))]
    NoMeta { which: PathBuf },

    #[snafu(visibility(pub(crate)))]
    #[snafu(display("Failed to get current working directory: {}", source))]
    GetDirError { source: io::Error },

    #[snafu(visibility(pub(crate)))]
    #[snafu(display("Failed to change directory to {}: {}", to.display(), source))]
    ChangeDirError { to: PathBuf, source: io::Error },
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
    pub static_actual: Option<Box<ProjectElement<StaticActual>>>,
    pub real_actual: Option<Box<RealActual>>,

    pub deep: i16,
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.to_string().as_str())?;
        Ok(())
    }
}

impl Project {
    pub fn new() -> Self {
        Self {
            meta: None,
            static_actual: None,
            real_actual: None,
            deep: 0,
        }
    }

    pub fn get_name(&self) -> Option<&String> {
        if let Some(ref meta) = self.meta {
            return Some(&meta.element.name);
        }

        None
    }

    pub fn get_files(&self) -> Option<Vec<PathBuf>> {
        if let Some(ref real) = self.real_actual {
            if let Some(ref files) = real.files {
                return Some(files.clone());
            }
        }

        None
    }

    pub fn to_string(&self) -> std::string::String {
        let mut name: &str = "DOOFUS";
        let mut version: &str = "DOOFUS";

        if let Some(ref meta) = self.meta {
            name = meta.element.name.as_str();
            version = meta.element.version.as_str();
        }

        [name, ":", version].join("")
    }

    /// Looks for a project in the current directory, and
    /// read it if it exists.
    pub fn read_all(&mut self) -> Result<(), ProjectError> {
        if !Path::new(META_FILE).exists() {
            return Err(ProjectError::NoMeta {
                which: std::env::current_dir().unwrap(),
            });
        }

        let src = fs::read_to_string(META_FILE).context(ConfigReadError { path: META_FILE })?;

        self.meta = Some(Box::from(ProjectElement {
            element: Meta::new(),
            src: src,
        }));

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
    pub fn construct_real_actual(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.static_actual.is_none() {
            return Ok(());
        }
        let mut r = RealActual::new();
        r.from_static(self.static_actual.take().unwrap().element)?;
        self.real_actual = Some(Box::from(r));

        Ok(())
    }

    /// Recurse to get every single child.
    pub fn get_all_children(&self) -> Option<Vec<&Project>> {
        let mut stack: Vec<&Project> = Vec::new();

        if let Some(ref real) = self.real_actual {
            if let Some(ref deps) = real.dependencies {
                for dep in deps {
                    if let Some(ref project) = dep.project {
                        stack.push(project);

                        if let Some(ref mut children) = project.get_all_children() {
                            stack.append(children);
                        }
                    }
                }
            }
        }

        stack.reverse();
        Some(stack)
    }

    /// Just a wrapper around .get_all_children()
    pub fn topo_sort(&self) -> Option<Vec<&Project>> {
        self.get_all_children()
    }
}

/// Removes duplicates from a vector of projects.
fn remove_dups(projects: Vec<&Project>) -> Vec<&Project> {
    let mut ret: Vec<&Project> = Vec::new();
    let mut set: HashSet<String> = HashSet::new();
    for project in projects {
        let as_string = project.to_string();

        if set.contains(&as_string) {
            // Duplicate
            continue;
        }

        ret.push(project);
        set.insert(as_string);
    }

    ret.reverse();

    ret
}

/// We need to sort all projects by their deepness.
pub fn into_layer_sort(unsorted_projects: Vec<&Project>) -> Option<Vec<Vec<&Project>>> {
    let mut ret: Vec<Vec<&Project>> = Vec::new();
    let projects = remove_dups(unsorted_projects);

    for project in projects {
        let deep = project.deep * -1;
        if ret.len() < deep as usize {
            // Doesn't exist
            for _ in 0..(deep - (ret.len() as i16)) {
                ret.push(Vec::new());
            }
        }

        ret[(deep - 1) as usize].insert(0, project);
    }

    ret.reverse();

    Some(ret)
}
