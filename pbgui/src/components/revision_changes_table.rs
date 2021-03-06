use crate::constants::*;
use crate::table_headers;
use crate::utility::qs;
use qt_core::AlignmentFlag;
use qt_core::QFlags;
use qt_widgets::{
    cpp_core::CppBox,
    q_abstract_item_view::{EditTrigger, SelectionBehavior, SelectionMode},
    q_header_view::ResizeMode,
    QTableWidget,
};

/// Create the Changes Table.
/// There are 1 to N number of changes per revision. The Revisions table
/// is populated as a consequence of selecting a particular revision from
/// the Revisions Table.
///
/// # Arguments
/// * None
///
/// # Returns
/// * The Change History Table
pub fn create() -> CppBox<QTableWidget> {
    unsafe {
        let mut changes = QTableWidget::new_2a(0, CHNG_HEADERS.len() as i32);
        let mut changes_ptr = changes.as_mut_ptr();
        changes.vertical_header().hide();
        //changes.horizontal_header().hide();
        changes.set_selection_behavior(SelectionBehavior::SelectRows);
        changes.set_edit_triggers(QFlags::from(EditTrigger::NoEditTriggers));
        changes.set_selection_mode(SelectionMode::SingleSelection);
        changes.horizontal_header().set_stretch_last_section(true);
        changes
            .horizontal_header()
            .set_default_alignment(QFlags::from(AlignmentFlag::AlignLeft));
        changes
            .horizontal_header()
            .set_section_resize_mode_1a(ResizeMode::ResizeToContents);
        changes.set_show_grid(false);
        // The following two statements are responsible for the spacing
        // between entries in the changes table
        changes.vertical_header().set_maximum_section_size(20);
        changes
            .vertical_header()
            .set_section_resize_mode_1a(ResizeMode::ResizeToContents);
        table_headers::setup(&mut changes_ptr, &CHNG_HEADERS);
        changes
            .horizontal_header()
            .set_object_name(&qs("ChangesHeader"));
        changes
    }
}
