use crate::utility::qs;
use qt_core::{DockWidgetArea, Orientation};
use qt_widgets::{cpp_core::MutPtr, QAction, QDockWidget, QListWidget, QMainWindow};

/// Given the  QMainWindow, create and return the withpackage widget.
pub fn create(window: &mut MutPtr<QMainWindow>) -> (MutPtr<QDockWidget>, MutPtr<QAction>) {
    unsafe {
        let mut withpackage_listwidget = QListWidget::new_0a();
        withpackage_listwidget.set_object_name(&qs("WithsListWidget"));
        let mut dock_widget = QDockWidget::from_q_string(&qs("Withs"));
        let dock_widget_ptr = dock_widget.as_mut_ptr();
        dock_widget.set_widget(withpackage_listwidget.into_ptr());
        let dwaction = dock_widget.toggle_view_action();
        window.add_dock_widget_3a(
            DockWidgetArea::RightDockWidgetArea,
            dock_widget.into_ptr(),
            Orientation::Vertical,
        );
        (dock_widget_ptr, dwaction)
    }
}
