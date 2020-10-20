use snafu::Snafu;
use std::io;

/// Describes errors that may happen during the
/// building, parsing, and configuration of
/// projects and their files.
#[derive(Debug, Snafu)]
pub enum ProjectError {
    /// Failed to read a project file.
    #[snafu(display("Failed to read project file at {}: {}", path, source))]
    FileRead { source: io::Error, path: String },

    /// Failed to parse a project file.
    /// TODO: Maybe better serde_yaml::Error formatting?
    #[snafu(display("Failed to parse project file at {}: {}"))]
    FileParse { source: serde_yaml::Error, path: String }
}
