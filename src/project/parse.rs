use super::Project;
use crate::project::project_error::ProjectError;

use std::mem;

pub trait Parse {
    fn from_string(&mut self, src: &str) -> Result<(), ProjectError>;
}

impl Project {
    /// This function simply calls the parse functions
    /// for meta and fetchfile, and (maybe, TODO) runs the dynamic.
    pub fn parse_all(&mut self) -> Result<(), ProjectError> {
        if self.meta.is_some() {
            // Parse the meta source
            let mut e = self.meta.take().unwrap();
            e.element.from_string(
                e.src.as_ref()
            )?;

            mem::swap(&mut self.meta, &mut Some(e));
            // self.meta = Some(e);
        }

        if self.fetchfile.is_some() {
            // Parse the fetch source 
            let mut e = self.fetchfile.take().unwrap();
            e.element.from_string(
                e.src.as_ref()
            )?;

            mem::swap(&mut self.fetchfile, &mut Some(e));
            // self.fetchfile = Some(e);
        }

        if self.static_actual.is_some() {
            // Parse the static actual source
            let mut e = self.static_actual.take().unwrap();
            e.element.from_string(
                e.src.as_ref()
            )?;

            mem::swap(&mut self.static_actual, &mut Some(e));
            // self.static_actual = Some(e);
        }

        Ok(())
    }
}
