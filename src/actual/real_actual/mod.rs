use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::actual::project_dependency::Dependency;
use crate::actual::static_actual::StaticActual;

/// This "real actual" struct is what Hydrogen actually uses
/// for building and dependency walking.
#[derive(Debug, Serialize, Deserialize)]
pub struct RealActual {
    pub files: Option<Vec<String>>,
    pub dependencies: Option<Vec<Dependency>>,
}

impl RealActual {
    pub fn new() -> Self {
        Self {
            files: None,
            dependencies: None,
        }
    }

    pub fn read_children(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.dependencies.is_none() {
            return Ok(());
        }
        let mut deps = self.dependencies.take().unwrap();

        for dep in &mut deps {
            dep.read_dependency()?;
        }

        self.dependencies.replace(deps);
        Ok(())
    }

    pub fn parse_children(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.dependencies.is_none() {
            return Ok(());
        }
        let mut deps = self.dependencies.take().unwrap();

        for dep in &mut deps {
            dep.parse_dependency()?;
        }

        self.dependencies.replace(deps);
        Ok(())
    }

    /// # NOT IMPLEMENTED
    /// Constructs a RealActual from a DynamicActual.
    /// Consumes the dynamic actual.
    pub fn apply_dynamic(&mut self) {
        panic!("NOT IMPLEMENTED: apply_dynamic: RealActual");
    }

    /// Constructs a RealActual from a StaticActual.
    /// Consumes the static actual.
    pub fn from_static(&mut self, sa: StaticActual) {
        /* if sa.dependencies.is_some() {
            (*self).dependencies = sa.dependencies;
        } */

        if sa.files.is_some() {
            (*self).files = sa.files;
        }

        if sa.dependencies.is_some() {
            (*self).dependencies = Some(sa.dependencies.unwrap());
        }
    }
}
