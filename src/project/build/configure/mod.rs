pub mod binary;
pub mod compiler;
pub mod library;

use snafu::Snafu;

use super::configure::library::FoundLibrary;
use super::language::Language;

use crate::project::kinds::ProjectType;

use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Snafu)]
pub enum ConfigureError {
    #[snafu(visibility(pub(crate)))]
    #[snafu(display("Unable to find compiler for {:?}.", lang))]
    CompilerProbeError { lang: Language },

    #[snafu(visibility(pub(crate)))]
    #[snafu(display("Compiler for {:?} isn't configured.", lang))]
    UnconfiguredCompilerError { lang: Language },

    #[snafu(visibility(pub(crate)))]
    #[snafu(display("Unable to find binary {}.", bin))]
    BinaryFindError { bin: String },

    #[snafu(visibility(pub(crate)))]
    #[snafu(display("Unable to run binary {} due to not being configured.", bin))]
    UnconfiguredBinaryError { bin: String },

    #[snafu(visibility(pub(crate)))]
    #[snafu(display("Unable to find library {}.", lib))]
    LibraryFindError { lib: String },

    #[snafu(visibility(pub(crate)))]
    #[snafu(display("Unable to use library {} due to not being configured.", lib))]
    UnconfiguredLibraryError { lib: String },
}

/// Contains information about the
/// output of a build, like library
/// location and compile flags.
/// This applies for vendored builds
/// only.
#[derive(Debug)]
pub struct BuildContext {
    pub found_library: Option<library::FoundLibrary>,
    pub type_of: ProjectType,
    pub out: PathBuf,
}

/// This applies to non-vendored builds only.
pub struct ConfigurePool {
    /// Keys are languages, and values
    /// are the locations of the compiler.
    pub compilers: HashMap<Language, PathBuf>,

    /// A list of executables, and
    /// where they are.
    pub executables: HashMap<String, PathBuf>,

    /// A list of libraries, and
    /// where they are.
    pub libraries: HashMap<String, library::FoundLibrary>,
}

impl BuildContext {
    pub fn from_static_lib(out: PathBuf) -> BuildContext {
        BuildContext {
            found_library: Some(FoundLibrary {
                compilation_flags: None,
                linking_flags: None,
            }),
            type_of: ProjectType::StaticLibrary,
            out,
        }
    }

    pub fn from_dynamic_lib(out: PathBuf) -> BuildContext {
        BuildContext {
            found_library: Some(FoundLibrary {
                compilation_flags: None,
                linking_flags: None,
            }),
            type_of: ProjectType::DynamicLibrary,
            out,
        }
    }
}

impl ConfigurePool {
    pub fn new() -> Self {
        Self {
            compilers: HashMap::new(),
            executables: HashMap::new(),
            libraries: HashMap::new(),
        }
    }
}
