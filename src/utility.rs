use qt_core::{q_io_device::OpenModeFlag, QFile, QFlags, QResource, QString, QTextStream};
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    QWidget,
};

/// Given an input of &str or String, return a boxed QString
pub fn qs<S: AsRef<str>>(input: S) -> CppBox<QString> {
    QString::from_std_str(input.as_ref())
}

pub fn load_stylesheet(mut parent_widget: MutPtr<QWidget>) {
    unsafe {
        // Does not work
        //QResource::add_search_path(&QString::from_std_str("/Users/jgerber/bin/"));
        let _result = QResource::register_resource_q_string(&QString::from_std_str(
            "/Users/jgerber/bin/pbgui.rcc",
        ));
        //println!("Loading resource successful?: {}", result);
        let mut file = QFile::from_q_string(&QString::from_std_str("/Users/jgerber/bin/pbgui.qss"));
        if file.open_1a(QFlags::from(OpenModeFlag::ReadOnly)) {
            let mut text_stream = QTextStream::new();
            text_stream.set_device(file.as_mut_ptr());
            let stylesheet = text_stream.read_all();
            parent_widget.set_style_sheet(stylesheet.as_ref());
        } else {
            println!("stylesheet not found");
        }
    }
}
