use crate::cache::PinChangesCache;
use crate::change_type::Change;
use crate::utility::qs;
use crate::ClientProxy;
use packybara::traits::*;

use log;
use packybara::db::update::versionpins::VersionPinChange;
use packybara::packrat::PackratDb;
use qt_widgets::{cpp_core::MutPtr, QInputDialog, QMessageBox, QPushButton, QTableWidget, QWidget};
use std::rc::Rc;
use whoami;
// TODO: clear usage_ptr
pub fn save_versionpin_changes(
    root_widget_ptr: MutPtr<QWidget>,
    pinchanges_ptr: &mut MutPtr<QTableWidget>,
    query_button_ptr: &mut MutPtr<QPushButton>,
    pinchange_cache: Rc<PinChangesCache>,
) {
    let client = ClientProxy::connect().expect("unable to connect via ClientProxy");
    unsafe {
        // grab all the data from the pin changes
        let mut ok = false;
        let ok_p: *mut bool = &mut ok;
        let ok_ptr = MutPtr::from_raw(ok_p);
        //
        // present comments dialog
        //
        let comments = match comments_dialog(root_widget_ptr.clone(), ok_ptr) {
            Ok(c) => c,
            Err(_) => {
                return;
            }
        };
        // update fields.
        let mut pb = PackratDb::new(client);
        let mut tx = pb.transaction();
        let mut tx_cnt = 0;
        for idx in pinchange_cache.change_indexes() {
            let change = pinchange_cache
                .change_at(idx)
                .expect("unable to unwrap change");
            match change {
                Change::ChangeDistribution {
                    vpin_id,
                    new_dist_id,
                } => {
                    let change = VersionPinChange::new(vpin_id, Some(new_dist_id), None);
                    let mut update = PackratDb::update_versionpins(tx)
                        .change(change)
                        .update()
                        .unwrap();
                    tx = update.take_tx();
                    tx_cnt += 1;
                }
                Change::ChangeWiths { vpin_id, withs } => {
                    let mut update = PackratDb::add_withs(tx).create(vpin_id, withs).unwrap();
                    tx = update.take_tx()
                }
                _ => panic!("not implemented"),
            }
        }
        let user = whoami::username();
        // reset book keeping
        pinchange_cache.reset();
        let results = PackratDb::commit(tx, user.as_str(), comments.as_str(), tx_cnt);
        if results.is_ok() {
            pinchanges_ptr.clear();
            pinchanges_ptr.set_row_count(0);
            let mut mb = QMessageBox::new();
            // re-execute query
            query_button_ptr.click();
            mb.set_text(&qs("Success"));
            mb.exec();

        //todo - reset color of query
        } else {
            let mut mb = QMessageBox::new();
            mb.set_text(&qs("Error Occured"));
            mb.set_detailed_text(&qs(format!("{:#?}", results)));
            mb.exec();
        }
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
        } else if *ok_ptr == false {
            let mut mb = QMessageBox::new();
            mb.set_text(&qs("Cancelled"));
            mb.exec();
            return Err(());
        }
        Ok(comments)
    }
}
