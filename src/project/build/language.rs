<<<<<<< HEAD
<<<<<<< HEAD
use super::configure::{ConfigurePool};
=======
use super::configure::ConfigurePool;
>>>>>>> 01f6c2494ac5bd90b98f5797f796ad31592f4cc5
=======
use super::configure::ConfigurePool;
>>>>>>> 01f6c2494ac5bd90b98f5797f796ad31592f4cc5
use super::langs::c::gcc;
use super::langs::cpp::gpp;
use std::path::PathBuf;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Language {
    C,
    Cpp,
}

impl Language {
    pub fn from_file(f: &PathBuf) -> Option<Self> {
        let ex = f.extension();
        if ex.is_none() {
            return None;
        }

        match ex.unwrap().to_str().unwrap().to_lowercase().as_str() {
            "cpp" => return Some(Self::Cpp),
            "cxx" => return Some(Self::Cpp),
            "hpp" => return Some(Self::Cpp),
            "hxx" => return Some(Self::Cpp),
            "c" => return Some(Self::C),
            "h" => return Some(Self::C),
            _ => return None,
        };
    }

    pub fn get_builder(
        &self,
    ) -> Box<dyn Fn(&PathBuf, &ConfigurePool) -> Result<(), Box<dyn std::error::Error>>> {
        match self {
            Self::C => Box::new(gcc::build_file),
            Self::Cpp => Box::new(gpp::build_file),
        }
    }

    pub fn get_configurer(
        &self,
    ) -> Box<dyn Fn(&PathBuf, &mut ConfigurePool) -> Result<(), Box<dyn std::error::Error>>> {
        match self {
            Self::C => Box::new(gcc::configure_file),
            Self::Cpp => Box::new(gpp::configure_file),
        }
    }

    pub fn get_backend_name(&self) -> String {
        match self {
            Self::C => "gcc".into(),
            Self::Cpp => "g++".into(),
        }
    }
}
