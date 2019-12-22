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

/// Create the Table of Revisions.
/// Each transaction executed by a user results in a Revision tracking the
/// event. For each Revision, there are 1 to N number of Changes, presented
/// in the companion Changes Table.
///
/// # Arguments
/// * None
///
/// # Returns
/// * The Revisions Table
pub fn create() -> CppBox<QTableWidget> {
    unsafe {
        let mut revisions = QTableWidget::new_2a(0, REV_HEADERS.len() as i32);
        let mut revisions_ptr = revisions.as_mut_ptr();
        revisions.vertical_header().hide();
        //revisions.horizontal_header().hide();
        revisions.set_selection_behavior(SelectionBehavior::SelectRows);
        revisions.set_edit_triggers(QFlags::from(EditTrigger::NoEditTriggers));
        revisions.set_selection_mode(SelectionMode::SingleSelection);
        revisions.horizontal_header().set_stretch_last_section(true);
        revisions
            .horizontal_header()
            .set_default_alignment(QFlags::from(AlignmentFlag::AlignLeft));
        revisions
            .horizontal_header()
            .set_section_resize_mode_1a(ResizeMode::ResizeToContents);
        revisions.set_show_grid(false);
        // The following two statements are responsible for the spacing
        // between entries in the revisions table
        revisions.vertical_header().set_maximum_section_size(20);
        revisions
            .vertical_header()
            .set_section_resize_mode_1a(ResizeMode::ResizeToContents);
        table_headers::setup(&mut revisions_ptr, &REV_HEADERS);
        revisions
            .horizontal_header()
            .set_object_name(&qs("RevisionsHeader"));
        revisions
    }
}
