use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum ProjectError {
    #[snafu(display("Hy.yml esc file found to have invalid syntax (parse error): {}", source))]
    ParsingError {
        source: serde_yaml::Error,
    },

    #[snafu(display("Failed to read hy.yml esc file: {}", source))]
    ReadingError {
        source: std::io::Error,
    },

    #[snafu(display("Couldn't find hy.yml esc file: {}", reason))]
    FindError {
        reason: Box<String>
    }
}