use pbgui_logger::log_win::LogWin;
//use qt_core::QResource;
use qt_widgets::{QApplication, QFrame, QMainWindow};
use rustqt_utils::create_vlayout;

fn main() {
    QApplication::init(|_app| unsafe {
        let mut main_window = QMainWindow::new_0a();
        let mut main_widget = QFrame::new_0a();
        let main_widget_ptr = main_widget.as_mut_ptr();

        // main_layout
        let main_layout = create_vlayout();
        //let  main_layout_ptr = main_layout.as_mut_ptr();
        main_widget.set_layout(main_layout.into_ptr());
        // set main_widget as the central widget in main_window
        main_window.set_central_widget(main_widget.into_ptr());

        let mylogger = LogWin::new(main_widget_ptr);
        mylogger.inner().debug("this is a test");
        mylogger
            .inner()
            .info("adding information on the info level");
        mylogger.inner().warn("you really shouldnt see this");
        mylogger
            .inner()
            .error("and you absolutely shouldnt see this");

        main_window.show();
        QApplication::exec()
    });
}
