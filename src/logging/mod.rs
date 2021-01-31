use log::info;

pub fn init() {
    if std::path::Path::new("hydrogen.build.log").exists() {
        // We don't care if it errs.
        let _ = std::fs::remove_file("hydrogen.build.log");
    }

    let file_config: fern::Dispatch = get_file_config();
    file_config.apply().expect("Failed to create file logger.");

    info!(
        "\n\n\n\t ---- NEW RUN OF HYDROGEN: {} ----",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    );
}

fn get_file_config() -> fern::Dispatch {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "\n[{date}/{file}] -- {level} \n{message}",
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                file = record.file().unwrap(),
                level = record.level(),
                message = message,
            ));
        })
        .level(log::LevelFilter::Trace)
        .chain(fern::DateBased::new("hydrogen.", "build.log"))
}
