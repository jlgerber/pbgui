use crate::outgoing::opackages_tree::OPackagesTree;
use crate::OMsg;
use crate::Sender;

pub fn init(to_thread_sender: Sender<OMsg>) {
    to_thread_sender
        .send(OMsg::PackagesTree(OPackagesTree::GetPackages))
        .expect("unable to get packages");
    to_thread_sender
        .send(OMsg::PackagesTree(OPackagesTree::GetSites))
        .expect("unable to get sites");
}
