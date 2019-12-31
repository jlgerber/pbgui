use crate::cache::PinChangesCache;
use crate::change_type::{Change, ChangeType};
use crate::traits::RowTrait;
use crate::utility::qs;
use crate::versionpin_row::VersionPinRow;
use crate::ClientProxy;
use packybara::packrat::PackratDb;
use qt_core::QString;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    QListWidget, QTableWidget,
};
use std::rc::Rc;

/// Updates the withpackages in response to versionpin selection in the main view
pub fn update_withpackages(
    row: i32,
    vpin_tablewidget_ptr: &mut MutPtr<QTableWidget>,
    withpackage_list: &mut MutPtr<QListWidget>,
    cache: Rc<PinChangesCache>,
) {
    let client = ClientProxy::connect().expect("Unable to connect via ClientProxy");
    let mut packratdb = PackratDb::new(client);
    unsafe {
        let table_row =
            VersionPinRow::<CppBox<QString>>::from_table_at_row(&vpin_tablewidget_ptr, row)
                .unwrap();
        //let vpin_id = vpin_tablewidget_ptr.item(row, COL_ID).data(2).to_int_0a();
        let vpin_id = table_row.id;
        if let Some(row) = cache.change_row_from_id(vpin_id as u64, &ChangeType::ChangeWiths) {
            if let Some(Change::ChangeWiths { withs, .. }) = cache.change_at(row) {
                withpackage_list.clear();
                for item in withs {
                    withpackage_list.add_item_q_string(&qs(item));
                }
            } else {
                log::error!(
                    "Missing cached change::ChangeWiths for row {} an vpin {}",
                    row,
                    vpin_id
                );
                return;
                //panic!("should not be here");
            }
        } else {
            let mut withs_finder = packratdb.find_all_versionpin_withs(vpin_id);
            withpackage_list.clear();
            for item in withs_finder.query().expect("unable to get result") {
                withpackage_list.add_item_q_string(&qs(item.with));
            }
        }
    }
}
