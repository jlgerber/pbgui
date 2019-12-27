use crate::constants::*;

use crate::cache::PinChangesCache;
use crate::change_type::ChangeType;
pub use crate::ClientProxy;
use log;
use packybara::packrat::PackratDb;
use qt_core::QVariant;
use qt_gui::{QBrush, QColor};
use qt_widgets::{
    cpp_core::{CppBox, MutPtr, Ref /*Ptr,*/},
    qt_core::QString,
    qt_core::QStringList,
    QInputDialog, QTableWidget, QTableWidgetItem, QWidget,
};
use std::collections::HashMap;
use std::rc::Rc;
macro_rules! qcolor_blue {
    () => {
        QColor::from_rgb_3a(100, 150, 255)
    };
}

//------------------------------------//
// choose_alternative_distribution    //
//------------------------------------//
// button double click Slot delegates //
// the work to this function          //
//------------------------------------//
pub fn choose_alternative_distribution(
    r: i32,
    mut vpin_tablewidget_ptr: MutPtr<QTableWidget>,
    root_widget_ptr: MutPtr<QWidget>,
    mut pinchanges_ptr: MutPtr<QTableWidget>,
    pinchange_cache: Rc<PinChangesCache>,
) {
    unsafe {
        // check all ptrs
        if vpin_tablewidget_ptr.is_null() {
            log::error!("vpin_tablewidget_ptr is null");
            return;
        }
        if root_widget_ptr.is_null() {
            log::error!("root_widget_ptr is null");
            return;
        }
        if pinchanges_ptr.is_null() {
            log::error!("pinchanges_ptr is null. returning");
            return;
        }
        let mut dist_item = vpin_tablewidget_ptr.item(r, COL_DISTRIBUTION);
        let mut orig_qstr = dist_item.text();
        let orig_text = orig_qstr.to_std_string();
        // split up the distribution into the package name
        // and the version
        let (package, version) =
            if let &[package, version] = &*orig_text.split("-").collect::<Vec<_>>() {
                (package, version)
            } else {
                panic!("unable to extract package and version from row");
            };
        let client = ClientProxy::connect()
            .expect("unable to unwrap clientproxy connection in choose distributions");
        let mut packratdb = PackratDb::new(client);
        let results = packratdb
            .find_all_distributions()
            .package(package)
            .query()
            .expect("unable to unwrap query of distributions");
        let mut qsl = QStringList::new();
        let mut idx = 0;
        let mut cnt = 0;
        let mut dist_versions = HashMap::new();
        for r in results {
            if r.version == version {
                idx = cnt;
            }
            cnt += 1;
            dist_versions.insert(r.version.clone(), r.id);
            qsl.append_q_string(&QString::from_std_str(r.version));
        }
        let mut ok_or_cancel = false;
        let ok_or_cancel_ptr = MutPtr::from_raw(&mut ok_or_cancel);
        // Get New version by popping up a Dialog
        let new_version = QInputDialog::get_item_7a(
            root_widget_ptr,
            &QString::from_std_str("Pick Version"),
            &QString::from_std_str(package),
            &qsl,
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
            let value = new_version.to_std_string();
            let new_dist_id = match dist_versions.get(value.as_str()) {
                Some(id) => id,
                // TODO: handle this more appropriately
                None => {
                    log::error!("ERROR: Unable to get dist id.");
                    return;
                }
            };
            let new_value = format!("{}-{}", package, value);
            if orig_text == new_value {
                log::info!("new value and old value match. Skipping");
                return;
            }
            let (level, role, platform, site, vpin_id, dist_id, pkgcoord_id) =
                get_coords_from_row(&mut vpin_tablewidget_ptr, r);
            let new_value_qstr = QString::from_std_str(new_value);
            // build up new string
            dist_item.set_text(&new_value_qstr);
            orig_qstr.append_q_string(&QString::from_std_str("   ->   "));
            orig_qstr.append_q_string(&new_value_qstr);
            orig_qstr.append_q_string(&QString::from_std_str(format!(
                "     ({}, {}, {}, {})     distribution id: {}     pkgcoord id: {}",
                level.to_std_string(),
                role.to_std_string(),
                platform.to_std_string(),
                site.to_std_string(),
                dist_id,
                pkgcoord_id
            )));

            if pinchange_cache.has_key(pkgcoord_id) {
                let row = match pinchange_cache.index(pkgcoord_id) {
                    Some(r) => r,
                    None => {
                        log::error!("ERROR: Problem retrieving row from QT");
                        return;
                    }
                };
                if pinchanges_ptr.is_null() {
                    log::error!("pinchanges_ptr is now null");
                    return;
                }

                let mut item = pinchanges_ptr.item(row, COL_PC_DISPLAY);
                if item.is_null() {
                    log::error!("problem retreiving row from pinchanges_ptr using cached row number. item is null");
                    return;
                }
                item.set_text(&orig_qstr);
            } else {
                let row_cnt = pinchanges_ptr.row_count() + 1;
                pinchanges_ptr.set_row_count(row_cnt);

                set_pinchange(
                    &mut pinchanges_ptr,
                    row_cnt,
                    ChangeType::Distribution,
                    vpin_id,
                    *new_dist_id,
                    pkgcoord_id,
                    orig_qstr.as_ref(),
                );
                let update_color = qcolor_blue!();
                dist_item.set_foreground(&QBrush::from_q_color(update_color.as_ref()));
                dist_item.table_widget().clear_selection();
                let idx = pinchange_cache.row_count();
                pinchange_cache.cache_dist(pkgcoord_id, idx);
                pinchange_cache.increment_rowcount();
            }
        }
    }
}

