#[macro_use] use crate::util::out::*;

pub trait Parse {
    fn from_string(&mut self, src: &str) -> Result<(), serde_yaml::Error> ;
}

pub fn show_parse_error(e: serde_yaml::Error) {
    tell_info!("");
}
