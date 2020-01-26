use crate::messaging::outgoing::opackage_withs::OPackageWiths;
use crate::messaging::OMsg;
use crate::messaging::Sender;

pub fn init(to_thread_sender: Sender<OMsg>) {
    to_thread_sender
        .send(OMsg::PackageWiths(OPackageWiths::GetPackages))
        .expect("unable to get packages");
}
