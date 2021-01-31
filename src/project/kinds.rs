use crate::project::build::link::*;
use crate::project::Project;

use serde::{Deserialize, Serialize};

/// Contains the type of the project.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum ProjectType {
    // TODO: Rename to something like ProjectType
    #[serde(alias = "Parent", alias = "Container", alias = "Meta", alias = "None")]
    Super,
    #[serde(alias = "Static", alias = "Library")]
    StaticLibrary,
    #[serde(alias = "Dynamic", alias = "jazz")]
    DynamicLibrary,
    #[serde(alias = "Binary", alias = "dog")]
    Executable,
}

/// Where to download software from.
/// Supports:
///     1. Github
///     2. Git from URL (TODO)
///     3. .tar(.gz) (TODO)
#[derive(Debug, Serialize, Deserialize)]
pub enum Vendor {
    #[serde(alias = "Github")]
    Zip,
}

impl Default for ProjectType {
    fn default() -> Self {
        Self::Super
    }
}

impl ProjectType {
    pub fn get_type_link_flags(&self) -> Option<Box<dyn Fn(&Project) -> Vec<String>>> {
        match self {
            ProjectType::Executable => Some(Box::from(exe::get_link_flags)),
            ProjectType::StaticLibrary => Some(Box::from(stlib::get_link_flags)),
            ProjectType::DynamicLibrary => Some(Box::from(dylib::get_link_flags)),
            ProjectType::Super => None,
        }
    }

    pub fn get_type_output_file(&self) -> Option<Box<dyn Fn(&Project) -> String>> {
        match self {
            ProjectType::Executable => Some(Box::from(exe::get_output_file)),
            ProjectType::StaticLibrary => Some(Box::from(stlib::get_output_file)),
            ProjectType::DynamicLibrary => Some(Box::from(dylib::get_output_file)),
            ProjectType::Super => None,
        }
    }

    pub fn get_type_output_flags(&self) -> Option<Box<dyn Fn(&str) -> Vec<&str>>> {
        match self {
            ProjectType::Executable => Some(Box::from(exe::get_output_flags)),
            ProjectType::StaticLibrary => Some(Box::from(stlib::get_output_flags)),
            ProjectType::DynamicLibrary => Some(Box::from(dylib::get_output_flags)),
            ProjectType::Super => None,
        }
    }
}
