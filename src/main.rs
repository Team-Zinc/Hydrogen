#[macro_use]
mod util;
mod actual;
mod cats;
mod cli;
mod fetchfile;
mod meta;
mod project; // I challenge you to explore.....

use project::Project;
use rand::seq::SliceRandom;
use structopt::StructOpt;

/// Entry point.
fn main() {
    // Init stuff
    // Parse Options
    let opts = cli::Opts::from_args();

    match opts.get_command() {
        cli::Subcommand::Help {} => {
            tell_error!("Please use the --help flag with not sub-command for help!");
            std::process::exit(exitcode::NOINPUT);
        }

        cli::Subcommand::Build {} => {
            // Look for root project.
            let mut root: Project = Project::new();
            match root.read_all() {
                Ok(()) => (),
                Err(e) => {
                    tell_failure!("{}", e);
                    std::process::exit(1);
                }
            };

            tell_info!("Recursing and parsing over everything just for you!");
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

            tell_info!("Building all of it because I'm nice like that....");
            // Build

            tell_success!("Done! Everything should be built! Check above just in case of hisses.");

            println!("{}", serde_json::to_string_pretty(&root).unwrap()); // FIXME: REMOVE
        }

        cli::Subcommand::Catz {} => {
            let cat: Vec<_> = cats::CATS
                .choose_multiple(&mut rand::thread_rng(), 1)
                .collect();
            println!("\n{}", cat.get(0).unwrap());
        }
    };
}
