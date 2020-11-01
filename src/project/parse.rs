#[macro_use] use crate::util::out;
use crate::project::project_error::ProjectError;
use snafu::{ResultExt, Snafu};

pub trait Parse {
    fn from_string(&mut self, src: &str) -> Result<(), ProjectError>;
}
