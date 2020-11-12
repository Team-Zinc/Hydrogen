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
            tell_error!("Please use the --help flag with not sub-command for help!");
            trace!("Exiting with error code exitcode::NOINPUT ({})", exitcode::NOINPUT);
            std::process::exit(exitcode::NOINPUT);
        },


        cli::Subcommand::Build{} => {
            // Look for root project.
            let mut root: Project = Project::new();
            match root.read_all() {
                Ok(()) => (),
                Err(e) => { 
                    tell_failure!("{}", e);
                    std::process::exit(1);
                },
            };

            tell_info!("Recursing and parsing over everything just for you!");
            // Parse the base.
            match root.parse_all() {
                Ok(()) => (),
                Err(e) => { 
                    tell_failure!("{}", e);
                    std::process::exit(1);
                },
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
                    },
                };
            }

            tell_info!("Building all of it because I'm nice like that....");
            // Build

            tell_success!("Done! Everything should be built! Check above just in case of hisses.");

            println!("{}", serde_json::to_string_pretty(&root).unwrap()); // FIXME: REMOVE
        },

        cli::Subcommand::Catz{} => {
            let cat: Vec<_> = cats::CATS.choose_multiple(&mut rand::thread_rng(), 1).collect();
            println!("\n{}", cat.get(0).unwrap());
        }
    };
}
