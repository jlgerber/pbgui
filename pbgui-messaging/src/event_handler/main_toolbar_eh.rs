use super::*;
use crate::{event::main_toolbar::MainToolbar, incoming::imain_toolbar::IMainToolbar};
use pbgui_toolbar::toolbar::MainToolbar as MainToolbarUiElem;
use std::rc::Rc;

pub fn match_main_toolbar(
    event: MainToolbar,
    toolbar: Rc<MainToolbarUiElem>,
    receiver: &Receiver<IMsg>,
) {
    match event {
        MainToolbar::GetShows => {
            if let Ok(IMsg::MainToolbar(IMainToolbar::Shows(shows))) = receiver.recv() {
                let shows_ref = shows.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                toolbar.set_level_items(shows_ref);
            } else {
                log::error!("MainToolbar::GetShows IMsg does not match event state");
            }
        }
        MainToolbar::GetRoles => {
            if let Ok(IMsg::MainToolbar(IMainToolbar::Roles(roles))) = receiver.recv() {
                let roles_ref = roles.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                toolbar.set_role_items(roles_ref);
            } else {
                log::error!("MainToolbar::GetRoles IMsg does not match event state");
            }
        }
        MainToolbar::GetPlatforms => {
            if let Ok(IMsg::MainToolbar(IMainToolbar::Platforms(platforms))) = receiver.recv() {
                let platforms_ref = platforms.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                toolbar.set_platform_items(platforms_ref);
            } else {
                log::error!("MainToolbar::GetPlatforms IMsg does not match event state");
            }
        }
        MainToolbar::GetSites => {
            if let Ok(IMsg::MainToolbar(IMainToolbar::Sites(sites))) = receiver.recv() {
                let sites_ref = sites.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                toolbar.set_site_items(sites_ref);
            } else {
                log::error!("MainToolbar::GetSites IMsg does not match event state");
            }
        }
    }
}
