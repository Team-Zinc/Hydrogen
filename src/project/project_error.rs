use snafu::{ResultExt, Snafu};
use std::{fs, io};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ProjectError {
    #[snafu(display("Unable to read configuration from {}: {}", path, source))]
    ReadFile {
        source: Box<io::Error>,
        path: String
    },

    #[snafu(display("Unable to parse configuration from {}: {}", filetype, source))]
    ParseFile {
        #[snafu(source(from(serde_yaml::Error, Box::new)))]
        source: Box<serde_yaml::Error>,
        filetype: String
    },
}
