pub fn get_static_lib_extension() -> String {
    if std::env::consts::OS == "windows" {
        ".lib".to_owned()
    } else {
        ".a".to_owned()
    }
}
