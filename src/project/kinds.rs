use serde::{Serialize, Deserialize};

/// Contains the type of the project.
#[derive(Debug, Serialize, Deserialize)]
pub enum Type { // TODO: Rename to something like ProjectType 
    #[serde(alias = "Parent", alias = "Container", alias = "Meta", alias = "None")]
    Super,
    #[serde(alias = "Static", alias = "Library")]
    StaticLibrary,
    #[serde(alias = "Dynamic", alias = "jazz")]
    DynamicLibrary,
    #[serde(alias = "Binary", alias = "dog")]
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

/// Where to download software from.
/// Supports:
///     1. Github
///     2. Git from URL (TODO)
///     3. .tar(.gz) (TODO)
#[derive(Debug, Serialize, Deserialize)]
pub enum Vendor {
    #[serde(alias = "Github")]
    GitHub
}

impl Default for Type {
    fn default() -> Self { Self::Super }
}

impl Default for Language {
    fn default() -> Self { Self::None }
}
