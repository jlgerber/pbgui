use qt_core::QString;
use qt_widgets::cpp_core::MutPtr;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io;

#[derive(Debug, Serialize, Deserialize)]
pub struct LogItem {
    level: String,
    datetime: String,
    target: String,
    file: String,
    line: String,
    msg: String,
}

impl LogItem {
    pub fn new(
        level: &MutPtr<QString>,
        datetime: &MutPtr<QString>,
        target: &MutPtr<QString>,
        file: &MutPtr<QString>,
        line: &MutPtr<QString>,
        msg: String,
    ) -> LogItem {
        LogItem {
            level: level.to_std_string(),
            datetime: datetime.to_std_string(),
            target: target.to_std_string(),
            file: file.to_std_string(),
            line: line.to_std_string(),
            msg,
        }
    }

    ///Append to the msg
    pub fn append_msg(&mut self, msg: &str) {
        self.msg.push_str(msg);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    items: Vec<LogItem>,
}

impl Log {
    /// new up the Log
    pub fn new(items: Vec<LogItem>) -> Self {
        Self { items }
    }

    /// Add a log item
    pub fn push(&mut self, item: LogItem) {
        self.items.push(item);
    }

    /// write log out to disk
    pub fn write(&self, path: &str) -> io::Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        serde_json::to_writer_pretty(file, &self)?;
        Ok(())
    }
}

impl Default for Log {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}
