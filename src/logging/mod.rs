extern crate log;

mod logging_file;

/// Initialize logging. A wrapper that calls logging_file::get_dispatch()
/// and applies it.
pub fn init() {
    let file_config: fern::Dispatch = logging_file::get_dispatch();
    file_config.apply().expect("Failed to create file logger.");
}
