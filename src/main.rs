mod cli;
mod logging;
mod project;
mod policy;

use project::project::Project;
use project::check;

use structopt::StructOpt;
use console::style;
use log::*;

fn main() {
    let opts = cli::Opts::from_args();
    let mut root_project: Project = Project::new();

    logging::init();

    match opts.get_command() {
        cli::Subcommand::Help{} => {
            eprintln!("Please use the --help flag with not sub-command for help!");
            trace!("Exiting with error code exitcode::NOINPUT ({})", exitcode::NOINPUT);
            std::process::exit(exitcode::NOINPUT);
        },

        cli::Subcommand::Build{} => {
            println!("{} Preforming Configuration....",
                style("[***]").bold().dim()
            );

            root_project = match project::find_and_parse() {
                Ok(p) => p,
                Err(e) => {
                    println!("{} Failed to preform configuration: a parse error was encountered!",
                        style("[!!!]").bold().red()
                    );

                    println!("{} {}",
                        style("[!!!]").bold().red(), e
                    );

                    log::trace!("Exiting with code exitcode::OK ({})", exitcode::OK);
                    std::process::exit(-1);
                }
            };

            println!("{} Checking Configuration....",
                style("[***]").bold().dim()
            );
            check::user_out(check::check_hy(&root_project));
        },
    }

    // Exit successfully
    log::trace!("Exiting with code exitcode::OK ({})", exitcode::OK);
    std::process::exit(0);
}
