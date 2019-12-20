#![windows_subsystem = "windows"]
use env_logger;
use log;
use packybara::packrat::PackratDb;
use pbgui::{parent_form, ClientProxy};
use qt_widgets::QApplication;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let client = ClientProxy::connect()?;
    let mut vpin_finder = PackratDb::new(client);
    QApplication::init(|_| unsafe {
        let mut _form = parent_form::Form::new(&mut vpin_finder);
        QApplication::exec()
    });
}
