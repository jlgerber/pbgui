use crate::constants::*;
use crate::table_headers;
use crate::utility::qs;
use qt_core::QFlags;
use qt_widgets::{
    cpp_core::MutPtr,
    q_abstract_item_view::{EditTrigger, SelectionBehavior, SelectionMode},
    q_header_view::ResizeMode,
    qt_core::ContextMenuPolicy,
    QSplitter, QTableWidget,
};

/// Setup the main VersionPin Table.
///
/// This table presents VersionPins, populated in response to the user's request via
/// the top push button.
///
/// # Arguments
/// * `vsplit_ptr` Pointer to the main vertical splitter (running horizontally)
///
/// # Returns
/// * `MutPtr<QTableWidget>` - a mutable pointer to the main table
pub fn create(vsplit_ptr: &mut MutPtr<QSplitter>) -> MutPtr<QTableWidget> {
    unsafe {
        // create the tablewidget
        let mut vpin_tablewidget = QTableWidget::new_2a(0, HEADERS.len() as i32);
        let mut tablewidget_ptr = vpin_tablewidget.as_mut_ptr();
        tablewidget_ptr.set_object_name(&qs("VersionPinTable"));
        tablewidget_ptr
            .horizontal_header()
            .set_object_name(&qs("VersionPinTableHeader"));
        vsplit_ptr.add_widget(vpin_tablewidget.into_ptr());
        // configure the tablewidget
        tablewidget_ptr.vertical_header().hide();
        tablewidget_ptr.set_selection_behavior(SelectionBehavior::SelectRows);
        tablewidget_ptr.set_edit_triggers(QFlags::from(EditTrigger::NoEditTriggers));
        tablewidget_ptr.set_selection_mode(SelectionMode::SingleSelection);
        tablewidget_ptr.set_show_grid(false);
        tablewidget_ptr.set_alternating_row_colors(true);
        tablewidget_ptr.set_context_menu_policy(ContextMenuPolicy::CustomContextMenu);
        table_headers::setup(&mut tablewidget_ptr, &HEADERS);
        tablewidget_ptr
            .horizontal_header()
            .set_section_resize_mode_1a(ResizeMode::Stretch); //Stretch
        tablewidget_ptr
            .horizontal_header()
            .set_section_resize_mode_2a(COL_ID, ResizeMode::ResizeToContents);
        tablewidget_ptr
            .horizontal_header()
            .set_section_resize_mode_2a(COL_WITHS, ResizeMode::ResizeToContents);
        tablewidget_ptr
    }
}
