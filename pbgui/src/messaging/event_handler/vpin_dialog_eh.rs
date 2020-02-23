//! Provides a function that processes `messaging::event::VpinDialog` events, updating the ui state or
//! logging errors
use super::*;

pub fn match_vpin_dialog<'a>(
    event: VpinDialog,
    dialog: Rc<vpin_dialog::VpinDialog<'a>>,
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
            if let Ok(IMsg::VpinDialog(IVpinDialog::SetVpin(bool))) = receiver.recv() {
                // TODO
                println!("bool {}", bool);
            } else {
                log::error!("IMsg does not have Vpin");
            }
        }
    }
}
