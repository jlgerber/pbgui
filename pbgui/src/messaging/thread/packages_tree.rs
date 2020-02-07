use super::*;

/// perform a submatch against the OVpinDialog msg
pub(crate) fn match_packages_tree(
    msg: OPackagesTree,
    db: &mut packybara::db::packrat::PackratDb,
    conductor: &mut qt_thread_conductor::conductor::Conductor<Event>,
    sender: &Sender<IMsg>,
) {
    match msg {
        OPackagesTree::GetPackages => {
            let packages = match db.find_all_packages().query() {
                Ok(packages) => packages,
                Err(err) => {
                    sender
                        .send(IMsg::Error(format!(
                            "Unable to get packages from db: {}",
                            err
                        )))
                        .expect("unable to send error msg");
                    conductor.signal(Event::Error);
                    return;
                }
            };
            let packages = packages
                .into_iter()
                .map(|mut x| std::mem::replace(&mut x.name, String::new()))
                .collect::<Vec<_>>();
            sender
                .send(IPackagesTree::Packages(packages).to_imsg())
                .expect("unable to send packages");
            conductor.signal(PackagesTree::GetPackages.to_event());
        }

        OPackagesTree::GetSites => {
            let sites = match db.find_all_sites().query() {
                Ok(sites) => sites,
                Err(e) => {
                    sender
                        .send(IMsg::Error(format!("Unable to get sites from db: {}", e)))
                        .expect("unable to send error msg");
                    conductor.signal(Event::Error);
                    return;
                }
            };
            // we use std::mem::replace because this should be a bit more efficient
            // than clone, and certainly more
            let sites = sites
                .into_iter()
                .map(|mut x| std::mem::replace(&mut x.name, String::new()))
                .collect::<Vec<_>>();
            sender
                .send(IPackagesTree::Sites(sites).to_imsg())
                .expect("unable to send sites");
            conductor.signal(PackagesTree::GetSites.to_event());
        }

        OPackagesTree::GetPackageDists {
            package,
            package_row,
        } => {
            let results = match db.find_all_distributions().package(&package).query() {
                Ok(dists) => dists,
                Err(e) => {
                    sender
                        .send(IMsg::Error(format!(
                            "Unable to get distributions from db: {}",
                            e
                        )))
                        .expect("unable to send error msg");
                    conductor.signal(Event::Error);
                    return;
                }
            };
            let results = results
                .iter()
                .map(|s| s.version.as_str().to_string())
                .collect::<Vec<_>>();
            // we use std::mem::replace because this should be a bit more efficient
            // // than clone, and certainly more
            // let sites = sites
            //     .into_iter()
            //     .map(|mut x| std::mem::replace(&mut x.name, String::new()))
            //     .collect::<Vec<_>>();
            sender
                .send(
                    IPackagesTree::DistsForPackage {
                        dists: results,
                        row: package_row,
                    }
                    .to_imsg(),
                )
                .expect("unable to send distributions");
            conductor.signal(PackagesTree::GetDistsForPackage.to_event());
        }
    }
}
