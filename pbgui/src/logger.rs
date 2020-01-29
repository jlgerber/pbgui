use crate::messaging::outgoing::oui_logger::OUiLogger;
use crate::messaging::OMsg;
use crate::messaging::Sender;
use log::{Level, Log, Metadata, Record};
use log::{LevelFilter, SetLoggerError};

pub struct UiLogger {
    min_level: Level,
    to_thread_sender: Sender<OMsg>,
}

impl Log for UiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.min_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut level = Some(record.metadata().level());
            for x in format!("{}", record.args()).split("\n") {
                self.to_thread_sender
                    .send(OMsg::UiLogger(OUiLogger::SendLog(level, x.to_string())))
                    .expect("unable to send log data");
                level = None;
            }
        }
    }

    fn flush(&self) {}
}

impl UiLogger {
    pub fn new(to_thread_sender: Sender<OMsg>) -> Self {
        Self {
            min_level: Level::Debug,
            to_thread_sender,
        }
    }

    pub fn set_log_level(&mut self, level: Level) {
        self.min_level = level;
    }
}

pub fn init(to_thread_sender: Sender<OMsg>) -> Result<(), SetLoggerError> {
    let logger = UiLogger::new(to_thread_sender);
    log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(LevelFilter::Info))
}
