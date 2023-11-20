use log4rs::{
    append::{
        console::ConsoleAppender,
        file::FileAppender,
    },
    config::{Appender,Config,Root},
};

pub(crate) fn log_with_level(level: log::LevelFilter) {
    let stdout = ConsoleAppender::builder().build();
    let requests = FileAppender::builder()
        .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new("{d} - {m}{n}")))
        .build("log/requests.log")
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .build(Root::builder().appender("stdout").appender("requests").build(level))
        .unwrap();
    log4rs::init_config(config).unwrap();
}