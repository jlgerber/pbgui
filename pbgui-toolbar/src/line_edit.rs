use qt_core::{ContextMenuPolicy, QString, WidgetAttribute};
use qt_widgets::{
    cpp_core::MutPtr,
    cpp_core::{CppBox, StaticUpcast},
    QAction, QFrame, QLayout, QLineEdit, QMenu,
};
use rustqt_utils::{create_hlayout, qs};

/// Create a line_edit
pub fn create<I>(hlayout_ptr: MutPtr<I>) -> (MutPtr<QLineEdit>, CppBox<QMenu>, MutPtr<QAction>)
where
    I: StaticUpcast<QLayout>,
{
    unsafe {
        let mut package_line_edit = QLineEdit::new();
        package_line_edit.set_attribute_2a(WidgetAttribute::WAMacShowFocusRect, false);
        package_line_edit.set_object_name(&qs("PackageLineEdit"));
        package_line_edit.set_clear_button_enabled(true);
        package_line_edit.set_context_menu_policy(ContextMenuPolicy::CustomContextMenu);
        let mut line_edit_popup_menu = QMenu::new();
        //let _line_edit_popup_menu_ptr = line_edit_popup_menu.as_mut_ptr();
        let choose_line_edit_clear_action =
            line_edit_popup_menu.add_action_q_string(&QString::from_std_str("Clear"));
        let line_edit_ptr = package_line_edit.as_mut_ptr();
        let mut line_edit_frame = QFrame::new_0a();
        line_edit_frame.set_object_name(&qs("PackageLineEditFrame"));
        let mut line_edit_layout = create_hlayout();
        let mut lel_ptr = line_edit_layout.as_mut_ptr();
        line_edit_frame.set_layout(line_edit_layout.into_ptr());
        lel_ptr.add_widget(package_line_edit.into_ptr());

        I::static_upcast_mut(hlayout_ptr).add_widget(line_edit_frame.into_ptr());

        (
            line_edit_ptr,
            line_edit_popup_menu,
            choose_line_edit_clear_action,
        )
    }
}
