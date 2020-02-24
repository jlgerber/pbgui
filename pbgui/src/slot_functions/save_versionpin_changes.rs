use crate::cache::PinChangesCache;
use crate::change_type::Change;
use crate::messaging::outgoing::omain_win::OMainWin;
use crate::messaging::OMsg;
use crate::messaging::Sender;
use crate::utility::qs;
use log;
use qt_widgets::{cpp_core::MutPtr, QInputDialog, QMessageBox, QWidget};
use std::rc::Rc;
use whoami;

pub fn save_versionpin_changes(
    root_widget_ptr: MutPtr<QWidget>,
    pinchange_cache: Rc<PinChangesCache>,
    to_thread_sender: Sender<OMsg>,
) {
    unsafe {
        // grab all the data from the pin changes
        let mut ok = false;
        let ok_p: *mut bool = &mut ok;
        let ok_ptr = MutPtr::from_raw(ok_p);
        //
        // present comments dialog
        //
        let comments = match comments_dialog(root_widget_ptr, ok_ptr) {
            Ok(c) => c,
            Err(_) => {
                return;
            }
        };

        // We will send change to secondary thread as vec<change>
        let mut change_vec: Vec<Change> = Vec::new();
        // Retrieve the indexes of the changes in the cache and look up the changes
        // from the cache. We introduce this indirection to make it simple to delete
        // a change from the change table without having to delete an item from the
        // vector of changes in the cache, which would lead to an O(n) operation.
        for idx in pinchange_cache.change_indexes() {
            let change = pinchange_cache
                .change_at(idx)
                .expect("unable to unwrap change");

            change_vec.push(change);
        }
        let user = whoami::username();

        // Now that we have used the cache for its intended purpose, we reset it back to
        // its initial state.
        pinchange_cache.reset();
        log::debug!("signaling SaveVpinChanges");
        to_thread_sender
            .send(OMsg::MainWin(OMainWin::SaveVpinChanges {
                changes: change_vec,
                user,
                comments,
            }))
            .expect("unable to save versionpin changes");
    }
}

// present the dialog for comments to the user and handle the return values
fn comments_dialog(root_widget_ptr: MutPtr<QWidget>, ok_ptr: MutPtr<bool>) -> Result<String, ()> {
    unsafe {
        let comments = QInputDialog::get_multi_line_text_5a(
            root_widget_ptr,
            &qs("Save Changes"),
            &qs("Comment"),
            &qs(""),
            ok_ptr,
        )
        .to_std_string();
        if ok_ptr.is_null() {
            log::error!("In save_versionpin_changes. QInputDialog returned null ok_ptr.");
            let mut mb = QMessageBox::new();
            mb.set_text(&qs("QT Problem Detected - null ok_ptr"));
            mb.exec();
            return Err(());
        } else if !(*ok_ptr) {
            let mut mb = QMessageBox::new();
            mb.set_text(&qs("Cancelled"));
            mb.exec();
            return Err(());
        }
        Ok(comments)
    }
}
