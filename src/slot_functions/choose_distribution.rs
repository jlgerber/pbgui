use crate::constants::*;

pub use crate::ClientProxy;
use packybara::packrat::PackratDb;
use qt_core::QVariant;
use qt_gui::{QBrush, QColor};
use qt_widgets::{
    cpp_core::{CppBox, MutPtr /*Ptr,*/},
    qt_core::QString,
    qt_core::QStringList,
    QInputDialog, QTableWidget, QTableWidgetItem, QWidget,
};
use std::cell::{Cell, RefCell};
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
    usage_ptr: Rc<RefCell<HashMap<i32, i32>>>,
    root_widget_ptr: MutPtr<QWidget>,
    mut pinchanges_ptr: MutPtr<QTableWidget>,
    update_cnt_ptr: Rc<Cell<i32>>,
) {
    unsafe {
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
            println!("ok_or_cancel_ptr is null. Problem on QT side. Returning");
            return;
        }
        if *ok_or_cancel_ptr == false {
            println!("cancelled");
        } else {
            let value = new_version.to_std_string();
            let new_dist_id = match dist_versions.get(value.as_str()) {
                Some(id) => id,
                // TODO: handle this more appropriately
                None => {
                    println!("ERROR: Unable to get dist id.");
                    return;
                }
            };
            let new_value = format!("{}-{}", package, value);
            if orig_text == new_value {
                println!("new value and old value match. Skipping");
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

            if usage_ptr.borrow().contains_key(&dist_id) {
                let row = usage_ptr.borrow();
                let row = match row.get(&dist_id) {
                    Some(r) => r,
                    None => {
                        println!("ERROR: Problem retrieving row from QT");
                        return;
                    }
                };
                let mut item = pinchanges_ptr.item(*row, COL_PC_DISPLAY);
                item.set_text(&orig_qstr);
            } else {
                let row_cnt = pinchanges_ptr.row_count() + 1;
                pinchanges_ptr.set_row_count(row_cnt);
                // VPIN ID
                let mut pinchanges_item = QTableWidgetItem::new();
                let variant = QVariant::from_int(vpin_id);
                pinchanges_item.set_data(
                    2, // EditRole
                    variant.as_ref(),
                );
                pinchanges_ptr.set_item(row_cnt - 1, COL_PC_VPINID, pinchanges_item.into_ptr());
                // DIST ID
                let mut pinchanges_item = QTableWidgetItem::new();
                let variant = QVariant::from_int(*new_dist_id);
                pinchanges_item.set_data(
                    2, // EditRole
                    variant.as_ref(),
                );
                pinchanges_ptr.set_item(row_cnt - 1, COL_PC_DISTID, pinchanges_item.into_ptr());
                // PKGCOORD ID
                let mut pinchanges_item = QTableWidgetItem::new();
                let variant = QVariant::from_int(pkgcoord_id);
                pinchanges_item.set_data(
                    2, // EditRole
                    variant.as_ref(),
                );
                pinchanges_ptr.set_item(row_cnt - 1, COL_PC_PKGCOORDID, pinchanges_item.into_ptr());
                // DISPLAY
                let pinchanges_item = QTableWidgetItem::from_q_string(&orig_qstr);
                pinchanges_ptr.set_item(row_cnt - 1, COL_PC_DISPLAY, pinchanges_item.into_ptr());

                let update_color = qcolor_blue!();
                dist_item.set_foreground(&QBrush::from_q_color(update_color.as_ref()));
                dist_item.table_widget().clear_selection();
                let idx = update_cnt_ptr.get();
                usage_ptr.borrow_mut().insert(dist_id, idx);
                update_cnt_ptr.set(idx + 1);
            }
        }
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
