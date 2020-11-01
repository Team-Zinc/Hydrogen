use crate::project::{parse::Parse, kinds::Vendor};
use crate::project::project_error::{ProjectError};
use crate::project::project_error;

use snafu::{ResultExt};
use serde::{Serialize, Deserialize};
// The fetchfile (located in fetch.yml) describes
// where and how to download a dependency.

#[derive(Debug, Serialize, Deserialize)]
pub struct Fetchfile {
    vendor: Vendor,
    from: String
}

impl Fetchfile {
    pub fn new() -> Self {
        Self {
            vendor: Vendor::GitHub,
            from: "".into(),
        }
    }
}

impl Parse for Fetchfile {
    fn from_string(&mut self, src: &str) -> Result<(), ProjectError> {
        *self = serde_yaml::from_str(src).context(
            project_error::ParseFile {
                filetype: "fetchfile",
            }
        )?;

        Ok(())
    }
}

/* fn from_string(&mut self, src: &str) -> Result<(), ProjectError>; */
