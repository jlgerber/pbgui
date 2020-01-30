use super::*;
use crate::change_type::Change;
use packybara::db::update::versionpins::VersionPinChange;
use packybara::LtreeSearchMode;
use packybara::OrderDirection;
use packybara::OrderRevisionBy;
use std::str::FromStr;

pub fn match_main_win(
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

            //let filter_package = if line_edit_txt != "" { true } else { false };

            let mut vpins = match results {
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
            if let Some(package_str) = package {
                vpins = vpins
                    .into_iter()
                    .filter(|x| x.distribution.package().starts_with(&package_str))
                    .collect::<Vec<_>>();
            }
            sender
                .send(IMainWin::Vpins(vpins).to_imsg())
                .expect("unable to send version pins");
            conductor.signal(MainWin::GetVpins.to_event());
        }
        OMainWin::GetWithsForVpin { vpin_id } => {
            let results = db.find_all_versionpin_withs(vpin_id).query();
            let withs = match results {
                Ok(withs) => withs,
                Err(err) => {
                    sender
                        .send(IMsg::Error(format!(
                            "Unable to get with packages from db: {}",
                            err
                        )))
                        .expect("unable to send error msg");
                    conductor.signal(Event::Error);
                    return;
                }
            };
            sender
                .send(IMainWin::WithPackages(withs).to_imsg())
                .expect("unable to send version pins");
            conductor.signal(MainWin::GetWithsForVpin.to_event());
        }
        OMainWin::GetTransactionChanges { tx_id } => {
            let results = db.find_all_changes().transaction_id(tx_id as i64).query();
            let changes = match results {
                Ok(changes) => changes,
                Err(err) => {
                    sender
                        .send(IMsg::Error(format!(
                            "Unable to get with packages from db: {}",
                            err
                        )))
                        .expect("unable to send error msg");
                    conductor.signal(Event::Error);
                    return;
                }
            };
            sender
                .send(IMainWin::Changes(changes).to_imsg())
                .expect("unable to send version pins");
            conductor.signal(MainWin::GetTransactionChanges.to_event());
        }
        OMainWin::GetHistoryRevisions => {
            let results = db
                .find_all_revisions()
                //transaction_id(tx_id as i64)
                .order_by(vec![OrderRevisionBy::Id])
                .order_direction(OrderDirection::Desc)
                .query();
            let revisions = match results {
                Ok(revisions) => revisions,
                Err(err) => {
                    sender
                        .send(IMsg::Error(format!(
                            "Unable to get with revisions from db: {}",
                            err
                        )))
                        .expect("unable to send error msg");
                    conductor.signal(Event::Error);
                    return;
                }
            };
            sender
                .send(IMainWin::HistoryRevisions(revisions).to_imsg())
                .expect("unable to send revisions");
            conductor.signal(MainWin::GetHistoryRevisions.to_event());
        }
        OMainWin::SaveVpinChanges {
            changes,
            comments,
            user,
        } => {
            // let results = db
            //     .find_all_revisions()
            //     //transaction_id(tx_id as i64)
            //     .order_by(vec![OrderRevisionBy::Id])
            //     .order_direction(OrderDirection::Desc)
            //     .query();
            // let revisions = match results {
            //     Ok(revisions) => revisions,
            //     Err(err) => {
            //         sender
            //             .send(IMsg::Error(format!(
            //                 "Unable to get with revisions from db: {}",
            //                 err
            //             )))
            //             .expect("unable to send error msg");
            //         conductor.signal(Event::Error);
            //         return;
            //     }
            // };
            let mut tx = db.transaction();
            let mut tx_cnt = 0;
            for change in changes {
                match change {
                    Change::ChangeDistribution {
                        vpin_id,
                        new_dist_id,
                    } => {
                        let change = VersionPinChange::new(vpin_id, Some(new_dist_id), None);
                        let mut update = PackratDb::update_versionpins(tx)
                            .change(change)
                            .update()
                            .unwrap();
                        tx = update.take_tx();
                        tx_cnt += 1;
                    }
                    Change::ChangeWiths { vpin_id, withs } => {
                        let mut update = PackratDb::add_withs(tx).create(vpin_id, withs).unwrap();
                        tx = update.take_tx()
                    }
                    _ => panic!("not implemented"),
                }
            }
            let results = PackratDb::commit(tx, user.as_str(), comments.as_str(), tx_cnt);

            sender
                .send(IMainWin::SaveVpinChanges(results.is_ok()).to_imsg())
                .expect("unable to send changes");
            conductor.signal(MainWin::SaveVpinChanges.to_event());
        }

        OMainWin::ChooseDistribution {
            package,
            version,
            row,
        } => {
            let results = db
                .find_all_distributions()
                .package(package.as_str())
                .query()
                .expect("unable to unwrap query of distributions");
            sender
                .send(
                    IMainWin::ChooseDistribution {
                        distributions: results,
                        package,
                        version,
                        row,
                    }
                    .to_imsg(),
                )
                .expect("unable to send changes");
            conductor.signal(MainWin::ChooseDistribution.to_event());
        }
    }
}
