use crate::project::build::configure::ConfigureError;
use crate::project::build::configure::ConfigurePool;
use crate::project::build::language::Language;

pub fn probe_c_compiler(p: &mut ConfigurePool) -> Result<(), ConfigureError> {
    let compilers = ["gcc", "clang"];
    let mut found = false;

    for compiler in compilers.iter() {
        if p.find_compiler_as(compiler.to_string(), Language::C)
            .is_ok()
        {
            found = true;
            break;
        }
    }

    if !found {
        return Err(ConfigureError::CompilerProbeError { lang: Language::C });
    }
    Ok(())
}
