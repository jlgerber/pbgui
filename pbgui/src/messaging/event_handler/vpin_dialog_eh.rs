//! Provides a function that processes `messaging::event::VpinDialog` events, updating the ui state or
//! logging errors
use super::*;
use crate::change_type::Change;
use crate::components::versionpin_row::*;
use crate::traits::RowSetterTrait;
use qt_widgets::cpp_core::CppBox;
use rustqt_utils::ToQString;

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
                    // set a versionpin_table row per change
                    // set a versionpin_changes_table row per change
                    // set the cache
                    let mut vpin_tablewidget_ptr = main_win.vpin_table();

                    vpin_tablewidget_ptr.set_sorting_enabled(false);
                    let rcount = vpin_tablewidget_ptr.row_count();
                    let mut cnt = rcount;
                    vpin_tablewidget_ptr.set_row_count(rcount + changes.len() as i32);
                    for change in changes {
                        let id = cache.next_fake_row_id();

                        if let Change::AddDistribution {
                            distribution,
                            level,
                            role,
                            platform,
                            site,
                        } = change
                        {
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
                            versionpin_row.set_table_row(&mut vpin_tablewidget_ptr, cnt);
                            cnt += 1;
                        };
                    }
                    vpin_tablewidget_ptr.set_sorting_enabled(true);
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
