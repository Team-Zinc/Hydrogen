#[macro_use] mod util;
mod cli;
mod logging;
mod project;
mod meta;
mod fetchfile;
mod actual;
mod cats; // I challenge you to explore.....

use log::*;
use project::Project;
use structopt::StructOpt;
use rand::seq::SliceRandom;

/// Entry point.
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

            tell_info!("Parsing everything for your convenience....");
            // Parse root
            match root.parse_all() {
                Ok(()) => (),
                Err(e) => {
                    tell_failure!("{}", e);
                    std::process::exit(1);
                },
            }

            tell_info!("Recursing over all dependencies just for you!");
            // Recurse and parse

            tell_info!("Building all of it because I'm nice like that....");
            // Build

            tell_success!("Done! Everything should be built! Check above just in case.");

            println!("{:?}", root); // FIXME: REMOVE
        },

        cli::Subcommand::Catz{} => {
            let cat: Vec<_> = cats::CATS.choose_multiple(&mut rand::thread_rng(), 1).collect();
            println!("\n{}", cat.get(0).unwrap());
        }
    };
}
