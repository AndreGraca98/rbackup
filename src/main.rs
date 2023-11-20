mod logger;
mod parser;

use clap::Parser;
use log::{debug, error, info, trace, warn};

fn main() {
    let args = parser::Cfg::parse();
    println!("{:?}", args);

    let logfile: &str = args.logfile.as_deref().unwrap_or_default();
    let loglevel: &str = args.loglevel.as_str();
    let _ = logger::setup_logger(loglevel, logfile);

    // debug!("Debugging..");
    // info!("Hello, world!");
    // warn!("Warning!");
    // error!("Error!");
    // trace!("Trace!");
}
