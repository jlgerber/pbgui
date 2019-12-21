//! Adds switchable context controls at the same
//! horizontal level as the buttons to switch
//! the bottom stacked widget
//! --------------------------------------------
//!                        -------------------- |
//! page1 page2            | Context Controls | |
//! -----                  -------------------- |
//! ------------------------------------------- |
//! |                                           |
//!

use crate::utility::qs;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    QHBoxLayout, QStackedWidget, QWidget,
};

/// Create a stacked widget housing context
/// specific controls (buttons etc) that sits
/// on the right hand side of the window, next to
/// the context switching buttons that drive the
/// main bottom stacked widget
pub fn create(
    parent_layout: &mut MutPtr<QHBoxLayout>,
    children: Vec<CppBox<QWidget>>,
) -> MutPtr<QStackedWidget> {
    unsafe {
        // create stacked widget, add it to the parent layout
        let mut stacked = QStackedWidget::new_0a();
        let mut stacked_ptr = stacked.as_mut_ptr();
        parent_layout.add_widget(stacked.into_ptr());
        stacked_ptr.set_object_name(&qs("ContextStackedWidget"));
        // iterate through children and add them to the stacked widget
        for child in children.into_iter() {
            stacked_ptr.add_widget(child.into_ptr());
        }

        stacked_ptr
    }
}
