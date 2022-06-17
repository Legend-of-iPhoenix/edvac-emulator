use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};

struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.target().starts_with("edvac")
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        println!("{} -- {}", record.level(), record.args(),);
    }

    fn flush(&self) {}
}

static LOGGER: Logger = Logger;

#[cfg(not(debug_assertions))]
const MAX_LEVEL: LevelFilter = LevelFilter::Info;
#[cfg(debug_assertions)]
const MAX_LEVEL: LevelFilter = LevelFilter::Trace;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(MAX_LEVEL))
}
