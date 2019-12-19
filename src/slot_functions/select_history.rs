use crate::constants::*;
use crate::ClientProxy;
use packybara::packrat::PackratDb;
use packybara::OrderDirection;
use packybara::OrderRevisionBy;
use qt_core::{QString, QVariant};
use qt_widgets::{cpp_core::MutPtr, QStackedWidget, QTableWidget, QTableWidgetItem};

pub fn select_history(
    revisions_ptr: &mut MutPtr<QTableWidget>,
    stacked_ptr: &mut MutPtr<QStackedWidget>,
) {
    unsafe {
        stacked_ptr.set_current_index(1);
        let client = ClientProxy::connect().expect("unable to connect via CLientproxy");
        let mut packratdb = PackratDb::new(client);
        let mut revisions_finder = packratdb.find_all_revisions();
        let results = revisions_finder
            .order_by(vec![OrderRevisionBy::Id])
            .order_direction(OrderDirection::Desc)
            .query()
            .expect("failed to call db");
        //println!("{:?}", results);
        revisions_ptr.clear_contents();
        //revisions_ptr.set_row_count(0);
        let r_len = results.len() as i32;
        //println!("length {}", r_len);
        revisions_ptr.set_row_count(r_len);
        let mut cnt = 0;
        for revision in results {
            let mut revisions_table_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(revision.transaction_id as i32);
            revisions_table_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            revisions_ptr.set_item(cnt, COL_REV_TXID, revisions_table_item.into_ptr());
            // Author
            let mut revisions_table_item = QTableWidgetItem::new();
            revisions_table_item
                .set_text(&QString::from_std_str(revision.author.to_string().as_str()));
            revisions_ptr.set_item(cnt, COL_REV_AUTHOR, revisions_table_item.into_ptr());
            // Datetime
            let mut revisions_table_item = QTableWidgetItem::new();
            revisions_table_item.set_text(&QString::from_std_str(
                revision.datetime.format("%F %r").to_string().as_str(),
            ));
            revisions_ptr.set_item(cnt, COL_REV_DATETIME, revisions_table_item.into_ptr());
            // comment
            let mut revisions_table_item = QTableWidgetItem::new();
            revisions_table_item.set_text(&QString::from_std_str(
                revision.comment.to_string().as_str(),
            ));
            revisions_ptr.set_item(cnt, COL_REV_COMMENT, revisions_table_item.into_ptr());
            cnt += 1;
        }
    }
}
