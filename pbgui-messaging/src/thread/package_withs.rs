use super::*;

/// perform a submatch against the OVpinDialog msg
pub(crate) fn match_package_withs(
    msg: OPackageWiths,
    db: &mut packybara::db::packrat::PackratDb,
    conductor: &mut qt_thread_conductor::conductor::Conductor<Event>,
    sender: &Sender<IMsg>,
) {
    match msg {
        OPackageWiths::GetPackages => {
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
                .send(IPackageWiths::Packages(packages).to_imsg())
                .expect("unable to send packages");
            conductor.signal(PackageWiths::GetPackages.to_event());
        }
    }
}
