use crate::utility::qs;
use qt_core::{ContextMenuPolicy, QString, WidgetAttribute};
use qt_widgets::{cpp_core::CppBox, cpp_core::MutPtr, QAction, QHBoxLayout, QLineEdit, QMenu};

pub fn create(
    hlayout_ptr: &mut MutPtr<QHBoxLayout>,
) -> (MutPtr<QLineEdit>, CppBox<QMenu>, MutPtr<QAction>) {
    unsafe {
        let mut package_line_edit = QLineEdit::new();
        package_line_edit.set_attribute_2a(WidgetAttribute::WAMacShowFocusRect, false);
        package_line_edit.set_object_name(&qs("PackageLineEdit"));
        // let clear_icon = QIcon::from_q_string(&QString::from_std_str(":/images/clear.png"));
        // if clear_icon.is_null() {
        //     log::warn!("The :/images/clear.png icon was unable to be located.");
        // }
        // let _clear_action = package_line_edit.add_action_q_icon_action_position(
        //     clear_icon.as_ref(),
        //     ActionPosition::TrailingPosition,
        // );
        package_line_edit.set_clear_button_enabled(true);
        package_line_edit.set_context_menu_policy(ContextMenuPolicy::CustomContextMenu);
        let mut line_edit_popup_menu = QMenu::new();
        let _line_edit_popup_menu_ptr = line_edit_popup_menu.as_mut_ptr();
        let choose_line_edit_clear_action =
            line_edit_popup_menu.add_action_q_string(&QString::from_std_str("Clear"));
        let line_edit_ptr = package_line_edit.as_mut_ptr();
        hlayout_ptr.add_widget(package_line_edit.into_ptr());

        (
            line_edit_ptr,
            line_edit_popup_menu,
            choose_line_edit_clear_action,
        )
    }
}
