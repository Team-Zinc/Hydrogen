use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum ProjectError {
    #[snafu(display("Hy.yml file at `{}` found to have invalid syntax (parse error): {}", path, source))]
    ParsingError {
        source: serde_yaml::Error,
        path: Box<String>,
    },

    #[snafu(display("Hy.yml file at `{}` was not readable: {}", path, source))]
    ReadingError {
        source: std::io::Error,
        path: Box<String>,
    },

    #[snafu(display("Couldn't find hy.yml file: {}", reason))]
    FindError {
        reason: Box<String>
    }
}