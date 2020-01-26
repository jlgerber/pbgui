use crate::constants::*;
use crate::messaging::outgoing::omain_win::OMainWin;
use crate::messaging::OMsg;
use crate::messaging::Sender;
//use log;
use qt_widgets::{cpp_core::MutPtr, QTableWidget};

/// Update the changes table with new data
pub fn update_changes_table(
    row: i32,
    revisions_ptr: MutPtr<QTableWidget>,
    mut changes_table_ptr: MutPtr<QTableWidget>,
    to_thread_sender: Sender<OMsg>,
) {
    unsafe {
        changes_table_ptr.clear_contents();
        let data = revisions_ptr.item(row, COL_REV_TXID).data(2).to_int_0a();
        to_thread_sender
            .send(OMsg::MainWin(OMainWin::GetTransactionChanges {
                tx_id: data,
            }))
            .expect("unable to get vpins");
    }
}
