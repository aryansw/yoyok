use colored::Colorize;
use log::{Level, Metadata, Record};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match record.metadata().level() {
                Level::Error => print!("{}", "[ERROR] ".bright_red()),
                Level::Warn => print!("{}", "[WARN] ".bright_yellow()),
                Level::Info => print!("{}", "[INFO] ".bright_green()),
                _ => (),
            };
            println!("{}", record.args());
        }
    }

    fn flush(&self) {}
}

use log::{LevelFilter, SetLoggerError};

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init(v: bool) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| {
        if v {
            log::set_max_level(LevelFilter::Trace)
        } else {
            log::set_max_level(LevelFilter::Info)
        }
    })
}
