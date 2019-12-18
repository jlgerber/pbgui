use crate::constants::*;
use crate::utility::qs;
use crate::ClientProxy;
use packybara::db::update::versionpins::VersionPinChange;
use packybara::packrat::PackratDb;
use qt_widgets::{cpp_core::MutPtr, QInputDialog, QMessageBox, QTableWidget, QWidget};
use whoami;

pub fn save_versionpin_changes(
    root_widget_ptr: MutPtr<QWidget>,
    pinchanges_ptr: &mut MutPtr<QTableWidget>,
) {
    let client = ClientProxy::connect().unwrap();
    unsafe {
        // grab all the data from the pin changes
        let mut ok = false;
        let ok_p: *mut bool = &mut ok;
        let ok_ptr = MutPtr::from_raw(ok_p);
        let comments = QInputDialog::get_multi_line_text_5a(
            root_widget_ptr,
            &qs("Save Changes"),
            &qs("Comment"),
            &qs(""),
            ok_ptr,
        )
        .to_std_string();
        if *ok_ptr == false {
            let mut mb = QMessageBox::new();
            mb.set_text(&qs("Cancelled"));
            mb.exec();
            return;
        }
        let mut pb = PackratDb::new(client);
        let user = whoami::username();
        let mut update = pb.update_versionpins(comments.as_str(), user.as_str());
        let mut changes = Vec::new();
        for row_idx in 0..pinchanges_ptr.row_count() {
            let vpin_id = pinchanges_ptr.item(row_idx, COL_PC_VPINID).data(2);
            let dist_id = pinchanges_ptr.item(row_idx, COL_PC_DISTID).data(2);
            changes.push(VersionPinChange::new(
                vpin_id.to_int_0a(),
                Some(dist_id.to_int_0a()),
                None,
            ));
        }
        let results = update.changes(&mut changes).update();
        if results.is_ok() {
            pinchanges_ptr.clear();
            pinchanges_ptr.set_row_count(0);
            let mut mb = QMessageBox::new();
            mb.set_text(&qs("Success"));
            mb.exec();
        //todo - reset color of query
        } else {
            let mut mb = QMessageBox::new();
            mb.set_text(&qs("Error Occured"));
            mb.set_detailed_text(&qs(format!("{:#?}", results)));
            //println!("{:#?}", results);
            mb.exec();
        }
    }
}
