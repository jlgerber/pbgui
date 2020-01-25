use crate::cache::PinChangesCache;
use crate::change_type::{Change, ChangeType};
use crate::traits::RowTrait;
//use crate::utility::qs;
use crate::versionpin_row::VersionPinRow;
use crate::ClientProxy;
use packybara::packrat::PackratDb;
use packybara::traits::*;
use pbgui_withs::WithsList;
use qt_core::QString;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    QTableWidget,
};
use std::cell::RefCell;
use std::rc::Rc;

/// Updates the withpackages in response to versionpin selection in the main view
pub fn update_withpackages(
    row: i32,
    vpin_tablewidget_ptr: &mut MutPtr<QTableWidget>,
    item_list: Rc<RefCell<WithsList>>,
    cache: Rc<PinChangesCache>,
) {
    let client = ClientProxy::connect().expect("Unable to connect via ClientProxy");
    let mut packratdb = PackratDb::new(client);
    let table_row =
        VersionPinRow::<CppBox<QString>>::from_table_at_row(&vpin_tablewidget_ptr, row).unwrap();
    let vpin_id = table_row.id;
    if let Some(row) = cache.change_row_from_id(vpin_id as u64, &ChangeType::ChangeWiths) {
        if let Some(Change::ChangeWiths { withs, .. }) = cache.change_at(row) {
            {
                item_list.borrow_mut().clear();
                item_list.borrow_mut().set_items(withs);
            }
        } else {
            log::error!(
                "Missing cached change::ChangeWiths for row {} an vpin {}",
                row,
                vpin_id
            );
            return;
        }
    } else {
        let mut withs_finder = packratdb.find_all_versionpin_withs(vpin_id);
        item_list.borrow_mut().clear();
        let withs = withs_finder.query().expect("unable to get result");
        let withs = withs.iter().map(|x| x.with.as_str()).collect();
        item_list.borrow_mut().set_items(withs);
    }
}
