use crate::constants::*;
use crate::table_headers;
use qt_core::QFlags;
use qt_widgets::{
    cpp_core::CppBox,
    q_abstract_item_view::{EditTrigger, SelectionBehavior, SelectionMode},
    q_header_view::ResizeMode,
    QTableWidget,
};

/// Create the Table tracking Changes per Revision. For each Revision, there
/// are 1 to N Changes, desplayed within this table. The user selects a
/// Revision from the revision table and this table is populated in response.
///
/// # Arguments
/// * None
///
/// # Returns
/// * The Revision Table
pub fn create() -> CppBox<QTableWidget> {
    unsafe {
        let mut pinchanges = QTableWidget::new_2a(0, PC_HEADERS.len() as i32);
        let mut pinchanges_ptr = pinchanges.as_mut_ptr();
        table_headers::setup(&mut pinchanges_ptr, &PC_HEADERS);
        pinchanges.vertical_header().hide();
        pinchanges.horizontal_header().hide();
        pinchanges.set_selection_behavior(SelectionBehavior::SelectRows);
        pinchanges.set_edit_triggers(QFlags::from(EditTrigger::NoEditTriggers));
        pinchanges.set_selection_mode(SelectionMode::SingleSelection);
        pinchanges
            .horizontal_header()
            .set_stretch_last_section(true);
        pinchanges
            .horizontal_header()
            .set_section_resize_mode_1a(ResizeMode::Stretch);
        pinchanges.set_show_grid(false);
        // The following two statements are responsible for the spacing
        // between entries in the pinchanges table
        pinchanges.vertical_header().set_maximum_section_size(20);
        pinchanges
            .vertical_header()
            .set_section_resize_mode_1a(ResizeMode::ResizeToContents);
        pinchanges
            .horizontal_header()
            .set_section_resize_mode_2a(COL_ID, ResizeMode::ResizeToContents);
        pinchanges
    }
}
