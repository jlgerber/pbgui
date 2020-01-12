use crate::constants::*;
use crate::utility::{update_row, RowType};
use crate::ClientProxy;
use log;
use packybara::packrat::PackratDb;
use packybara::traits::*;
use qt_widgets::{cpp_core::MutPtr, QTableWidget};

/// Update the changes table with new data
pub fn update_changes_table(
    row: i32,
    revisions_ptr: MutPtr<QTableWidget>,
    mut changes_table_ptr: MutPtr<QTableWidget>,
) {
    unsafe {
        changes_table_ptr.clear_contents();
        let client = match ClientProxy::connect() {
            Ok(c) => c,
            Err(e) => {
                log::error!("Problem getting proxy client to db {}", e);
                return;
            }
        };
        let mut packratdb = PackratDb::new(client);
        let mut changes_finder = packratdb.find_all_changes();

        let data = revisions_ptr.item(row, COL_REV_TXID).data(2).to_int_0a();
        let mut cnt = 0;
        let results = changes_finder
            .transaction_id(data as i64)
            .query()
            .expect("failed to call db");
        let r_len = results.len() as i32;
        changes_table_ptr.set_row_count(r_len);
        for result in results {
            update_row(
                RowType::Int(result.id as i32),
                &mut changes_table_ptr,
                cnt,
                COL_CHNG_ID,
            );
            update_row(
                RowType::Int(result.transaction_id as i32),
                &mut changes_table_ptr,
                cnt,
                COL_CHNG_TXID,
            );
            update_row(
                RowType::Str(result.action.to_string().as_str()),
                &mut changes_table_ptr,
                cnt,
                COL_CHNG_ACTION,
            );
            update_row(
                RowType::Str(result.level.to_string().as_str()),
                &mut changes_table_ptr,
                cnt,
                COL_CHNG_LEVEL,
            );
            update_row(
                RowType::Str(result.role.to_string().as_str()),
                &mut changes_table_ptr,
                cnt,
                COL_CHNG_ROLE,
            );
            update_row(
                RowType::Str(&result.platform.to_string()),
                &mut changes_table_ptr,
                cnt,
                COL_CHNG_PLATFORM,
            );
            update_row(
                RowType::Str(&result.site.to_string()),
                &mut changes_table_ptr,
                cnt,
                COL_CHNG_SITE,
            );
            update_row(
                RowType::Str(&result.package.to_string()),
                &mut changes_table_ptr,
                cnt,
                COL_CHNG_PKG,
            );
            update_row(
                RowType::Str(&result.old.version()),
                &mut changes_table_ptr,
                cnt,
                COL_CHNG_OLD,
            );
            update_row(
                RowType::Str(&result.new.version()),
                &mut changes_table_ptr,
                cnt,
                COL_CHNG_NEW,
            );

            cnt += 1;
        }
    }
}
