mod cli;
mod logging;
mod project;

use log::*;
use project::Project;
use structopt::StructOpt;

fn main() {
    // Init stuff
    // Parse Options
    let opts = cli::Opts::from_args();
    logging::init();

    match opts.get_command() {
        cli::Subcommand::Help{} => {
            eprintln!("Please use the --help flag with not sub-command for help!");
            trace!("Exiting with error code exitcode::NOINPUT ({})", exitcode::NOINPUT);
            std::process::exit(exitcode::NOINPUT);
        },


        cli::Subcommand::Build{} => {
            // Look for root project.
            let mut root: Project = Project::new();
            root.look_for();
            match root.read_all() {
                Err(e) => eprintln!("{}", e),
                Ok(v) => v,
            };

            println!("{:?}", root); // FIXME: REMOVE

            // Parse root

            // Recurse and parse
            // Build
        },
    };
}
