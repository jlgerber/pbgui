use super::*;
use crate::{event::packages_tree::PackagesTree, IPackagesTree};
use pbgui_tree::tree;
use std::cell::RefCell;
use std::rc::Rc;

pub fn match_packages_tree<'a>(
    event: PackagesTree,
    tree: Rc<RefCell<tree::DistributionTreeView<'a>>>,
    receiver: &Receiver<IMsg>,
) {
    match event {
        PackagesTree::GetPackages => {
            if let Ok(IMsg::PackagesTree(IPackagesTree::Packages(packages))) = receiver.recv() {
                let packages_ref = packages.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                tree.borrow().set_packages(packages_ref);
            } else {
                log::error!("PackagesTree::GetPackages IMsg does not match event state");
            }
        }
        PackagesTree::GetSites => {
            if let Ok(IMsg::PackagesTree(IPackagesTree::Sites(sites))) = receiver.recv() {
                let sites_ref = sites.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                tree.borrow().set_sites(sites_ref, "portland"); // TODO: pass current site in IPackagesTree::Sites IMsg
            } else {
                log::error!("IMsg does not have Sites")
            }
        }
    }
}
