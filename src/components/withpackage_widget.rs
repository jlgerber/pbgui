use crate::utility::{create_vlayout, qs};
use qt_widgets::{cpp_core::MutPtr, QFrame, QListWidget, QSplitter};

/// create and return the withpackage list widget, given the parent splitter.
///
/// # Arguments
/// * `splitter` - The  splitter which we will give ownership of the widget to.
///
/// # Returns
/// * A pointer to the Withs List Widget
pub fn create(splitter: &mut MutPtr<QSplitter>) -> MutPtr<QListWidget> {
    unsafe {
        // create the inner withpackage
        let mut withpackage_listwidget = QListWidget::new_0a();
        withpackage_listwidget.set_object_name(&qs("WithsListWidget"));
        // create a pointer to it
        let withpackage_listwidget_ptr = withpackage_listwidget.as_mut_ptr();
        // create an outer frame widget
        let mut frame = QFrame::new_0a();
        // hold a pointer to it
        let mut frame_ptr = frame.as_mut_ptr();
        // transfer ownership to the splitter
        splitter.add_widget(frame.into_ptr());
        let mut layout = create_vlayout();
        layout.add_widget(withpackage_listwidget.into_ptr());
        frame_ptr.set_layout(layout.into_ptr());
        withpackage_listwidget_ptr
    }
}
