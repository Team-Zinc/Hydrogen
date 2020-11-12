use structopt::StructOpt;
use structopt::clap::AppSettings;

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
    commands: Option<Subcommand>,
}

impl Opts {
    pub fn get_command(&self) -> &Subcommand {
        self.commands.as_ref().unwrap_or(&Subcommand::Help{})
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "hy")]
pub enum Subcommand {
    #[structopt(about = "Get help", aliases = &["halp", "usage"])]
    Help {

    },

    #[structopt(about = "Build the application", aliases = &["compile", "make", "plz"])]
    Build {

    },

    #[structopt(setting = AppSettings::Hidden, aliases = &["cat", "cats"])]
    Catz {
        
    },
}
