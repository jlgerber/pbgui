use super::*;

/// perform a submatch against the OVpinDialog msg
pub(crate) fn match_vpin_dialog(
    msg: OVpinDialog,
    db: &mut packybara::db::packrat::PackratDb,
    conductor: &mut qt_thread_conductor::conductor::Conductor<Event>,
    sender: &Sender<IMsg>,
) {
    match msg {
        OVpinDialog::GetRoles => {
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
                .map(|mut x| std::mem::replace(&mut x.role, String::new()))
                .collect::<Vec<_>>();
            sender
                .send(IVpinDialog::Roles(roles).to_imsg())
                .expect("unable to send roles");
            conductor.signal(VpinDialog::UpdateRoles.to_event());
        }

        OVpinDialog::GetSites => {
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
                .send(IVpinDialog::Sites(sites).to_imsg())
                .expect("unable to send sites");
            conductor.signal(VpinDialog::UpdateSites.to_event());
        }

        OVpinDialog::GetLevels(ref show) => {
            let levels = match db.find_all_levels().show(show).query() {
                Ok(levels) => levels,
                Err(e) => {
                    sender
                        .send(IMsg::Error(format!(
                            "Unable to get levels from db for {}: {}",
                            show, e
                        )))
                        .expect("unable to send error msg");
                    conductor.signal(Event::Error);
                    return;
                }
            };
            let mut level_map = LevelMap::new();
            // If we dont have any sequences or shots, then only the show will be returned.
            // The length of the returned vec will be 1. We can return an empty map and continue.
            if levels.len() == 1 {
                sender
                    .send(IVpinDialog::Levels(level_map).to_imsg())
                    .expect("Unable to send levelmap");
                conductor.signal(VpinDialog::UpdateLevels.to_event());
                return;
            }
            // Now we get rid of the show name
            let levels = &levels[1..];
            // initialize a blank key (sequence)
            let mut key = "".to_string();
            // and an empty vec for shots
            let mut shots: Vec<String> = Vec::new();
            for level in levels {
                let pieces = level.level.split(".").collect::<Vec<_>>();
                let pieces_len = pieces.len();
                // if we have two pieces, they are show and sequence.
                if pieces_len == 2 {
                    // if the key is blank, then we have only just begun
                    if &key == "" {
                        key = pieces[1].to_string();
                    } else {
                        // we must have a previous sequence. It is time to insert
                        // whatever sequence and shots we have collected thus far, and
                        // set them up for the new sequence
                        let old_shots = std::mem::replace(&mut shots, Vec::new());
                        level_map.insert(key.clone(), old_shots);
                        // and the new sequence is in the second spot in the vector
                        key = pieces[1].to_string();
                    }
                // we are in a shot
                } else if pieces_len == 3 {
                    shots.push(pieces[2].to_string());
                } else {
                    // if we are not in a show sequence or shot then what is going on?
                    panic!("Incorrect number of pieces from get_all_levels");
                }
            }
            // we need to account for the last sequence and potential shots
            // as they will never get inserted in the previous loop
            // Of course, there is always the possiblity that we have no sequences
            // or shots. So we guard against that.
            if &key != "" {
                level_map.insert(key, shots);
            }
            // now lets send our work
            sender
                .send(IVpinDialog::Levels(level_map).to_imsg())
                .expect("Unable to send levelmap");
            conductor.signal(VpinDialog::UpdateLevels.to_event());
        }
    }
}
