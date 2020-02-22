//! Provides a function that processes `messaging::event::MainToolbar` events, updating the ui state or
//! logging errors
use super::*;
use crate::messaging::{event::main_toolbar::MainToolbar, incoming::imain_toolbar::IMainToolbar};
use pbgui_toolbar::toolbar::MainToolbar as MainToolbarUiElem;
use std::rc::Rc;

/// Function to process the MainToolbar events, updating the supplied toolbar ui element. We
/// match on the event, pull data from the secondary thread from the provided channel, and
/// updae the supplied toolbar ui element in response, depending upon the event.
pub fn match_main_toolbar(
    event: MainToolbar,
    toolbar: Rc<MainToolbarUiElem>,
    receiver: &Receiver<IMsg>,
) {
    match event {
        // Update the toolbar's show combobox with the supplied list of shows
        MainToolbar::GetShows => {
            if let Ok(IMsg::MainToolbar(IMainToolbar::Shows(shows))) = receiver.recv() {
                let shows_ref = shows.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                toolbar.set_level_items(shows_ref);
            } else {
                log::error!("MainToolbar::GetShows IMsg does not match event state");
            }
        }
        // Update the role combobox with a list of shows supplied via the receiver
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
