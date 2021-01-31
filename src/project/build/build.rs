use super::{link, project, BuildError, BuildPool, BuiltJob, ConfigurePool};
use crate::project::build::configure::BuildContext;
use crate::project::build::language::Language;
use project::Project;

use log::{debug, error, info, trace};
use snafu::ResultExt;
use std::env;
use std::path::{Path, PathBuf};

impl<'a> BuildPool<'a> {
    pub fn go(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        target()?; // Make sure target exists, and if not, create it.

        let neededs = vec!["pkg-config", "ld", "ar"];
        for needed in neededs {
            self.configure_pool.find_binary(needed.to_owned())?;
        }

        println!("");
        trace!("{}", serde_json::to_string_pretty(&self.projects).unwrap());

        for projects in &self.projects {
            for project in projects {
                // Don't build the project that don't have a type.
                if let Some(ref meta) = project.meta {
                    match meta.element.type_of {
                        project::kinds::ProjectType::Super => continue,
                        _ => (),
                    };
                }

                let mut job = Box::from(BuiltJob::new(project));
                let start = std::time::Instant::now();

                tell_info!("Building project {}.", project.to_string());
                info!("Building project {}.", project.to_string());

                if let Some(ref _real) = project.real_actual {
                    let out = build_and_link_files(project, &mut self.configure_pool, &mut job);
                    if out.is_ok() {
                        job.context = out.unwrap();
                    }
                }

                job.time = start.elapsed().as_millis();

                if job.build_result.is_err() || job.link_result.is_err() {
                    tell_error!("{}", job.get_failure_description());
                    tell_failure!("Stopping due to above error(s)....");

                    error!("Failed to build project: {}", job.get_failure_description());
                    break;
                }

                tell_success!("{}\n", &job.get_success_description());
                info!(
                    "Project built successfully: {}\n\n",
                    job.get_success_description()
                );

                self.built.push(job);
            }
        }

        Ok(())
    }
}

fn target() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new("target").exists() {
        std::fs::create_dir("target").context(super::TargetDirError)?;
    }

    Ok(())
}

fn build_and_link_files<'a>(
    p: &Project,
    c: &mut ConfigurePool,
    j: &mut BuiltJob,
) -> Result<Option<BuildContext>, Box<dyn std::error::Error>> {
    let mut objs: Vec<PathBuf> = Vec::new();
    let mut context: Option<BuildContext> = None;

    if let Some(ref files) = p.get_files() {
        for file in files {
            let build;

            tell_info!("Building {}", file.display());
            info!("Compiling {} into object.", file.display());
            build = build_file(&file, c);

            if build.is_err() {
                j.build_result = Err(build.err().unwrap());
                return Err(Box::from(BuildError::CompilerError {}));
            }

            objs.push(build.ok().unwrap());
        }
    }

    if let Some(ref meta) = p.meta {
        let link;
        let out;

        tell_info!("Linking {}", meta.element.name);
        info!("Linking {} into final.", meta.element.name);
        link = link::link_objs(p, c, &objs);

        if link.is_err() {
            j.link_result = Err(link.err().unwrap());
            return Err(Box::from(BuildError::LinkerError {}));
        }

        out = link.unwrap().unwrap();
        match meta.element.type_of {
            project::kinds::ProjectType::Executable => {}
            project::kinds::ProjectType::StaticLibrary => {
                context = Some(BuildContext::from_static_lib(out));
            }
            project::kinds::ProjectType::DynamicLibrary => {
                context = Some(BuildContext::from_dynamic_lib(out));
            }
            project::kinds::ProjectType::Super => {}
        }
    }

    Ok(context)
}

fn build_file(
    file: &PathBuf,
    p: &mut ConfigurePool,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let old = env::current_dir().context(project::GetDirError)?;
    env::set_current_dir(file.parent().unwrap()).context(project::ChangeDirError {
        to: file.parent().unwrap(),
    })?;

    let lang = match Language::from_file(file) {
        Some(l) => l,
        None => {
            return Err(Box::new(super::BuildError::LanguageDiscernError {
                path: file.clone(),
            }))
        }
    };

    debug!(
        "Using the {:?} configuration and compilation backend {}.",
        lang,
        lang.get_backend_name()
    );
    lang.get_configurer()(file, p)?;
    lang.get_builder()(file, p)?;

    let mut obj = file.canonicalize().unwrap();
    obj.set_extension("o");

    env::set_current_dir(&old).context(project::ChangeDirError { to: old })?;

    Ok(obj)
}
