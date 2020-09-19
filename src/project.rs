pub mod version;
pub mod project;
pub mod project_error;
pub mod check;
pub mod dependency;

use project::Project;
use project_error::ProjectError;
use rayon::prelude::*;
use serde::{Deserialize};
use std::{fs, path, env};
use std::string::ToString;
use log::*;

#[derive(Deserialize, Debug)]
pub enum Build {
    Hydrogen,
    CMake,
}

#[derive(Deserialize, Debug)]
pub enum Kind {
    Container,
    #[serde(rename="console")]
    Console,
    #[serde(rename="static_lib")]
    StaticLib,
    #[serde(rename="dynamic_lib")]
    DynamicLib,
}

#[derive(Deserialize, Debug)]
pub enum Language {
    Container,
    C,
    #[serde(rename="C++")]
    Cpp,
}

/// Get matches in the current directory for hy.yml files.
pub fn get_matches() -> Vec<String> {
    let acceptable_names = vec!["hy.yml"];
    let paths = fs::read_dir(&path::Path::new(&env::current_dir().unwrap())).unwrap();

    debug!("Acceptable names: {:?}", acceptable_names);

    trace!("Iterating over all files in current directory into Vector");
    let files = paths.filter_map(|entry| {
        entry.ok().and_then( |e|
            e.path().file_name()
            .and_then(|n| n.to_str().map(|s| String::from(s)))
        )
    }).collect::<Vec<String>>();

    debug!("Files in directory: {:?}", files);

    trace!("Finding acceptable names in list of files concurrently");
    let c = files.into_iter().filter(|s| {
        acceptable_names.par_iter().any(|item| s.contains(item))
    }).collect();
    debug!("Matches: {:?}", c);

    c
}

pub fn find_hy_in_matches(matches: Vec<String>) -> Result<String, ProjectError> {
    trace!("Was a hy.yml matche found?");
    if matches.len() == 0 {
        return Err(ProjectError::FindError{
            reason: Box::from("no matches found in current directory".to_owned()),
        })  
    }

    Ok(matches[0].clone())
}

pub fn find_and_parse<'a>() -> Result<Project, ProjectError> {
    let mut root_project: Project = Project::new();
    let root_hy: String;

    trace!("Finding hy.yml....");
    root_hy = find_hy_in_matches(get_matches())?.to_string();

    debug!("Parsing workspace config file {}....", &root_hy);
    root_project = root_project.deserialize_hy(&root_hy)?;

    Ok(root_project)
}
