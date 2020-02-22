//! Provides the implementation of the Rust Logger for the application
//! which sends logs to the UI via the messaging module's mechanisms.
use crate::messaging::outgoing::oui_logger::OUiLogger;
use crate::messaging::OMsg;
use crate::messaging::Sender;
use log::SetLoggerError;
use log::{Level, Log, Metadata, Record};

/// Stores state for Rust Log implementation which communicates with the UI
/// Log component
pub struct UiLogger {
    min_level: Level,
    to_thread_sender: Sender<OMsg>,
}

fn cs(input: Option<&str>) -> Option<String> {
    match input {
        Some(s) => Some(s.to_string()),
        None => None,
    }
}

impl Log for UiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.min_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = record.level();
            let target = record.target().to_string();
            let file = cs(record.file());
            let line = record.line();

            let msg = format!("{}", record.args());
            self.to_thread_sender
                .send(OMsg::UiLogger(OUiLogger::SendLog {
                    level,
                    target,
                    file,
                    line,
                    msg,
                }))
                .expect("unable to send log data");
        }
    }

    fn flush(&self) {}
}

impl UiLogger {
    /// New up a UiLogger instance, given a channel sender from the `messaging`
    /// submodule.
    pub fn new(to_thread_sender: Sender<OMsg>) -> Self {
        Self {
            min_level: Level::Debug,
            to_thread_sender,
        }
    }

    /// Set the minimum log level to report logs for, as an instance of `Level`.
    pub fn set_log_level(&mut self, level: Level) {
        self.min_level = level;
    }
}

/// Initialize the logger for the application, given a sender and a default level. This method should
/// be called once, after the QApplication has been instantiated.
pub fn init(to_thread_sender: Sender<OMsg>, default_level: &str) -> Result<(), SetLoggerError> {
    let mut logger = UiLogger::new(to_thread_sender);
    let level = match default_level {
        "trace" => Level::Trace,
        "debug" => Level::Debug,
        "info" => Level::Info,
        "warn" => Level::Warn,
        "error" => Level::Error,
        _ => Level::Warn,
    };
    logger.set_log_level(level);
    log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(level.to_level_filter()))
}
