use std::io::{stderr, Write};

use log::{Log, SetLoggerError};

pub(crate) struct StderrLogger {}

impl StderrLogger {
    pub fn init() -> Result<(), SetLoggerError> {
        static LOGGER: &StderrLogger = &StderrLogger {};
        log::set_logger(LOGGER)
    }
}

impl Log for StderrLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        // filtering is done elsewhere
        true
    }

    fn log(&self, record: &log::Record) {
        let mut stderr = stderr().lock();

        // ignore errors, nothing reasonable we can do in response to them

        let _ = match (record.file(), record.line()) {
            (Some(file), Some(line)) => writeln!(
                stderr,
                "font-info: {}|{file}:{line}: {}",
                record.level(),
                record.args()
            ),
            (_, _) => writeln!(
                stderr,
                "font-info: {}: {}",
                record.level(),
                record.args()
            ),
        };
    }

    fn flush(&self) {
        // ignore errors, nothing reasonable we can do in response to them
        let _ = stderr().flush();
    }
}
