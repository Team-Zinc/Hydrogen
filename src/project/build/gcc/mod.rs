mod flags;

use crate::project::Project;

/// Builds a project. Doesn't recurse.
pub fn build_project(p: &Project) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(ref meta) = p.meta {
        println!("Configuring {}", meta.element.name);
    }

    Ok(())
}

/// Configures and check environment
/// sanity for a project. Doesn't recurse.
pub fn configure_project(p: &Project) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(ref meta) = p.meta {
        println!("Configuring {}", meta.element.name);
    }
    
    Ok(())
}
