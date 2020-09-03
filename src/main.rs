mod cli;
mod logging;
mod hydrofile;

use structopt::StructOpt;
use log::*;

fn main() {
    // Init our cli interface
    let _opts = cli::Opts::from_args();

    // Init our logger 
    logging::init();

    // Find Hydrobuild file
    trace!("Finding Hydrobuild....");
    let matches = hydrofile::get_matches();
    if matches.len() == 0 {
        error!("no matches found when looking for Hydrobuild files!");
        eprintln!("Couldn't find Hydrobuild/hydrobuild file! Are you in the right directory?");
        
        trace!("Exiting with error code exitcode::NOINPUT ({})", exitcode::NOINPUT);
        std::process::exit(exitcode::NOINPUT);
    }

    // Exit successfully
    log::trace!("Exiting with error code {}", exitcode::OK);
    std::process::exit(exitcode::OK);
}
