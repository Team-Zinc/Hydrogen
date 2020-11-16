use crate::project::Project;

pub fn build_subcommand() {
    tell_info!("Recursing and parsing over everything just for you!");

    // Read all the build files in the root directory.
    let mut root: Project = Project::new();
    match root.read_all() {
        Ok(()) => (),
        Err(e) => {
            tell_failure!("{}", e);
            std::process::exit(1);
        }
    };

    // Parse the base.
    match root.parse_all() {
        Ok(()) => (),
        Err(e) => {
            tell_failure!("{}", e);
            std::process::exit(1);
        }
    };

    // Contruct the base actual from a static actual, if one exists.
    // Please note that this consumes static_actual with take().
    // Also, we should parse it too.
    if root.static_actual.is_some() {
        root.construct_real_actual();
        match root.parse_all_children() {
            Ok(()) => (),
            Err(e) => {
                tell_failure!("{}", e);
                std::process::exit(1);
            }
        };
    }

    // Now we run a simple topographical sort to
    // get everything into the correct order for
    // building and simple iteration.
    let sorted_projects = root.topo_sort().unwrap_or_default();

    tell_info!("Building all of it because I'm nice like that....");
    // First, we iterate over all the projects
    sorted_projects.into_iter().for_each(|p| {
        println!("\n\n\n{:?}", p);
    });

    tell_success!("Done! Everything should be built! Check above just in case of hisses.");

    println!("{}", serde_json::to_string_pretty(&root).unwrap()); // FIXME: REMOVE
}
