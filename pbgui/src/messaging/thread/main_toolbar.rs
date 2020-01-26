use super::*;

pub(crate) fn match_main_toolbar(
    msg: OMainToolbar,
    db: &mut packybara::db::packrat::PackratDb,
    conductor: &mut qt_thread_conductor::conductor::Conductor<Event>,
    sender: &Sender<IMsg>,
) {
    match msg {
        OMainToolbar::GetShows => {
            let shows = match db.find_all_levels().depth(1).query() {
                Ok(shows) => shows,
                Err(err) => {
                    sender
                        .send(IMsg::Error(format!("Unable to get shows from db: {}", err)))
                        .expect("unable to send error msg");
                    conductor.signal(Event::Error);
                    return;
                }
            };
            let mut results = vec!["facility".to_string()];
            shows
                .into_iter()
                .map(|mut show| std::mem::replace(&mut show.show, String::new()))
                .for_each(|show| results.push(show));
            sender
                .send(IMainToolbar::Shows(results).to_imsg())
                .expect("unable to send shows");
            conductor.signal(MainToolbar::GetShows.to_event());
        }
        OMainToolbar::GetRoles => {
            let roles = match db.find_all_roles().query() {
                Ok(roles) => roles,
                Err(err) => {
                    sender
                        .send(IMsg::Error(format!("Unable to get roles from db: {}", err)))
                        .expect("unable to send error msg");
                    conductor.signal(Event::Error);
                    return;
                }
            };
            let roles = roles
                .into_iter()
                .map(|mut role| std::mem::replace(&mut role.role, String::new()))
                .collect::<Vec<_>>();
            sender
                .send(IMainToolbar::Roles(roles).to_imsg())
                .expect("unable to send roles");
            conductor.signal(MainToolbar::GetRoles.to_event());
        }
        OMainToolbar::GetPlatforms => {
            let platforms = match db.find_all_platforms().query() {
                Ok(platforms) => platforms,
                Err(err) => {
                    sender
                        .send(IMsg::Error(format!(
                            "Unable to get platforms from db: {}",
                            err
                        )))
                        .expect("unable to send error msg");
                    conductor.signal(Event::Error);
                    return;
                }
            };
            let platforms = platforms
                .into_iter()
                .map(|mut platform| std::mem::replace(&mut platform.name, String::new()))
                .collect::<Vec<_>>();
            sender
                .send(IMainToolbar::Platforms(platforms).to_imsg())
                .expect("unable to send platforms");
            conductor.signal(MainToolbar::GetPlatforms.to_event());
        }
        OMainToolbar::GetSites => {
            let sites = match db.find_all_sites().query() {
                Ok(sites) => sites,
                Err(err) => {
                    sender
                        .send(IMsg::Error(format!("Unable to get sites from db: {}", err)))
                        .expect("unable to send error msg");
                    conductor.signal(Event::Error);
                    return;
                }
            };
            let sites = sites
                .into_iter()
                .map(|mut site| std::mem::replace(&mut site.name, String::new()))
                .collect::<Vec<_>>();
            sender
                .send(IMainToolbar::Sites(sites).to_imsg())
                .expect("unable to send sites");
            conductor.signal(MainToolbar::GetSites.to_event());
        }
    }
}
