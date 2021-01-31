use crate::project::Project;

pub fn get_output_flags(o: &str) -> Vec<&str> {
    vec![o]
}

pub fn get_link_flags(_p: &Project) -> Vec<String> {
    vec!["-shared".into()]
}

pub fn get_output_file(p: &Project) -> String {
    [
        "target/",
        std::env::consts::DLL_PREFIX,
        p.get_name().unwrap().as_str(),
        std::env::consts::DLL_SUFFIX,
    ]
    .join("")
}
