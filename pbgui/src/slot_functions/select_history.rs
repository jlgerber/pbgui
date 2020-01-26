use crate::messaging::outgoing::omain_win::OMainWin;
use crate::messaging::OMsg;
use crate::messaging::Sender;

use qt_widgets::{cpp_core::MutPtr, QStackedWidget, QTableWidget};

pub fn select_history(
    revisions_ptr: &mut MutPtr<QTableWidget>,
    stacked_ptr: &mut MutPtr<QStackedWidget>,
    to_thread_sender: Sender<OMsg>,
) {
    unsafe {
        stacked_ptr.set_current_index(1);
        log::debug!("clearing contents for revisions");
        revisions_ptr.clear_contents();
        log::debug!("contents cleared");
        to_thread_sender
            .send(OMsg::MainWin(OMainWin::GetHistoryRevisions))
            .expect("unable to get history revisions");
    }
}
