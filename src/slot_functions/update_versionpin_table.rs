use crate::traits::RowSetterTrait;
use crate::ClientProxy;
use packybara::packrat::PackratDb;
use packybara::traits::*;
use packybara::LtreeSearchMode;
use pbgui_toolbar::toolbar;
use qt_widgets::{cpp_core::MutPtr, QAction, QTableWidget};
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

/// update the main versionpin table by gathering the user's requested query parameters from    
/// the comboboxes up top, querying the database, and updating the table.alloc
///
/// # Arguments
///                         
pub fn update_vpin_table(
    // direction
    toolbar: Rc<RefCell<toolbar::MainToolbar>>,
    search_shows: &MutPtr<QAction>,
    mut vpin_tablewidget_ptr: MutPtr<QTableWidget>,
) {
    // will do better
    let client = ClientProxy::connect().expect("Unable to connect via ClientProxy");
    let mut packratdb = PackratDb::new(client);
    let mut vpin_finder = packratdb.find_all_versionpins();

    unsafe {
        let dirtxt = toolbar.borrow().dir.current_text().to_std_string();
        let line_edit_txt = toolbar.borrow().line_edit.text().to_std_string();
        let showtxt = toolbar.borrow().level.current_text().to_std_string();
        let roletxt = toolbar.borrow().role.current_text().to_std_string();
        let platformtxt = toolbar.borrow().platform.current_text().to_std_string();
        let sitetxt = toolbar.borrow().site.current_text().to_std_string();

        vpin_finder
            .level(showtxt.as_str())
            .isolate_facility(search_shows.is_checked())
            .role(roletxt.as_str())
            .platform(platformtxt.as_str())
            .site(sitetxt.as_str())
            .search_mode(LtreeSearchMode::from_str(dirtxt.as_str()).expect("unable to find vpin"));
        let filter_package = if line_edit_txt != "" { true } else { false };
        let results = vpin_finder
            .query()
            .expect("unable to unwrap vpin_finder.query");
        let mut cnt = 0;
        vpin_tablewidget_ptr.set_sorting_enabled(false);
        vpin_tablewidget_ptr.set_row_count(0);
        vpin_tablewidget_ptr.set_row_count(results.len() as i32);
        let mut filtered_cnt = 0;
        for result in results {
            if filter_package && line_edit_txt != "" {
                if !result
                    .distribution
                    .package()
                    .contains(line_edit_txt.as_str())
                {
                    filtered_cnt += 1;
                    continue;
                }
            }
            result.set_table_row(&mut vpin_tablewidget_ptr, cnt);
            cnt += 1;
        }
        if filtered_cnt > 0 {
            let rc = vpin_tablewidget_ptr.row_count() - filtered_cnt;
            if rc != cnt {
                log::warn!("Row count: {} not equal to cnt {}", rc, cnt);
            }
            vpin_tablewidget_ptr.set_row_count(rc);
        }
        vpin_tablewidget_ptr.set_sorting_enabled(true);
    }
}
