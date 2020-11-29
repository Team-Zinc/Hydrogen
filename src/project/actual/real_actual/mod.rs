use glob::glob;
use serde::{Deserialize, Serialize};

use super::project_dependency::Dependency;
use super::static_actual::StaticActual;

use std::path::PathBuf;

/// This "real actual" struct is what Hydrogen actually uses
/// for building and dependency walking.
#[derive(Debug, Serialize, Deserialize)]
pub struct RealActual {
    pub files: Option<Vec<PathBuf>>,
    pub dependencies: Option<Vec<Dependency>>,
}

impl RealActual {
    pub fn new() -> Self {
        Self {
            files: None,
            dependencies: None,
        }
    }

    /// # NOT IMPLEMENTED
    /// Constructs a RealActual from a DynamicActual.
    /// Consumes the dynamic actual.
    #[allow(dead_code)]
    pub fn apply_dynamic(&mut self) {
        panic!("NOT IMPLEMENTED: apply_dynamic: RealActual");
    }

    /// Constructs a RealActual from a StaticActual.
    /// Consumes the static actual.
    pub fn from_static(&mut self, sa: StaticActual) -> Result<(), Box<dyn std::error::Error>> {
        /* if sa.dependencies.is_some() {
            (*self).dependencies = sa.dependencies;
        } */

        if let Some(ref patterns) = sa.files {
            let mut files: Vec<PathBuf> = Vec::new();
            for pattern in patterns {
                for file in glob(pattern)? {
                    if let Ok(ref path) = file {
                        files.push(path.canonicalize().unwrap());
                    }
                }
            }

            (*self).files = Some(files);
        }

        if sa.dependencies.is_some() {
            (*self).dependencies = Some(sa.dependencies.unwrap());
        }

        Ok(())
    }
}
