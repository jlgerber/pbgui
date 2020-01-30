use crate::messaging::outgoing::oui_logger::OUiLogger;
use crate::messaging::OMsg;
use crate::messaging::Sender;
use log::SetLoggerError;
use log::{Level, Log, Metadata, Record};

#[derive(Debug, PartialEq)]
pub struct LogMsgConfig {
    level: bool,
    datetime: bool,
    target: bool,
    module_path: bool,
    file: bool,
    line: bool,
}

impl Default for LogMsgConfig {
    fn default() -> Self {
        Self {
            level: true,
            datetime: true,
            target: false,
            module_path: false,
            file: false,
            line: false,
        }
    }
}
pub struct UiLogger<'a> {
    min_level: Level,
    to_thread_sender: Sender<OMsg<'a>>,
    msg_config: LogMsgConfig,
}

fn cs(input: Option<&str>) -> Option<String> {
    match input {
        Some(s) => Some(s.to_string()),
        None => None,
    }
}

impl<'a> Log for UiLogger<'a> {
    fn enabled(&self, metadata: &Metadata<'a>) -> bool {
        metadata.level() <= self.min_level
    }

    fn log(&self, record: &Record<'a>) {
        if self.enabled(record.metadata()) {
            let level = record.level();

            // let msg = format!("{}", record.args());
            // self.to_thread_sender
            //     .send(OMsg::UiLogger(OUiLogger::SendLog(record)))
            //     .expect("unable to send log data");
        }
    }

    fn flush(&self) {}
}

impl<'a> UiLogger<'a> {
    pub fn new(to_thread_sender: Sender<OMsg<'a>>) -> Self {
        Self {
            min_level: Level::Debug,
            to_thread_sender,
            msg_config: LogMsgConfig::default(),
        }
    }

    pub fn set_log_level(&mut self, level: Level) {
        self.min_level = level;
    }
}

pub fn init<'a>(
    to_thread_sender: Sender<OMsg<'a>>,
    default_level: &str,
) -> Result<(), SetLoggerError> {
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
    let boxlog = Box::new(logger);
    log::set_boxed_logger(boxlog).map(|()| log::set_max_level(level.to_level_filter()))
}
