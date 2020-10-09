mod cli;
mod logging;
mod project;
mod policy;

use project::Project;
use project::hy::Hy;
use structopt::StructOpt;
use console::style;
use log::*;

fn main() {
    let opts = cli::Opts::from_args();
    let mut root_project: Project = Project::new_root();

    logging::init();

    match opts.get_command() {
        cli::Subcommand::Help{} => {
            eprintln!("Please use the --help flag with not sub-command for help!");
            trace!("Exiting with error code exitcode::NOINPUT ({})", exitcode::NOINPUT);
            std::process::exit(exitcode::NOINPUT);
        },

        cli::Subcommand::Build{} => {
            println!("{} Preforming configuration on root....",
                style("[***]").bold().dim()
            );

            // We must guarantee a hy.yml file in the current directory.
            if project::find_root().is_none() {
                println!("{} Failed to find a hy.yml configuration file in the current directory!",
                    style("[!!!]").bold().red()
                );

                std::process::exit(1);
            }

            // Now we just parse.
            // We know there is a hy.yml, so we parse it.
            println!("{} Parsing root....",
                style("[***]").bold().dim()
            );

            root_project.hy = match Hy::parse_from_file("hy.yml".to_owned()) {
                Ok(h) => h,
                Err(e) => {
                    println!("{} Failed to parse root: a parse error was encountered!\n{} {}",
                        style("[!!!]").bold().red(),
                        style("[!!!]").bold().red(), e
                    );

                    std::process::exit(-1);
                }
            };

            // Now we look for wanted dependencies, find their dependencies, and so on.
            println!("{:?}", root_project.walk_dependencies());

            // Here we simply find the hy.yml file and parse it.
            // This doesn't recurse between dependencies. That comes later.
            /* root_project = match project::find_and_parse() {
                Ok(p) => p,
                Err(e) => {
                    println!("{} Failed to preform configuration: a parse error was encountered!\n{} {}",
                        style("[!!!]").bold().red(),
                        style("[!!!]").bold().red(), e
                    );

                    log::trace!("Exiting with code exitcode::OK ({})", exitcode::OK);
                    std::process::exit(-1);
                }
            };

            println!("{} Checking Configuration....",
                style("[***]").bold().dim()
            ); */

            // Check the hy.yml file and output the failures.
            // This function checks a lot of things for us, but it
            // doesn't check things like referenced local hy.yml
            // project existence.
        },
    };

    // Exit successfully
    log::trace!("Exiting with code exitcode::OK ({})", exitcode::OK);
    std::process::exit(0);
}
