use crate::project::Project;

pub fn get_output_flags(o: &str) -> Vec<&str> {
    vec![o]
}

pub fn get_link_flags(p: &Project) -> Vec<String> {
    vec!["-rs".into()]
}

pub fn get_output_file(p: &Project) -> String {
    let extension: &str = &crate::util::extension::get_static_lib_extension();

    [
        "target/",
        std::env::consts::DLL_PREFIX,
        p.get_name().unwrap().as_str(),
        extension,
    ]
    .join("")
}
