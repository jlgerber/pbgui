use super::*;
use packybara::LtreeSearchMode;
use std::str::FromStr;

pub(crate) fn match_main_win(
    msg: OMainWin,
    db: &mut packybara::db::packrat::PackratDb,
    conductor: &mut qt_thread_conductor::conductor::Conductor<Event>,
    sender: &Sender<IMsg>,
) {
    match msg {
        OMainWin::GetVpins {
            level,
            role,
            platform,
            site,
            dir,
            package,
        } => {
            let results = db
                .find_all_versionpins()
                .isolate_facility(true)
                .level(level.as_str())
                .role(role.as_str())
                .platform(platform.as_str())
                .site(site.as_str())
                .search_mode(LtreeSearchMode::from_str(dir.as_str()).expect("unable to find vpin"))
                .query();
            //.expect("unable to unwrap vpin_finder.query");

            //let filter_package = if line_edit_txt != "" { true } else { false };

            let vpins = match results {
                Ok(vpins) => vpins,
                Err(err) => {
                    sender
                        .send(IMsg::Error(format!(
                            "Unable to get version pins from db: {}",
                            err
                        )))
                        .expect("unable to send error msg");
                    conductor.signal(Event::Error);
                    return;
                }
            };
            // let vpins = vpins
            //     .into_iter()
            //     .map(|mut x| std::mem::replace(&mut x.name, String::new()))
            //     .collect::<Vec<_>>();
            sender
                .send(IMainWin::Vpins(vpins).to_imsg())
                .expect("unable to send version pins");
            conductor.signal(MainWin::GetVpins.to_event());
        }
    }
}
