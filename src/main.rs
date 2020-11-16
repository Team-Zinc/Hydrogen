#[macro_use]
mod util;
mod actual;
mod cats;
mod cli;
mod meta;
mod build;
mod project; // I challenge you to explore.....

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
            build::build_subcommand();
        }

        cli::Subcommand::Catz {} => {
            let cat: Vec<_> = cats::CATS
                .choose_multiple(&mut rand::thread_rng(), 1)
                .collect();
            println!("\n{}", cat.get(0).unwrap());
        }
    };
}
