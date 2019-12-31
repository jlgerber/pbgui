use crate::cache::PinChangesCache;
use crate::change_type::{Change, ChangeType};
use crate::traits::RowTrait;
use crate::utility::qs;
use crate::{
    versionpin_changes_row::{RowSetterTrait, VersionPinChangesRow},
    versionpin_row::VersionPinRow,
};
use qt_core::QString;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    QListWidget, QTableWidget,
};
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
            let _change_type = ChangeType::ChangeWiths;
            let table_row =
                VersionPinRow::<CppBox<QString>>::from_table_at_row(&versionpin_table, row);
            if table_row.is_none() {
                log::warn!("Tablerow {} is None", row);
                return;
            }
            let table_row = table_row.unwrap();
            println!("table row: {:#?}", table_row);
            println!("New Withs:\n{:#?}", &items);
            let new_withs = items.join(",");
            let change = Change::ChangeWiths {
                vpin_id: table_row.id,
                withs: items,
            };
            cache.cache_change(change);
            //cache.cache_withs(table_row.id, items);
            // store change
            let change_row = VersionPinChangesRow::<CppBox<QString>>::new(
                ChangeType::ChangeWiths,
                table_row.pkgcoord(),
                qs(""),
                qs(new_withs),
            );
            println!("storing {:?}", change_row);
            change_row.set_table_row(changes_table, changes_table.row_count());
        }
    }
}
