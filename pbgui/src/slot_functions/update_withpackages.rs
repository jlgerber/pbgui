use crate::cache::PinChangesCache;
use crate::change_type::{Change, ChangeType};
use crate::messaging::outgoing::omain_win::OMainWin;
use crate::messaging::OMsg;
use crate::messaging::Sender;
use crate::traits::RowTrait;
use crate::versionpin_row::VersionPinRow;
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
    to_thread_sender: Sender<OMsg>,
) {
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
        // clear item list
        item_list.borrow_mut().clear();
        to_thread_sender
            .send(OMsg::MainWin(OMainWin::GetWithsForVpin { vpin_id }))
            .expect("unable to get vpins");
    }
}
