use super::*;

// filter using is any
fn is_not_any(item: &str) -> Option<&str> {
    if item == "any" {
        None
    } else {
        Some(item)
    }
}

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
        OPackagesTree::GetDistPlatforms {
            package,
            version,
            package_row,
            dist_row,
        } => {
            // quiet down compiler whining about unused vars. we will need these later
            let _package = package;
            let _version = version;
            // TODO: we need ot add a call in packybara that will return the
            // platforms installed for a given distribution at a given location
            // and we will need to add site into the GetDistPlatforms
            let results = match db.find_all_platforms().query() {
                Ok(dists) => dists,
                Err(e) => {
                    sender
                        .send(IMsg::Error(format!(
                            "Unable to get platforms from db: {}",
                            e
                        )))
                        .expect("unable to send error msg");
                    conductor.signal(Event::Error);
                    return;
                }
            };
            let results = results
                .iter()
                .filter_map(|s| is_not_any(s.name.as_str()))
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            sender
                .send(
                    IPackagesTree::PlatformsForDist {
                        platforms: results,
                        package_row,
                        dist_row,
                    }
                    .to_imsg(),
                )
                .expect("unable to send distributions");
            conductor.signal(PackagesTree::GetPlatformsForDist.to_event());
        }
    }
}
