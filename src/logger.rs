use fern;
use humantime;

use std::time::SystemTime;

pub(crate) fn setup_logger(level: &str, path: &str) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}::{}::{} - {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(level.parse().unwrap_or(log::LevelFilter::Info))
        .chain(std::io::stdout())
        .chain(fern::log_file(path)?)
        .apply()?;
    Ok(())
}
