use std::path::PathBuf;
use std::process;
use std::str;

use super::{ConfigureError, ConfigurePool};
use crate::project::actual::project_dependency::Dependency;

#[derive(Debug)]
pub struct FoundLibrary {
    /// Flags used for linking with the library, god dammit!
    pub linking_flags: Option<Vec<String>>,
    /// Flags use for compiling a project that uses the library.
    pub compilation_flags: Option<Vec<String>>,
}

impl FoundLibrary {
    pub fn new() -> Self {
        Self {
            linking_flags: None,
            compilation_flags: None,
        }
    }
}

impl ConfigurePool {
    /// A wrapper around the pkg-config command.
    /// Finds a library, and gets it's flags for
    /// linking and compiling.
    pub fn find_library(&mut self, lib: Dependency) -> Result<(), Box<dyn std::error::Error>> {
        let pkg_config: &PathBuf = match self.executables.get("pkg-config") {
            Some(p) => p,
            None => {
                return Err(Box::from(ConfigureError::UnconfiguredBinaryError {
                    bin: "pkg-config".to_owned(),
                }))
            }
        };

        if let Some(ref external) = lib.external {
            let mut pkg_config_proc = process::Command::new(pkg_config);
            let mut found = FoundLibrary::new();

            // Does it exist?
            pkg_config_proc.args(&["--exists", lib.name.as_str()]);
            let pkg_config_out: process::Output = pkg_config_proc.output()?;
            if !pkg_config_out.status.success() {
                // Doesn't exist
                return Err(Box::from(ConfigureError::LibraryFindError {
                    lib: lib.name,
                }));
            }

            found.compilation_flags = Some(get_compilation_flags(pkg_config, lib.name.as_str())?);

            found.linking_flags = Some(get_linking_flags(pkg_config, lib.name.as_str())?);

            self.libraries.insert(lib.name, found);
        }

        Ok(())
    }
}

fn get_compilation_flags(
    pkg_config: &PathBuf,
    lib: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut pkg_config_proc = process::Command::new(pkg_config);
    pkg_config_proc.args(&["--cflags", lib]);
    let pkg_config_out = pkg_config_proc.output()?;

    let ret: Vec<String> = str::from_utf8(pkg_config_out.stdout[..].as_ref())?
        .to_owned()
        .replace("\n", "")
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    Ok(ret)
}

fn get_linking_flags(
    pkg_config: &PathBuf,
    lib: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut pkg_config_proc = process::Command::new(pkg_config);
    pkg_config_proc.args(&["--libs", lib]);
    let pkg_config_out = pkg_config_proc.output()?;

    let ret: Vec<String> = str::from_utf8(pkg_config_out.stdout[..].as_ref())?
        .to_owned()
        .replace("\n", "")
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    Ok(ret)
}
