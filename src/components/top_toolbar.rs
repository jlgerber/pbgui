use crate::utility::qs;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    QHBoxLayout, QMainWindow, QWidget,
};

pub fn create(main_window: &mut MutPtr<QMainWindow>, hlayout: CppBox<QHBoxLayout>) {
    unsafe {
        let mut top_toolbar = main_window.add_tool_bar_q_string(&qs("TopToolBar"));
        top_toolbar.set_floatable(false);
        top_toolbar.set_movable(false);

        let mut toolbar_widget = QWidget::new_0a();
        toolbar_widget.set_object_name(&qs("ToobarWidget"));
        toolbar_widget.set_layout(hlayout.into_ptr());
        top_toolbar.add_widget(toolbar_widget.into_ptr());
    }
}
