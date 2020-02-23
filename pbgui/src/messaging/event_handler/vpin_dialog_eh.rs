//! Provides a function that processes `messaging::event::VpinDialog` events, updating the ui state or
//! logging errors
use super::*;
use crate::change_type::{Change, ChangeType};
use crate::components::{versionpin_changes_row::*, versionpin_row::*};
use crate::traits::RowSetterTrait;
use qt_widgets::cpp_core::CppBox;
use rustqt_utils::{qs, ToQString};

pub fn match_vpin_dialog<'a>(
    event: VpinDialog,
    dialog: Rc<vpin_dialog::VpinDialog<'a>>,
    main_win: Rc<InnerMainWindow<'a>>,
    receiver: &Receiver<IMsg>,
) {
    match event {
        VpinDialog::UpdateSites => {
            if let Ok(IMsg::VpinDialog(IVpinDialog::Sites(sites))) = receiver.recv() {
                let sites_ref = sites.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                dialog.set_sites(sites_ref);
            } else {
                log::error!("Event::UpdateSites IMsg does not match event state");
            }
        }
        VpinDialog::UpdateRoles => {
            if let Ok(IMsg::VpinDialog(IVpinDialog::Roles(roles))) = receiver.recv() {
                let roles_ref = roles.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                dialog.set_roles(roles_ref);
            } else {
                log::error!("IMsg does not have Roles")
            }
        }
        VpinDialog::UpdateLevels => {
            if let Ok(IMsg::VpinDialog(IVpinDialog::Levels(level_map))) = receiver.recv() {
                dialog.set_levels(level_map);
            } else {
                log::error!("IMsg does not have LevelMap");
            }
        }
        VpinDialog::SetVpin => {
            if let Ok(IMsg::VpinDialog(IVpinDialog::SetVpin(changes))) = receiver.recv() {
                // TODO
                println!(
                    "from vpin dialog event handler back in ui thread, changes:  {:#?}",
                    changes
                );
                // get cache
                let cache = main_win.cache();
                unsafe {
                    // we have to
                    // set a versionpin_table row per change X
                    // set a versionpin_changes_table row per change
                    // set the cache
                    let mut versionpin_table = main_win.vpin_table();
                    let mut pinchanges_ptr = main_win.vpin_requested_changes_table();

                    versionpin_table.set_sorting_enabled(false);
                    let rcount = versionpin_table.row_count();
                    let changes_row_count = pinchanges_ptr.row_count();
                    println!("initial row count: {}", changes_row_count);
                    let mut cnt = 0;
                    versionpin_table.set_row_count(rcount + changes.len() as i32);
                    pinchanges_ptr.set_row_count(changes_row_count + changes.len() as i32);
                    for change in changes {
                        let id = cache.next_fake_row_id();

                        if let Change::AddDistribution {
                            ref distribution,
                            ref level,
                            ref role,
                            ref platform,
                            ref site,
                        } = change
                        {
                            let version = distribution.split("-").skip(1).next().unwrap();
                            let versionpin_row = VersionPinRow::<CppBox<QString>>::new(
                                id,
                                id,
                                id,
                                distribution.to_qstring(),
                                level.to_qstring(),
                                role.to_qstring(),
                                platform.to_qstring(),
                                site.to_qstring(),
                                0,
                            );
                            versionpin_row.set_table_row(&mut versionpin_table, rcount + cnt);
                            let vpc_row = VersionPinChangesRow::<CppBox<QString>>::new(
                                ChangeType::AddDistribution,
                                versionpin_row.pkgcoord(),
                                qs(""),
                                qs(version),
                            );

                            vpc_row.set_table_row(&mut pinchanges_ptr, changes_row_count + cnt);
                            println!("changes row count: {}", changes_row_count + cnt);
                            let idx = cache.row_count();
                            // cache the index of the change (idx) by the pkgcoord's db id.
                            cache.cache_dist(id, idx);
                            // cache the change
                            cache.cache_change(change);

                            cnt += 1;
                        };
                    }
                    versionpin_table.set_sorting_enabled(true);
                }

            // get
            } else {
                log::error!("IMsg does not have Vpin");
            }
        }
    }
}
/*
id: IdType,
        dist_id: IdType,
        pkgcoord_id: IdType,
        distribution: CppBox<QString>,
        level: CppBox<QString>,
        role: CppBox<QString>,
        platform: CppBox<QString>,
        site: CppBox<QString>,
        withs: i32,
*/
/*
 distribution: String,
        level: String,
        role: String,
        platform: String,
        site: String,
*/
