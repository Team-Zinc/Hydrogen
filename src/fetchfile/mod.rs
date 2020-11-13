use crate::project::kinds::Vendor;
use crate::project::parse;
use crate::project::parse::{Parse, ParsingError};

use serde::{Deserialize, Serialize};
use snafu::ResultExt;
// The fetchfile (located in fetch.yml) describes
// where and how to download a dependency.

#[derive(Debug, Serialize, Deserialize)]
pub struct Fetchfile {
    vendor: Vendor,
    from: String,
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
    fn from_string(&mut self, src: &str) -> Result<(), ParsingError> {
        *self = serde_yaml::from_str(src).context(parse::ParseError {
            filetype: "fetchfile",
        })?;

        Ok(())
    }
}
