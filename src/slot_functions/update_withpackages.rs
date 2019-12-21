use crate::constants::*;
use crate::utility::qs;
use crate::ClientProxy;
use packybara::packrat::PackratDb;
use qt_widgets::{cpp_core::MutPtr, QListWidget, QTableWidget};
pub fn update_withpackages(
    row: i32,
    vpin_tablewidget_ptr: &mut MutPtr<QTableWidget>,
    withpackage_list: &mut MutPtr<QListWidget>,
) {
    let client = ClientProxy::connect().expect("Unable to connect via ClientProxy");
    let mut packratdb = PackratDb::new(client);
    unsafe {
        let vpin_id = vpin_tablewidget_ptr.item(row, COL_ID).data(2).to_int_0a();
        let mut withs_finder = packratdb.find_all_versionpin_withs(vpin_id);
        withpackage_list.clear();
        for item in withs_finder.query().expect("unable to get result") {
            withpackage_list.add_item_q_string(&qs(item.with));
        }
    }
}
