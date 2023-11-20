use clap::Parser;

/// Simple program to make backups
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cfg {
    /// Source folder or file
    #[arg(index = 1)]
    pub source: String,

    /// Destination folder or file
    #[arg(index = 2)]
    pub destination: String,

    /// Log file. If not specified, log to stdout
    #[arg(short = 'f', long, default_value = ".rbackup.log")]
    pub logfile: Option<String>,

    /// Log level. Defaults to info
    #[arg(short = 'l', long, default_value = "info")]
    pub loglevel: String,
}
