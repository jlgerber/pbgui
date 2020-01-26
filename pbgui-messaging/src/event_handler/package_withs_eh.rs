use super::*;
use crate::{event::package_withs::PackageWiths, incoming::ipackage_withs::IPackageWiths};
use pbgui_withs::WithsList;
use std::cell::RefCell;
use std::rc::Rc;

pub fn match_package_withs<'a>(
    event: PackageWiths,
    withs: Rc<RefCell<WithsList<'a>>>,
    receiver: &Receiver<IMsg>,
) {
    match event {
        PackageWiths::GetPackages => {
            if let Ok(IMsg::PackageWiths(IPackageWiths::Packages(packages))) = receiver.recv() {
                let packages_ref = packages.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                withs.borrow().set_cb_items(packages_ref);
            } else {
                log::error!("PackagesTree::GetPackages IMsg does not match event state");
            }
        }
    }
}
