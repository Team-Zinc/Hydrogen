mod hy;
mod dep;
mod py;
mod project_error;

use project_error::ProjectError;
use std::path::Path;
use serde::Deserialize;

// A project is a container that contains a hy.yml, hy.py, and a dep.yml
#[derive(Deserialize, Debug)]
struct Project {
    hy: hy::Hy,
    dep: dep::Dep,
    // TODO: PYTHON INTEGRATION
}

impl Project {
    pub fn parse_from_str<'a>(&mut self, s: &str) -> Result<Project, ProjectError> {
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
