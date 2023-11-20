// use clap::Parser;
// mod logger;

// /// Simple program to make backups 
// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     /// Source folder or file
//     #[arg(short, long)]
//     src: String,

//     /// Destination folder or file
//     #[arg(short, long)]
//     dst: String,
// }

// fn main() {
//     let args = Args::parse();
//     println!("{:?}", args);

//     logger::log_with_level(log::LevelFilter::Info);
// }

use log::{info, error, LevelFilter};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};

fn main() {
    // Create a file to log to
    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} - {m}\n")))
        .build("log.txt")
        .unwrap();

    // Create a logger that logs to both the terminal and the file
    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(Root::builder().appender("file").build(LevelFilter::Info))
        .unwrap();

    // Initialize the logger
    log4rs::init_config(config).unwrap();

    // Log some messages
    info!("This message will be logged to the terminal and the file");
    error!("This message will also be logged to the terminal and the file");
}