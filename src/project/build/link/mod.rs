pub mod base;
pub mod dylib;
pub mod exe;
pub mod stlib;

use log::debug;
use std::path::PathBuf;

use super::super::kinds::ProjectType;
<<<<<<< HEAD
<<<<<<< HEAD

=======
>>>>>>> 01f6c2494ac5bd90b98f5797f796ad31592f4cc5
=======
>>>>>>> 01f6c2494ac5bd90b98f5797f796ad31592f4cc5
use super::{ConfigurePool, Project};

use command_run::Command;

pub fn link_objs(
    p: &Project,
    pool: &ConfigurePool,
    objs: &Vec<PathBuf>,
) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
    let mut ld: &PathBuf = &PathBuf::new();
    if let Some(ref meta) = p.meta {
        ld = match meta.element.type_of {
            ProjectType::Executable => &pool.executables.get("ld").unwrap(),
            ProjectType::StaticLibrary => &pool.executables.get("ar").unwrap(),
            ProjectType::DynamicLibrary => &pool.executables.get("todo").unwrap(),
            ProjectType::Super => return Ok(None),
        };
    }

    let mut command = Command::new(ld);
    let mut output_file: String = "".into();

    if let Some(ref meta) = p.meta {
        command.add_args((meta.element.type_of.get_type_link_flags().unwrap())(p));

        output_file = (meta.element.type_of.get_type_output_file().unwrap())(p);

        command.add_args((meta.element.type_of.get_type_output_flags().unwrap())(
            output_file.as_str(),
        ));

        command.add_args(&mut base::get_base_link_flags(p));
        for obj in objs {
            command.add_arg(obj.clone().into_os_string().into_string().unwrap());
        }

        debug!(
            "Running command:\n`{} {:?}`",
            command.program.display(),
            command.args
        );

        command.log_command = false;
        let _out = command.run()?;
    }

    Ok(Some(PathBuf::from(output_file)))
}
