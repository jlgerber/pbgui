use super::*;
use crate::{
    constants::*,
    main_window::InnerMainWindow,
    messaging::{event::main_win::MainWin, incoming::imain_win::IMainWin},
    traits::RowSetterTrait,
    utility::{qs, update_row, RowType},
};
use std::rc::Rc;

use qt_core::{QString, QVariant};
use qt_widgets::{QMessageBox, QTableWidgetItem};

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
        MainWin::GetTransactionChanges => {
            if let Ok(IMsg::MainWin(IMainWin::Changes(changes))) = receiver.recv() {
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
                log::error!("MainToolbar::GetTransactionChanges IMsg does not match event state");
            }
        }
        MainWin::GetHistoryRevisions => {
            if let Ok(IMsg::MainWin(IMainWin::HistoryRevisions(revisions))) = receiver.recv() {
                let mut revisions_ptr = main_win.revisions_table();
                let r_len = revisions.len() as i32;
                revisions_ptr.set_row_count(r_len);
                let mut cnt = 0;
                for revision in revisions {
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
            } else {
                log::error!("PackagesTree::GetHistoryRevisions IMsg does not match event state");
            }
        }
        MainWin::SaveVpinChanges => {
            if let Ok(IMsg::MainWin(IMainWin::SaveVpinChanges(success))) = receiver.recv() {
                let toolbar = main_win.main_toolbar();
                let mut pinchanges_ptr = main_win.vpin_requested_changes_table();

                let qb = toolbar.query_btn();
                let mut query_btn = qb.as_mut_ref().expect("unable to convert to mut");
                if success {
                    pinchanges_ptr.clear();
                    pinchanges_ptr.set_row_count(0);
                    let mut mb = QMessageBox::new();
                    // re-execute query
                    query_btn.click();
                    mb.set_text(&qs("Success"));
                    mb.exec();
                //todo - reset color of query
                } else {
                    let mut mb = QMessageBox::new();
                    mb.set_text(&qs("Error Occured"));
                    mb.set_detailed_text(&qs(format!("{:#?}", success)));
                    mb.exec();
                }
            } else {
                log::error!("PackagesTree::GetHistoryRevisions IMsg does not match event state");
            }
        }
    }
}
