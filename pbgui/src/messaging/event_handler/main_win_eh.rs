use super::*;
use crate::main_window::InnerMainWindow;
use crate::messaging::{event::main_win::MainWin, incoming::imain_win::IMainWin};
use crate::traits::RowSetterTrait;
//use std::cell::RefCell;
use crate::constants::*;
use crate::utility::{update_row, RowType};
use std::rc::Rc;

pub unsafe fn match_main_win<'a>(
    event: MainWin,
    main_win: Rc<InnerMainWindow<'a>>,
    receiver: &Receiver<IMsg>,
) {
    match event {
        MainWin::GetVpins => {
            if let Ok(IMsg::MainWin(IMainWin::Vpins(vpins))) = receiver.recv() {
                let mut vpin_tablewidget_ptr = main_win.vpin_table();

                let mut cnt = 0;
                vpin_tablewidget_ptr.set_sorting_enabled(false);
                vpin_tablewidget_ptr.set_row_count(0);
                vpin_tablewidget_ptr.set_row_count(vpins.len() as i32);
                //let mut filtered_cnt = 0;
                for result in vpins {
                    result.set_table_row(&mut vpin_tablewidget_ptr, cnt);
                    cnt += 1;
                }

                vpin_tablewidget_ptr.set_sorting_enabled(true);
            } else {
                log::error!("PackagesTree::GetPackages IMsg does not match event state");
            }
        }
        MainWin::GetWithsForVpin => {
            if let Ok(IMsg::MainWin(IMainWin::WithPackages(withs))) = receiver.recv() {
                let withs = withs.iter().map(|x| x.with.as_str()).collect();
                let withs_list = main_win.package_withs_list();
                withs_list.borrow_mut().set_items(withs);
            } else {
                log::error!("PackagesTree::GetPackages IMsg does not match event state");
            }
        }
        // TODO
        MainWin::GetTransactionChanges => {
            if let Ok(IMsg::MainWin(IMainWin::Changes(changes))) = receiver.recv() {
                //let chan_ref = sites.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                // toolbar.set_site_items(sites_ref);
                let mut changes_table_ptr = main_win.revision_changes_table();
                let mut cnt = 0;
                let r_len = changes.len() as i32;
                changes_table_ptr.set_row_count(r_len);
                for result in changes {
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
            } else {
                log::error!("MainToolbar::GetSites IMsg does not match event state");
            }
        }
    }
}
