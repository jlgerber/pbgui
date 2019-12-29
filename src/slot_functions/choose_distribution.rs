use crate::cache::PinChangesCache;
use crate::change_type::{Change, ChangeType};
use crate::constants::*;
pub use crate::utility::qs;
pub use crate::ClientProxy;
use log;
use packybara::packrat::PackratDb;
use packybara::types::IdType;
use qt_core::{QString, QVariant};
use qt_gui::{QBrush, QColor};
use qt_thread_conductor::traits::ToQString;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr, Ref /*Ptr,*/},
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
use packybara::db::find_all::distributions::FindAllDistributionsRow;

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
        if !validate_ptrs(&vpin_tablewidget_ptr, &root_widget_ptr, &pinchanges_ptr) {
            return;
        }
        // all we need is the package... perhaps we should change the vtable
        // TODO: show package & version as separate columns in versionpin table.
        let mut distribution = vpin_tablewidget_ptr.item(r, COL_DISTRIBUTION);
        let orig_vpin_table_distribution = distribution.text().to_std_string();
        // split up the distribution into the package name
        // and the version
        let (package, version) = if let &[package, version] =
            &*orig_vpin_table_distribution.split("-").collect::<Vec<_>>()
        {
            (package, version)
        } else {
            panic!("unable to extract package and version from row");
        };
        let client = ClientProxy::connect()
            .expect("unable to unwrap clientproxy connection in choose distributions");
        //
        // retrieve distributions for the current package from the database
        // in order to present choices to the user
        let mut packratdb = PackratDb::new(client);
        let results = packratdb
            .find_all_distributions()
            .package(package)
            .query()
            .expect("unable to unwrap query of distributions");
        let (versions_list, idx, dist_versions) = build_qstring_list_and_map(version, results);
        let mut ok_or_cancel = false;
        let ok_or_cancel_ptr = MutPtr::from_raw(&mut ok_or_cancel);
        // Get New version by popping up a Dialog
        let new_version = QInputDialog::get_item_7a(
            root_widget_ptr,
            &qs("Pick Version"),
            &qs(package),
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
            if orig_vpin_table_distribution == new_value {
                log::info!("new value and old value match. Skipping");
                return;
            }
            // replace with full get_row
            let (level, role, platform, site, vpin_id, _dist_id, pkgcoord_id) =
                get_coords_from_row(&mut vpin_tablewidget_ptr, r);
            // cache the change. we will use this later to update the db. The rest of
            // the code is for updating the ui
            let new_value_qstr = QString::from_std_str(new_value);
            // build up new string
            distribution.set_text(&new_value_qstr);
            // let change_qstr = version_change::build_changestr(
            //     qs(package).as_ref(),
            //     // original version
            //     qs(version).as_ref(),
            //     // new version
            //     new_version.as_ref(),
            //     level.as_ref(),
            //     role.as_ref(),
            //     platform.as_ref(),
            //     site.as_ref(),
            // );

            if pinchange_cache.has_key(pkgcoord_id) {
                let original_version = pinchange_cache
                    .orig_version_for(vpin_id)
                    .expect("failed to retrieve original version from cache.");
                let change_qstr = version_change::build_changestr(
                    qs(package).as_ref(),
                    // original version
                    qs(original_version).as_ref(),
                    // new version
                    new_version.as_ref(),
                    level.as_ref(),
                    role.as_ref(),
                    platform.as_ref(),
                    site.as_ref(),
                );
                let row = match pinchange_cache.index(pkgcoord_id) {
                    Some(r) => r,
                    None => {
                        log::error!("ERROR: Problem retrieving row from QT");
                        return;
                    }
                };
                log::info!("pinchange cache has pkgcoord {} @ row {}", pkgcoord_id, row);
                if pinchanges_ptr.is_null() {
                    log::error!("pinchanges_ptr is now null");
                    return;
                }

                let mut item = pinchanges_ptr.item(row, COL_PC_DISPLAY);
                if item.is_null() {
                    log::error!("problem retreiving row from pinchanges_ptr using cached row number. item is null");
                    return;
                }
                item.set_text(&change_qstr);
                let change = Change::ChangeDistribution {
                    vpin_id,
                    new_dist_id: *new_dist_id,
                };
                pinchange_cache.cache_change_at(change, row);
            } else {
                let change_qstr = version_change::build_changestr(
                    qs(package).as_ref(),
                    // original version
                    qs(version).as_ref(),
                    // new version
                    new_version.as_ref(),
                    level.as_ref(),
                    role.as_ref(),
                    platform.as_ref(),
                    site.as_ref(),
                );
                pinchange_cache.cache_original_version(vpin_id, version);
                let row_cnt = pinchanges_ptr.row_count() + 1;
                pinchanges_ptr.set_row_count(row_cnt);

                set_pinchange(
                    &mut pinchanges_ptr,
                    row_cnt,
                    ChangeType::ChangeDistribution,
                    vpin_id,
                    *new_dist_id,
                    pkgcoord_id,
                    change_qstr.as_ref(),
                );
                let update_color = qcolor_blue!();
                distribution.set_foreground(&QBrush::from_q_color(update_color.as_ref()));
                distribution.table_widget().clear_selection();
                let idx = pinchange_cache.row_count();
                log::info!("caching pkgcoord_id {} at row {}", pkgcoord_id, idx);
                pinchange_cache.cache_dist(pkgcoord_id, idx);
                let change = Change::ChangeDistribution {
                    vpin_id,
                    new_dist_id: *new_dist_id,
                };
                pinchange_cache.cache_change(change);
                //pinchange_cache.increment_rowcount();
            }
        }
    }
}

// Given
fn package_and_version_from_dist<'a, T>(dist: T) -> (String, String)
where
    T: Into<Ref<QString>>,
{
    unsafe {
        let qstr = dist.into();
        let orig_vpin_table_distribution = qstr.to_std_string();
        if let &[package, version] = &*orig_vpin_table_distribution.split("-").collect::<Vec<_>>() {
            (package.to_string(), version.to_string())
        } else {
            panic!("unable to extract package and version from row");
        }
    }
}
//
mod version_change {
    use super::*;

    #[allow(dead_code)]
    pub(super) fn build_changestr(
        package: Ref<QString>,
        original_version: Ref<QString>,
        new_version: Ref<QString>,
        level: Ref<QString>,
        role: Ref<QString>,
        platform: Ref<QString>,
        site: Ref<QString>,
    ) -> CppBox<QString> {
        unsafe {
            let changestr = qs(
                "%1-%2      ->      %1-%3        (level: %4,  role: %5,  platform: %6,  site: %7)",
            )
            .arg_7_q_string(
                package,
                original_version,
                new_version,
                level,
                role,
                platform,
                site,
            );
            changestr
        }
    }
}

// perform validation
fn validate_ptrs(
    vpin_tablewidget_ptr: &MutPtr<QTableWidget>,
    root_widget_ptr: &MutPtr<QWidget>,
    pinchanges_ptr: &MutPtr<QTableWidget>,
) -> bool {
    if vpin_tablewidget_ptr.is_null() {
        log::error!("vpin_tablewidget_ptr is null");
        return false;
    }
    if root_widget_ptr.is_null() {
        log::error!("root_widget_ptr is null");
        return false;
    }
    if pinchanges_ptr.is_null() {
        log::error!("pinchanges_ptr is null. returning");
        return false;
    }
    true
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
        let pinchanges_item = QTableWidgetItem::from_q_string(changetype.to_qstring().as_ref());
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
