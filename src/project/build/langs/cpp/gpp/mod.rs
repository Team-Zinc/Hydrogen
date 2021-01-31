mod flags;

use crate::project::build::configure::{ConfigureError, ConfigurePool};
use crate::project::build::language::Language;
use crate::project::build::BuildError;

use command_run::Command;
use log::debug;
use std::path::PathBuf;

/*
/// Configures and check environment
/// sanity for a project. Doesn't recurse.
pub fn configure_project(
    p: &Project,
    pool: &mut ConfigurePool,
) -> Result<Option<BuiltJob>, Box<dyn std::error::Error>> {
    base::base_configure(pool)?;

    Ok(None)
}

/// Builds a project. Doesn't recurse.
pub fn build_project(
    p: &Project,
    pool: &mut ConfigurePool,
) -> Result<Option<BuiltJob>, Box<dyn std::error::Error>> {
    Ok(None)
}
*/

/// Builds a single file into an object file.
pub fn build_file(f: &PathBuf, p: &ConfigurePool) -> Result<(), Box<dyn std::error::Error>> {
    let compiler = match p.compilers.get(&Language::Cpp) {
        Some(c) => c,
        None => {
            return Err(Box::from(ConfigureError::UnconfiguredCompilerError {
                lang: Language::Cpp,
            }))
        }
    };

    let mut obj = f.clone();
    let mut command = Command::new(compiler);

    obj.set_extension("o");
    command.add_args(&[f.to_str().unwrap(), "-c", "-o", obj.to_str().unwrap()]);

    debug!(
        "Running command:\n`{} {:?}`",
        command.program.display(),
        command.args
    );

    command.log_command = false;
    let out = command.disable_check().run(); // This should never fail "e.g. exec not found", so we can unwrap.
    if !out.unwrap().status.success() {
        return Err(Box::from(BuildError::CompilerError {}));
    }

    Ok(())
}

/// Configures and modifies the ConfigurePool
/// based on a file.
pub fn configure_file(
    _f: &PathBuf,
    p: &mut ConfigurePool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Do we need to find a compiler?
    if !p.compilers.contains_key(&Language::Cpp) {
        // Yes, yes we do.
        super::probe::probe_cpp_compiler(p)?;
    }

    Ok(())
}
