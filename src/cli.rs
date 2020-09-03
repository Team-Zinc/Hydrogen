use structopt::StructOpt;

/*
 * https://stackoverflow.com/questions/50673567/how-to-use-an-enum-that-represents-subcommands-with-structopt
 * https://github.com/TeXitoi/structopt/blob/master/examples/git.rs
 */

#[derive(StructOpt, Debug)]
#[structopt(name = "hy")]
pub struct Opts {
    #[structopt(help = "Turn on verbose mode. Multiple allowed", short = "v", long = "verbose", parse(from_occurrences))]
    pub verbosity: u8,

    #[structopt(help = "Set a working directory", short = "w", long = "working-directory", default_value = ".")]
    pub working_dir: String,

    #[structopt(subcommand)]
    commands: Option<Hydrogen>,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "hy")]
enum Hydrogen {
    #[structopt(about = "Configure the application", aliases = &["Config", "Conf"])]
    Configure {

    },

    #[structopt(about = "Build the application", aliases = &["Compile", "Make"])]
    Build {

    },

    #[structopt(about = "Fetch all the dependencies for the application", aliases = &["Getall", "Getit"])]
    Fetch {

    },

    #[structopt(about = "Check if the application is sane", aliases = &["Sanity", "Ward"])]
    Check {

    },

    #[structopt(about = "Scan the application to output a summary", aliases = &["Look", "Summary"])]
    Scan {

    },
}
