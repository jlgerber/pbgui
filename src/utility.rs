use qt_core::QString;
use qt_widgets::cpp_core::CppBox;

/// Given an input of &str or String, return a boxed QString
pub fn qs<S: AsRef<str>>(input: S) -> CppBox<QString> {
    QString::from_std_str(input.as_ref())
}
