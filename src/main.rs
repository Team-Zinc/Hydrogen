#[macro_use]
mod util;
mod build;
mod cats; // I implore you to explore.....
mod cli;
mod project; 
mod logging;

use rand::seq::SliceRandom;
use structopt::StructOpt;
use log::{info, trace};

/// Entry point.
fn main() {
    // Init stuff
    // Logging
    logging::init();

    // Parse Options
    let opts = cli::Opts::from_args();

    match opts.get_command() {
        cli::Subcommand::Help {} => {
            trace!("Help subcommand triggered.");
            tell_error!("Please use the --help flag with not sub-command for help!");
            std::process::exit(exitcode::NOINPUT);
        }

        cli::Subcommand::Build {} => {
            trace!("Build subcommand triggered.");
            build::build_subcommand();
        }

        cli::Subcommand::Catz {} => {
            trace!("Kitten Fever.");
            let cat: Vec<_> = cats::CATS
                .choose_multiple(&mut rand::thread_rng(), 1)
                .collect();
            let cat_ascii = cat.get(0).unwrap();

            info!("{}", cat_ascii);
            println!("\n{}", cat_ascii);
        }
    };
}
