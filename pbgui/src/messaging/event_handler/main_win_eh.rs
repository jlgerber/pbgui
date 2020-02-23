//! Provides a function that processes `messaging::event::MainWin` events, updating the ui state or
//! logging errors
use super::*;
use crate::change_type::{Change, ChangeType};
use crate::versionpin_changes_row::VersionPinChangesRow;
use crate::versionpin_row::VersionPinRow;
use crate::{
    constants::*,
    main_window::InnerMainWindow,
    messaging::{event::main_win::MainWin, incoming::imain_win::IMainWin},
    traits::{RowSetterTrait, RowTrait},
    utility::{qs, update_row, RowType},
};
use packybara::db::find_all::distributions::FindAllDistributionsRow;
use packybara::types::IdType;
use qt_core::{QString, QVariant};
use qt_gui::{QBrush, QColor};
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    qt_core::QStringList,
    QInputDialog, QMessageBox, QTableWidgetItem,
};
use std::collections::HashMap;
use std::rc::Rc;

macro_rules! qcolor_blue {
    () => {
        QColor::from_rgb_3a(100, 150, 255)
    };
}

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
        MainWin::ChooseDistribution => {
            if let Ok(IMsg::MainWin(IMainWin::ChooseDistribution {
                distributions,
                package,
                version,
                row, // row in the versionpin table
            })) = receiver.recv()
            {
                let pinchange_cache = main_win.cache();
                let root_widget_ptr = main_win.main_widget();
                let vpin_table = main_win.vpin_table();
                let mut distribution = vpin_table.item(row, COL_DISTRIBUTION);
                let mut pinchanges_ptr = main_win.vpin_requested_changes_table();

                let (versions_list, idx, dist_versions) =
                    build_qstring_list_and_map(version.as_str(), distributions);
                let mut ok_or_cancel = false;
                let ok_or_cancel_ptr = MutPtr::from_raw(&mut ok_or_cancel);
                // Get New version by popping up a Dialog
                let new_version = QInputDialog::get_item_7a(
                    root_widget_ptr,
                    &qs("Pick Version"),
                    &qs(package.as_str()),
                    &versions_list,
                    idx,
                    false,
                    ok_or_cancel_ptr,
                );
                if ok_or_cancel_ptr.is_null() {
                    log::error!("ok_or_cancel_ptr is null. Problem on QT side. Returning");
                    return;
                }
                if *ok_or_cancel_ptr == false {
                    log::info!("cancelled");
                } else {
                    let new_version_string = new_version.to_std_string();
                    let new_dist_id = match dist_versions.get(new_version_string.as_str()) {
                        Some(id) => id,
                        // TODO: handle this more appropriately
                        None => {
                            log::error!("ERROR: Unable to get dist id.");
                            return;
                        }
                    };
                    let orig_vpin_table_distribution = format!("{}-{}", package, version.as_str());
                    let new_distribution = format!("{}-{}", package, new_version_string);
                    if orig_vpin_table_distribution == new_distribution {
                        log::info!("new value and old value match. Skipping");
                        return;
                    }
                    // retrieve the value of the versionpin row from the versionpin table
                    let vpin_row = VersionPinRow::<CppBox<QString>>::from_table_at_row(
                        &vpin_table, //&vpin_tablewidget_ptr,
                        row,
                    )
                    .ok_or(false)
                    .expect("unable to retrieve the versionpin row from table");

                    // cache the change. we will use this later to update the db. The rest of
                    // the code is for updating the ui
                    let new_value_qstr = QString::from_std_str(new_distribution);
                    // build up new string
                    distribution.set_text(&new_value_qstr);
                    // if we arleady have the key in the pinchange table, we update the "to" value and
                    // leave the from value alone, as it is the original state of the table prior to our
                    // proposed changes - which we have not confirmed by hitting save yet.

                    if pinchange_cache.has_key(vpin_row.pkgcoord_id) {
                        let row = match pinchange_cache.index(vpin_row.pkgcoord_id) {
                            Some(r) => r,
                            None => {
                                log::error!("ERROR: Problem retrieving row from QT");
                                return;
                            }
                        };
                        let mut item = pinchanges_ptr.item(row, COL_PC_NEW_VALUE);
                        if item.is_null() {
                            log::error!("problem retreiving row from pinchanges_ptr using cached row number. item is null");
                            return;
                        }
                        item.set_text(&new_version);
                        let change = Change::ChangeDistribution {
                            vpin_id: vpin_row.id,
                            new_dist_id: *new_dist_id,
                        };
                        pinchange_cache.cache_change_at(change, row);
                    } else {
                        // this is a new modification for this row of the vpin table
                        let vpc_row = VersionPinChangesRow::<CppBox<QString>>::new(
                            ChangeType::ChangeDistribution,
                            vpin_row.pkgcoord(),
                            qs(version.as_str()),
                            new_version,
                        );
                        pinchange_cache.cache_original_version(vpin_row.id, version);
                        let row_cnt = pinchanges_ptr.row_count() + 1;
                        // increase the row count by 1 in the pinchanges table
                        pinchanges_ptr.set_row_count(row_cnt);
                        // cache the VersionPinChangesRow instance at the next row in the table
                        vpc_row.set_table_row(&mut pinchanges_ptr, row_cnt - 1);
                        let update_color = qcolor_blue!();
                        distribution.set_foreground(&QBrush::from_q_color(update_color.as_ref()));
                        distribution.table_widget().clear_selection();
                        let idx = pinchange_cache.row_count();
                        // cache the index of the change (idx) by the pkgcoord's db id.
                        pinchange_cache.cache_dist(vpin_row.pkgcoord_id, idx);
                        let change = Change::ChangeDistribution {
                            vpin_id: vpin_row.id,
                            new_dist_id: *new_dist_id,
                        };
                        // cache the change
                        pinchange_cache.cache_change(change);
                    }
                }
            } else {
                log::error!("PackagesTree::ChooseDistribution IMsg does not match event state");
            }
        }
        MainWin::SavePackagesXml => {
            // TODO
            if let Ok(IMsg::MainWin(IMainWin::SavePackagesXml(_success))) = receiver.recv() {
                log::info!("wrote out packages.xml to disk");
            } else {
                log::error!("Unable to save packages.xml to disk");
            }
        }
    }
}

// Construct a qstringlist of versions, identify the index of the currently selected version,
// and provide a hasmap mapping the version to the id
fn build_qstring_list_and_map(
    version: &str,
    results: Vec<FindAllDistributionsRow>,
) -> (CppBox<QStringList>, i32, HashMap<String, IdType>) {
    unsafe {
        let mut versions_list = QStringList::new();
        let mut idx = 0;
        let mut cnt = 0;
        let mut dist_versions = HashMap::new();
        for r in results {
            if r.version == version {
                idx = cnt;
            }
            cnt += 1;
            dist_versions.insert(r.version.clone(), r.id);
            versions_list.append_q_string(&QString::from_std_str(r.version));
        }
        (versions_list, idx, dist_versions)
    }
}
