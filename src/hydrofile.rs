use std::{fs, path, env};

use rayon::prelude::*;
use log::*;

pub fn get_matches() -> Vec<String> {
    let acceptable_names = vec!["Hydrobuild", "hydrobuild", "Hydrobuild.fb", "hydrobuild.fb"];
    let paths = fs::read_dir(&path::Path::new(&env::current_dir().unwrap())).unwrap();

    debug!("acceptable names: {:?}", acceptable_names);

    trace!("iterating over all files in current directory into Vector");
    let files = paths.filter_map(|entry| {
        entry.ok().and_then( |e|
            e.path().file_name()
            .and_then(|n| n.to_str().map(|s| String::from(s)))
        )
    }).collect::<Vec<String>>();

    debug!("files in directory: {:?}", files);

    trace!("finding acceptable names in list of files concurrently");
    let c = files.into_par_iter().filter(|s| {
        acceptable_names.iter().any(|item| s.contains(item))
    }).collect();
    debug!("matches: {:?}", c);

    c
}