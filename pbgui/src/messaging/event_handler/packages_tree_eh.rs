use super::*;
use crate::components::dist_tree::tree;
use crate::messaging::{event::packages_tree::PackagesTree, IPackagesTree};
use std::rc::Rc;

pub fn match_packages_tree<'a>(
    event: PackagesTree,
    // tree: Rc<RefCell<tree::DistributionTreeView<'a>>>,
    tree: Rc<tree::DistributionTreeView<'a>>,
    receiver: &Receiver<IMsg>,
) {
    match event {
        PackagesTree::GetPackages => {
            if let Ok(IMsg::PackagesTree(IPackagesTree::Packages(packages))) = receiver.recv() {
                let packages_ref = packages.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                tree.set_packages(packages_ref);
            } else {
                log::error!("PackagesTree::GetPackages IMsg does not match event state");
            }
        }
        PackagesTree::GetSites => {
            if let Ok(IMsg::PackagesTree(IPackagesTree::Sites(sites))) = receiver.recv() {
                let sites_ref = sites.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                //let mut tree = tree;
                tree.set_sites(sites_ref, "portland"); // TODO: pass current site in IPackagesTree::Sites IMsg
            } else {
                log::error!("IMsg does not have Sites")
            }
        }
        PackagesTree::GetDistsForPackage => {
            if let Ok(IMsg::PackagesTree(IPackagesTree::DistsForPackage { dists, row })) =
                receiver.recv()
            {
                let dists_ref = dists.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                //tree.borrow().set_sites(sites_ref, "portland"); // TODO: pass current site in IPackagesTree::Sites IMsg
                if dists.len() > 0 {
                    unsafe {
                        let model = tree.model();
                        let idx = model.index_2a(row, 0);
                        let item = model.item_from_index(idx.as_ref());
                        let mut model = model; //tree.borrow_mut().model();
                        model.remove_rows_3a(0, 1, idx.as_ref());
                        let inner = tree.inner();
                        inner.set_children(item, dists_ref, true);
                    }
                }
            } else {
                log::error!("IMsg does not have dists")
            }
        }
    }
}
