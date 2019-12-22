#![windows_subsystem = "windows"]
use env_logger;
use env_logger::Env;
use log;
use packybara::packrat::PackratDb;
use pbgui::{main_window, ClientProxy};
use qt_widgets::QApplication;
use std::env;
use structopt::StructOpt;

#[derive(StructOpt, Debug, PartialEq)]
pub struct PbGui {
    /// Set the log level. This may target one or more
    /// specific modules or be general.
    /// (levels: trace, debug, info, warn, error)
    #[structopt(long)]
    pub loglevel: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = PbGui::from_args();
    if let PbGui {
        loglevel: Some(ref level),
        ..
    } = opt
    {
        env::set_var("RUST_LOG", level);
    }
    env_logger::from_env(Env::default().default_filter_or("warn")).init();

    let client = ClientProxy::connect()?;
    let mut vpin_finder = PackratDb::new(client);
    QApplication::init(|app| unsafe {
        let mut _form = main_window::MainWindow::new(&mut vpin_finder);
        //let available_size = app.desktop().available_geometry().size();
        QApplication::exec()
    });
}
