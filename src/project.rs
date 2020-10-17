pub mod layout;

use layout::Layout;
use std::fs;
use std::path::Path;

const META_FILE_NAME: &str = "hy.yml";
const FETCH_FILE_NAME: &str = "dep.yml";
const DYNAMIC_FILE_NAME: &str = "hy.py";

/// A project is simply a container for a
/// metadata and static actual file (hy.yml),
/// a dependency declaration file (fetch.yml, optional),
/// and a dynamic actual file (hy.py, optional).
/// 
/// Please note the dynamic actual files are not yet implemented.
#[derive(Debug)]
pub struct Project {
    pub layout: Layout,

    meta_src: Option<String>,
    fetch_src: Option<String>,
    dynamic_src: Option<String>,
}

impl Project {
    pub fn new() -> Self {
        Self {
            layout: Layout::new(),
            meta_src: None,
            fetch_src: None,
            dynamic_src: None,
        }
    }

    /// Looks for a project in the current directory.
    pub fn look_for(&mut self) {
        if Path::new(META_FILE_NAME).exists() {
            self.layout.has_meta = true;
        }

        if Path::new(FETCH_FILE_NAME).exists() {
            self.layout.has_fetch = true;
        }

        if Path::new(DYNAMIC_FILE_NAME).exists() {
            self.layout.has_dynamic = true;
        }
    }

    /// Reads all the project specific files. Gets data from layout.
    pub fn read_all(&mut self) -> Result<(), std::io::Error> {
        if self.layout.has_meta {
            self.meta_src = Some(fs::read_to_string(META_FILE_NAME)?);
        }

        if self.layout.has_fetch {
            self.meta_src = Some(fs::read_to_string(FETCH_FILE_NAME)?);
        }

        if self.layout.has_dynamic {
            self.meta_src = Some(fs::read_to_string(DYNAMIC_FILE_NAME)?);
        }

        Ok(())
    }

    /// This function simply calls the parse functions
    /// for meta and fetchfile, and (TODO) runs the dynamic.
    pub fn parse_all(&self) {

    }
}
