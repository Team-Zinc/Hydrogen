#[macro_use] use crate::util::out;

pub trait Parse {
    fn from_string(&mut self, src: &str) -> Result<(), serde_yaml::Error> ;
}
