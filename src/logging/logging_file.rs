/// Set up a fern dispatch logger.
pub fn get_dispatch() -> fern::Dispatch {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{date}][{target}][{level}] {message}",
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                target = record.target(),
                level = record.level(),
                message = message,
            ));
        })
        .level(log::LevelFilter::Trace)
        .chain(fern::DateBased::new("hydrogen.", "%Y-%m-%d.log"))
}