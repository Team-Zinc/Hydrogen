use serde::{Serialize, Deserialize};

/// Contains the type of the project.
#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(alias = "Parent", alias = "Container", alias = "Meta")]
    Super,
    #[serde(alias = "Static", alias = "Library")]
    StaticLibrary,
    #[serde(alias = "Dynamic")]
    DynamicLibrary,
    #[serde(alias = "Binary")]
    Executable,
}

/// Contains the MAIN language of the project.
/// E.g. if your project contained mostly C++
/// with a little C, you would chose C++.
#[derive(Debug, Serialize, Deserialize)]
pub enum Language {
    C,
    #[serde(rename = "C++")]
    Cpp,
    None,
}

impl Default for Type {
    fn default() -> Self { Self::Super }
}

impl Default for Language {
    fn default() -> Self { Self::None }
}
