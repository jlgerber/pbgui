use crate::outgoing::omain_toolbar::OMainToolbar;
use crate::OMsg;
use crate::Sender;

pub fn init(to_thread_sender: Sender<OMsg>) {
    to_thread_sender
        .send(OMsg::MainToolbar(OMainToolbar::GetShows))
        .expect("unable to get shows");
    to_thread_sender
        .send(OMsg::MainToolbar(OMainToolbar::GetRoles))
        .expect("unable to get roles");
    to_thread_sender
        .send(OMsg::MainToolbar(OMainToolbar::GetPlatforms))
        .expect("unable to get platforms");
    to_thread_sender
        .send(OMsg::MainToolbar(OMainToolbar::GetSites))
        .expect("unable to get sites");
}
