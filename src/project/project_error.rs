use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum ProjectError<'a> {
    #[snafu(display("Hy.yml esc file found to have invalid syntax (parse error): {}", source))]
    ParsingError {
        source: serde_yaml::Error,
    },

    #[snafu(display("{}: {}", reason, source))]
    ReadingError {
        source: &'a std::io::Error,
        reason: Box<String>
    },

    #[snafu(display("Couldn't find hy.yml esc file: {}", reason))]
    FindError {
        reason: Box<String>
    }
}