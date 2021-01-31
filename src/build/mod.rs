use crate::project;
use crate::project::build::BuildPool;
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
        match root.construct_real_actual() {
            Ok(()) => (),
            Err(e) => {
                tell_failure!("{}", e);
                std::process::exit(1);
            }
        };

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
    let sorted_projects = project::into_layer_sort(root.topo_sort().unwrap()).unwrap();

    tell_info!("Building (and configuring) everything because I'm nice like that....");

    // First, we iterate over all the projects
    let mut hub = BuildPool::new(sorted_projects);
    match hub.go() {
        Ok(()) => (),
        Err(e) => {
            tell_failure!(
                "The build/configure process failed. Check above for hisses! {}",
                e
            );
            std::process::exit(1);
        }
    };

    tell_success!("Purrrr.... Everything should be built! Check above just in case of hisses!");

    // println!("{}", serde_json::to_string_pretty(&root).unwrap()); // FIXME: REMOVE
}
