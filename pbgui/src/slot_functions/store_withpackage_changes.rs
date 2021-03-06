use crate::cache::PinChangesCache;
use crate::change_type::{Change, ChangeType};
use crate::traits::RowTrait;
use crate::utility::qs;
use crate::{
    versionpin_changes_row::{RowSetterTrait, VersionPinChangesRow},
    versionpin_row::VersionPinRow,
};
use pbgui_withs::WithsList;
use qt_core::QString;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    QTableWidget,
};
use std::cell::RefCell;
use std::rc::Rc;

/// add the withpackage change to the listx`
pub fn store_withpackage_changes(
    item_list: Rc<RefCell<WithsList>>,
    versionpin_table: MutPtr<QTableWidget>,
    changes_table: &mut MutPtr<QTableWidget>,
    cache: Rc<PinChangesCache>,
) {
    unsafe {
        let items = item_list.borrow().items();

        // get current versionpin distribution_id
        let selection_model = versionpin_table.selection_model();
        if selection_model.has_selection() {
            let row = selection_model.selected_rows_0a().first().row();
            let ctype = ChangeType::ChangeWiths;
            let table_row =
                VersionPinRow::<CppBox<QString>>::from_table_at_row(&versionpin_table, row);
            if table_row.is_none() {
                log::warn!("Tablerow {} is None", row);
                return;
            }
            let table_row = table_row.ok_or(false).expect("unable to unwrap table_row");

            let new_withs = items.join(",");
            let change = Change::ChangeWiths {
                vpin_id: table_row.id,
                withs: items,
            };
            // store change
            let change_row = VersionPinChangesRow::<CppBox<QString>>::new(
                ChangeType::ChangeWiths,
                table_row.pkgcoord(),
                qs(""),
                qs(new_withs),
            );
            if let Some(row) = cache.change_row_from_id(change.id(), ctype) {
                // we found a row, we will insert in that row
                cache.cache_change_at(change, row);
                change_row.set_table_row(changes_table, row);
            } else {
                cache.cache_change(change);
                change_row.set_table_row(changes_table, changes_table.row_count());
            }
        }
    }
}
