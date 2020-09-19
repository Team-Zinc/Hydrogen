use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl Version {
    pub fn new() -> Self {
        Version {
            major: 0,
            minor: 0,
            patch: 0,
        }
    }
}
