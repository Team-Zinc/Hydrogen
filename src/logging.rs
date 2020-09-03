extern crate log;

mod logging_file;

pub fn init() {
    let file_config: fern::Dispatch = logging_file::get_file_config();
    file_config.apply().expect("Failed to create file logger.");
}
