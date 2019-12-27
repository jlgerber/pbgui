use crate::cache::PinChangesCache;
use crate::change_type::ChangeType;
use crate::components::versionpin_table::VersionPinRow;
use crate::constants::*;
use qt_widgets::{cpp_core::MutPtr, QListWidget, QTableWidget};
use std::rc::Rc;
/// add the withpackage change to the listx`
pub fn store_withpackage_changes(
    withpackage_list: MutPtr<QListWidget>,
    versionpin_table: MutPtr<QTableWidget>,
    changes_table: &mut MutPtr<QTableWidget>,
    cache: Rc<PinChangesCache>,
) {
    unsafe {
        // build up a list of packages in order as a vec of String
        let mut items = Vec::with_capacity(withpackage_list.count() as usize);
        for cnt in 0..withpackage_list.count() {
            let item = withpackage_list.item(cnt);
            items.push(item.text().to_std_string());
        }
        // get current versionpin distribution_id
        let selection_model = versionpin_table.selection_model();
        if selection_model.has_selection() {
            let row = selection_model.selected_rows_0a().first().row();
            let change_type = ChangeType::Withs;
            let table_row = VersionPinRow::from_table_at_row(&versionpin_table, row);
            if table_row.is_none() {
                log::warn!("Tablerow {} is None", row);
                return;
            }
            let table_row = table_row.unwrap();
            //let vpin_id = versionpin_table.item(row, COL_ID).data(2).to_int_0a();
            println!("table row: {:#?}", table_row);
            println!("New Withs:\n{:#?}", &items);
            cache.cache_withs(table_row.id, items);
            // store change
        }
    }
}
