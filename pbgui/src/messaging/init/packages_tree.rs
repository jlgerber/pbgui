use crate::messaging::outgoing::opackages_tree::OPackagesTree;
use crate::messaging::OMsg;
use crate::messaging::Sender;

pub fn init(to_thread_sender: Sender<OMsg>) {
    to_thread_sender
        .send(OMsg::PackagesTree(OPackagesTree::GetPackages))
        .expect("unable to get packages");
    to_thread_sender
        .send(OMsg::PackagesTree(OPackagesTree::GetSites))
        .expect("unable to get sites");
}
