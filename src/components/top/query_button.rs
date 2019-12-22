use crate::utility::{create_hlayout, qs};
use qt_widgets::{cpp_core::MutPtr, qt_core::QString, QFrame, QHBoxLayout, QPushButton};

//
// Create Query Button
//
pub fn create(hlayout_ptr: &mut MutPtr<QHBoxLayout>) -> MutPtr<QPushButton> {
    unsafe {
        let mut button = QPushButton::from_q_string(&QString::from_std_str("")); //Query
        button.set_object_name(&QString::from_std_str("QueryButton"));
        let button_ptr = button.as_mut_ptr();
        button.set_minimum_width(40);
        button.set_maximum_width(40);
        button.set_minimum_height(40);
        let mut widget = QFrame::new_0a();
        widget.set_object_name(&qs("ButtonFrame"));
        let mut widget_layout = create_hlayout();
        widget_layout.add_widget(button.into_ptr());
        widget.set_layout(widget_layout.into_ptr());
        hlayout_ptr.add_widget(widget.into_ptr());
        button_ptr
    }
}
