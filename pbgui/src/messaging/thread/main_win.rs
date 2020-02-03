use super::*;
use crate::change_type::Change;
use crate::io;
use packybara::db::update::versionpins::VersionPinChange;
use packybara::platform::Platform;
use packybara::site::Site;
use packybara::LtreeSearchMode;
use packybara::OrderDirection;
use packybara::OrderRevisionBy;
use packybara::Role;
use packybara::SearchAttribute;
use std::fs::File;
use std::io::{Error, Write};
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
        OMainWin::SavePackagesXml { show, output } => {
            // TODO: Imple
            // get a list of version pins for the show
            let results = db
                .find_all_versionpins()
                .isolate_facility(true)
                .level(show.as_str())
                .search_mode(LtreeSearchMode::Descendant)
                .order_by(vec![SearchAttribute::Role, SearchAttribute::Package])
                .query();
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
            // get a list of withs for the show
            // iterate through version pins, creating appropriate data structure for outgoing
            let mut show = io::Show::new(show);
            let mut last_role: Option<Role> = None;
            let mut role_tags = Vec::new();
            for row in vpins {
                let package = row.distribution.package();
                let version = row.distribution.version();
                let site = row.coords.site();
                let platform = row.coords.platform();
                // TODO: do not know how seq/shot is handled in packages.xml
                let level = row.coords.level(); // hwo is this handled?
                let role = row.coords.role();
                let mut package = io::Package::new(package, version, None, None);
                if site != &Site::Any {
                    package.set_site(Some(site.to_string()));
                }
                if platform != &Platform::Any {
                    package.set_os(Some(platform.to_string()));
                }
                if role != &Role::Any {
                    let role_str = role.to_string();
                    //println!("role: {}", role_str.as_str());
                    //let mut role_tag = io::Role::new(role_str);

                    //role_tag.add_package(package);

                    // if our last iter was a role
                    if let Some(ref last) = last_role {
                        // if the current role is the same as the last
                        // role, we add the package into our list
                        if role == last {
                            role_tags.push(package);
                        } else {
                            // otherwise we drain the list of saved role packages,
                            // adding them in to the show
                            let mut role_tag = io::Role::new(role_str);
                            for pkg in role_tags.drain(..) {
                                role_tag.add_package(pkg);
                            }
                            show.add_role(role_tag);
                            // and we push the current role tag into our list,
                            // which is now empty
                            //role_tags.push(role_tag);
                            role_tags.push(package);
                        }
                    } else {
                        // in the case where our last iter was NOT a role
                        // role tags should be zero sized
                        // for tag in role_tags.drain(..) {
                        //     show.add_role(tag);
                        // }
                        assert_eq!(role_tags.len(), 0);
                        //role_tags.push(role_tag);
                    }
                    //let role_cpy = role.clone();
                    last_role = Some(role.clone());
                //show.add_role(role_tag);
                } else {
                    show.add_package(package);
                    last_role = None;
                }
            }
            // serialise to disk
            let xml_writer = io::ToXml::new();
            let show = xml_writer.to_xml(show);
            let xml_str = io::ToXml::to_pretty_string(&show);
            let mut output = match File::create(output) {
                Ok(output) => output,
                Err(err) => {
                    sender
                        .send(IMsg::Error(format!(
                            "Unable to create packages.xml for writing: {}",
                            err
                        )))
                        .expect("unable to send error msg");
                    conductor.signal(Event::Error);
                    return;
                }
            };
            match write!(output, "{}", xml_str) {
                Ok(_) => {
                    sender
                        .send(IMainWin::SavePackagesXml(true).to_imsg())
                        .expect("unable to send changes");
                    conductor.signal(MainWin::SavePackagesXml.to_event());
                }
                Err(err) => {
                    sender
                        .send(IMsg::Error(format!(
                            "Unable to write packages.xml: {}",
                            err
                        )))
                        .expect("unable to send error msg");
                    conductor.signal(Event::Error);
                }
            };
        }
    }
}