// insert a row into teh pinchanges table
fn set_pinchange(
    pinchanges_table: &mut MutPtr<QTableWidget>,
    row_cnt: i32,
    changetype: ChangeType,
    vpin_id: i32,
    dist_id: i32,
    pkgcoord_id: i32,
    display: Ref<QString>,
) {
    unsafe {
        // CHANGETYPE
        let mut pinchanges_item = QTableWidgetItem::new();
        let dist_idx: i32 = changetype.into();
        let variant = QVariant::from_int(dist_idx);
        pinchanges_item.set_data(
            2, // EditRole
            variant.as_ref(),
        );
        pinchanges_table.set_item(row_cnt - 1, COL_PC_CHANGETYPE, pinchanges_item.into_ptr());
        // VPIN ID
        let mut pinchanges_item = QTableWidgetItem::new();
        let variant = QVariant::from_int(vpin_id);
        pinchanges_item.set_data(
            2, // EditRole
            variant.as_ref(),
        );
        pinchanges_table.set_item(row_cnt - 1, COL_PC_VPINID, pinchanges_item.into_ptr());
        // DIST ID
        let mut pinchanges_item = QTableWidgetItem::new();
        let variant = QVariant::from_int(dist_id);
        pinchanges_item.set_data(
            2, // EditRole
            variant.as_ref(),
        );
        pinchanges_table.set_item(row_cnt - 1, COL_PC_DISTID, pinchanges_item.into_ptr());
        // PKGCOORD ID
        let mut pinchanges_item = QTableWidgetItem::new();
        let variant = QVariant::from_int(pkgcoord_id);
        pinchanges_item.set_data(
            2, // EditRole
            variant.as_ref(),
        );
        pinchanges_table.set_item(row_cnt - 1, COL_PC_PKGCOORDID, pinchanges_item.into_ptr());
        // DISPLAY
        let pinchanges_item = QTableWidgetItem::from_q_string(display);
        pinchanges_table.set_item(row_cnt - 1, COL_PC_DISPLAY, pinchanges_item.into_ptr());
    }
}

unsafe fn get_coords_from_row(
    row_widget: &mut MutPtr<QTableWidget>,
    row: i32,
) -> (
    CppBox<QString>,
    CppBox<QString>,
    CppBox<QString>,
    CppBox<QString>,
    i32,
    i32,
    i32,
) {
    //level
    let level = row_widget.item(row, COL_LEVEL).text();
    let role = row_widget.item(row, COL_ROLE).text();
    let platform = row_widget.item(row, COL_PLATFORM).text();
    let site = row_widget.item(row, COL_SITE).text();
    let vpin_id = row_widget.item(row, COL_ID).data(2);
    let dist_id = row_widget.item(row, COL_DISTRIBUTION_ID).data(2);
    let pkgcoord_id = row_widget.item(row, COL_PKGCOORD_ID).data(2);

    (
        level,
        role,
        platform,
        site,
        vpin_id.to_int_0a(),
        dist_id.to_int_0a(),
        pkgcoord_id.to_int_0a(),
    )
}
