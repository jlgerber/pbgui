use crate::constants::*;
use crate::utility::qs;
use packybara::db::update::versionpins::VersionPinChange;
use packybara::packrat::{Client, NoTls, PackratDb};
use qt_widgets::{cpp_core::MutPtr, QInputDialog, QMessageBox, QTableWidget, QWidget};
use whoami;

pub fn save_versionpin_changes(
    root_widget_ptr: MutPtr<QWidget>,
    pinchanges_ptr: &mut MutPtr<QTableWidget>,
) {
    unsafe {
        // grab all the data from the pin changes
        let client = Client::connect(
            "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
            NoTls,
        )
        .unwrap();
        let comments = QInputDialog::get_multi_line_text_3a(
            root_widget_ptr,
            &qs("Save Changes"),
            &qs("Comment"),
        )
        .to_std_string();
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
