use crate::constants::*;
use crate::table_headers;
use crate::utility::qs;
use packybara::types::IdType;

use qt_core::{QFlags, QString};
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VersionPinRow {
    pub id: IdType,
    pub dist_id: IdType,
    pub pkgcoord_id: IdType,
    pub distribtution: String,
    pub level: String,
    pub role: String,
    pub platform: String,
    pub site: String,
    pub withs: i32,
}

impl VersionPinRow {
    pub fn new(
        id: IdType,
        dist_id: IdType,
        pkgcoord_id: IdType,
        distribution: CppBox<QString>,
        level: CppBox<QString>,
        role: CppBox<QString>,
        platform: CppBox<QString>,
        site: CppBox<QString>,
        withs: i32,
    ) -> Self {
        Self {
            id,
            dist_id,
            pkgcoord_id,
            distribtution: distribution.to_std_string(),
            level: level.to_std_string(),
            role: role.to_std_string(),
            platform: platform.to_std_string(),
            site: site.to_std_string(),
            withs,
        }
    }
    /// Given a reference to the versionpin table, and a row number. return the row
    pub fn from_table_at_row(
        versionpin_table: &MutPtr<QTableWidget>,
        row: i32,
    ) -> Option<VersionPinRow> {
        unsafe {
            if row < 0 || versionpin_table.row_count() <= row {
                log::warn!(
                    "row requested out of bounds: row count:{} requested:{}",
                    versionpin_table.row_count(),
                    row
                );
                return None;
            }
            let vpin_id = versionpin_table.item(row, COL_ID).data(2).to_int_0a();
            let dist_id = versionpin_table
                .item(row, COL_DISTRIBUTION_ID)
                .data(2)
                .to_int_0a();
            let pkgcoord_id = versionpin_table
                .item(row, COL_PKGCOORD_ID)
                .data(2)
                .to_int_0a();
            let distribtution = versionpin_table.item(row, COL_DISTRIBUTION).text();
            let level = versionpin_table.item(row, COL_LEVEL).text();
            let role = versionpin_table.item(row, COL_ROLE).text();
            let platform = versionpin_table.item(row, COL_PLATFORM).text();
            let site = versionpin_table.item(row, COL_SITE).text();
            let withs = versionpin_table.item(row, COL_WITHS).data(2).to_int_0a();
            Some(VersionPinRow::new(
                vpin_id,
                dist_id,
                pkgcoord_id,
                distribtution,
                level,
                role,
                platform,
                site,
                withs,
            ))
        }
    }
}
