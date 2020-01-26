use qt_widgets::{
    cpp_core::{MutPtr, StaticUpcast},
    qt_core::QString,
    QFrame, QLayout, QPushButton,
};
use rustqt_utils::{create_hlayout, qs};

//
// Create Query Button
//
/// Create query button for layout.
///
/// # Arguments
/// * `layout` - A Mutable Pointer to a type that can be upcast to a QLayout
///
/// # Returns
/// * A mutalble pointer wrapping a QPushButton
pub fn create<T>(label: Option<&str>, layout: MutPtr<T>) -> MutPtr<QPushButton>
where
    T: StaticUpcast<QLayout>,
{
    unsafe {
        let mut button = match label {
            Some(label) => QPushButton::from_q_string(&QString::from_std_str(label)),
            None => QPushButton::new(),
        };

        button.set_object_name(&QString::from_std_str("QueryButton"));
        let button_ptr = button.as_mut_ptr();

        let mut widget = QFrame::new_0a();

        widget.set_object_name(&qs("ButtonFrame"));

        let mut widget_layout = create_hlayout();
        widget_layout.add_widget(button.into_ptr());
        widget.set_layout(widget_layout.into_ptr());
        T::static_upcast_mut(layout).add_widget(widget.into_ptr());
        button_ptr
    }
}
