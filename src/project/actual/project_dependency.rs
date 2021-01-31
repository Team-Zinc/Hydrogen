use serde::{Deserialize, Serialize};

use crate::project::Project;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    #[serde(default)]
    pub at: String,
    #[serde(flatten)]
    pub external: Option<ExternalDependency>,
    pub project: Option<Project>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalDependency {
    pub version: String,
}

#[allow(dead_code)]
impl Dependency {
    pub fn new() -> Self {
        Self {
            name: "".to_owned(),
            at: "".to_owned(),
            external: None,
            project: None,
        }
    }

    /// Gets the output file name, but
    /// not the output file path.
    pub fn get_output_filename(&self) -> String {
        if let Some(ref project) = self.project {
            if let Some(ref meta) = project.meta {
                return [
                    self.name.as_str(),
                    match meta.element.type_of {
                        crate::project::kinds::ProjectType::Executable => {
                            std::env::consts::EXE_EXTENSION.to_string()
                        }
                        crate::project::kinds::ProjectType::StaticLibrary => {
                            crate::util::extension::get_static_lib_extension()
                        }
                        crate::project::kinds::ProjectType::DynamicLibrary => {
                            std::env::consts::DLL_EXTENSION.to_string()
                        }
                        crate::project::kinds::ProjectType::Super => "none".to_string(),
                    }
                    .as_str(),
                ]
                .join("");
            }
        }

        "".to_owned()
    }

    /* /// Gets the output file name, but
    /// not the output file path, and
    /// without the extension or pre-pending
    /// lib.
    pub fn get_output_id(&self) -> String {
        self.name.as_str().to_string().replace("lib", "")
    } */
}
