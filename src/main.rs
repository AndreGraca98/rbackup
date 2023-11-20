mod logger;
mod parser;
mod paths;
mod sync;

use std::path::Path;

use clap::Parser;
use log::{debug, error, info, trace, warn};

fn main() {
    let args = parser::Cfg::parse();
    println!("{:?}", args);

    let logfile: &str = args.logfile.as_deref().unwrap_or_default();
    let loglevel: &str = args.loglevel.as_str();
    let _ = logger::setup_logger(loglevel, logfile);

    // sync::create_dir("test");
    // sync::create_file("test/test.txt");
    // sync::remove_file("test/test.txt");
    // sync::remove_dir("test");

    // let path = Path::new("test");
    // let syncpath = sync::SyncPath::new(path);
    // error!(
    //     "path {} is_file: {}, is_dir: {}, is_symlink: {}",
    //     syncpath.path, syncpath.is_file, syncpath.is_dir, syncpath.is_symlink
    // );

    // debug!("Debugging..");
    // info!("Hello, world!");
    // warn!("Warning!");
    // error!("Error!");
    // trace!("Trace!");

    // let globed_paths = paths::gather_paths("src/*.rs");
    // for path in globed_paths.unwrap_or_else(|e| panic!("Error: {}", e)) {
    //     error!("Path: {}", path.unwrap().display());
    // }

    let source = args.source.as_str();

    let gloabed_paths = paths::gather_paths(Path::new(source)).borrow().clone();
    for p in gloabed_paths.iter() {
        let found_path = p.display().to_string();

        let old_base_path = Path::new(args.source.as_str());
        let new_base_path = Path::new(args.destination.as_str());
        let original_path = Path::new(found_path.as_str());

        let relative_path = original_path.strip_prefix(old_base_path);
        let new_path = new_base_path.join(relative_path.unwrap());

        // error!("old_base_path: {}", old_base_path.display());
        // error!("new_base_path: {}", new_base_path.display());
        // error!("original_path: {}", original_path.display());
        // error!(
        //     "old_path {} ; new_path: {}",
        //     p.display(),
        //     new_path.display()
        // );

        sync::SyncPath::new(p).copy_to(new_path.to_str().unwrap());
    }
}
