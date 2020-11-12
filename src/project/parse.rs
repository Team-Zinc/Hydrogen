use crate::project::Project;

use snafu::Snafu;

use std::mem;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ParsingError {
    #[snafu(visibility(pub(crate)))]
    #[snafu(display("Unable to parse configuration from {}: {}", filetype, source))]
    ParseError {
        source: serde_yaml::Error,
        filetype: String
    },
}

pub trait Parse {
    fn from_string(&mut self, src: &str) -> Result<(), ParsingError>;
}

impl Project {
    /// This function simply calls the parse functions
    /// for meta and fetchfile, and (maybe, TODO) runs the dynamic.
    pub fn parse_all(&mut self) -> Result<(), ParsingError> { 
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

    /// Parses all of its children. Not just the
    /// direct ones, but ALL of them.
    pub fn parse_all_children(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.real_actual.is_none() { return Ok(()); }
        let mut real = self.real_actual.take().unwrap();
        
        real.read_children()?;
        real.parse_children()?;

        let mut children = real.dependencies.take().unwrap();

        for child in &mut children {
            child.read_dependency()?;
            child.parse_dependency()?;

            let mut child_project = child.project.take().unwrap();

            if child_project.real_actual.is_some() {
                let mut tmp = child_project.real_actual.take().unwrap();
                let mut tmp2 = tmp.dependencies.take().unwrap();
                println!("{}: {}", child.name, tmp2.len());
                tmp.dependencies.replace(tmp2);
                child_project.real_actual.replace(tmp);
            } else {
                println!("{} doens't have any dependencies", child.name);
            }

            child_project.parse_all_children()?;
            child.project.replace(child_project); 
        }
    
        real.dependencies.replace(children);
        self.real_actual.replace(real);

        Ok(())
    }
}
