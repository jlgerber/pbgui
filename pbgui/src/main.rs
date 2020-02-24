#![windows_subsystem = "windows"]
use crossbeam_channel::{unbounded as channel, Receiver, Sender};
use main_error::MainError;
use pbgui::constants::{COL_DISTRIBUTION, COL_LEVEL, COL_PLATFORM, COL_ROLE, COL_SITE};
use pbgui::main_window;
use pbgui::messaging::init;
use pbgui::messaging::{
    event::Event, new_event_handler, thread as pbthread, IMsg, OMsg, OVpinDialog,
};
use pbgui::prefs::*;
use pbgui::utility::{distribution_from_idx, qs};
use pbgui_vpin::vpin_dialog;
use std::collections::HashMap;

use qt_core::{
    ApplicationAttribute, QChar, QCoreApplication, QModelIndex, QResource, QString, Slot,
    SlotOfQModelIndex,
};
use qt_thread_conductor::conductor::Conductor;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr, Ref},
    QApplication, QMainWindow,
};
use rustqt_utils::{enclose, ToQString};
use std::rc::Rc;
use structopt::StructOpt;

/// Map used to
type RoleMap = HashMap<String, CppBox<QString>>;
#[derive(StructOpt, Debug, PartialEq)]
pub struct PbGui {
    /// Set the log level. This may target one or more
    /// specific modules or be general.
    /// (levels: trace, debug, info, warn, error)
    #[structopt(long)]
    pub loglevel: Option<String>,

    /// Supply a path to the pbgui_preferences.yaml
    #[structopt(short = "f", long)]
    pub prefs: Option<String>,

    /// Set us into test-mode. If true we will search for preferences
    /// in the user's work directory in addition to standard locations.
    #[structopt(short, long)]
    pub testmode: bool,
}

//fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<(), MainError> {
    let opt = PbGui::from_args();
    let log_level = if let PbGui {
        loglevel: Some(ref level),
        ..
    } = opt
    {
        level
    } else {
        "debug"
    };

    let test_mode = match opt {
        PbGui { testmode, .. } => testmode,
    };

    let preference = if let PbGui {
        prefs: Some(ref prefs),
        ..
    } = opt
    {
        PbguiPrefs::load_file(prefs)?
    } else {
        let finder = DDPreferenceFinder::from_env(PreferenceName::Main("pbgui".to_string()));
        let ctx = if test_mode == true {
            DDContext::TestEqUser
        } else {
            DDContext::Normal
        };
        PbguiPrefs::load(&finder, ctx)?
    };
    // {
    //     env::set_var("RUST_LOG", level);
    // }
    //env_logger::from_env(Env::default().default_filter_or("warn")).init();
    // sender, receiver for communicating from secondary thread to primary ui thread
    let (sender, receiver): (Sender<IMsg>, Receiver<IMsg>) = channel();
    // sender and receiver for communicating from ui thread to secondary thread
    let (to_thread_sender, to_thread_receiver): (Sender<OMsg>, Receiver<OMsg>) = channel();
    // sender to handle quitting
    let to_thread_sender_quit = to_thread_sender.clone();
    // turn off native menubar
    unsafe {
        QCoreApplication::set_attribute_1a(ApplicationAttribute::AADontUseNativeMenuBar);
        QCoreApplication::set_attribute_1a(ApplicationAttribute::AADontShowIconsInMenus);
    }
    QApplication::init(|app| unsafe {
        let _result = QResource::register_resource_q_string(&qs(
            "/Users/jgerber/bin/pbgui-resources/pbgui.rcc",
        ));
        let _result = QResource::register_resource_q_string(&qs(
            "/Users/jgerber/bin/pbgui-resources/pbgui_withlist.rcc",
        ));
        let _result = QResource::register_resource_q_string(&qs(
            "/Users/jgerber/bin/pbgui-resources/pbgui_logger.rcc",
        ));
        let pbgui_root = main_window::MainWindow::new(to_thread_sender.clone());
        init::packages_tree::init(to_thread_sender.clone());
        init::package_withs::init(to_thread_sender.clone());
        init::main_toolbar::init(to_thread_sender.clone());

        let dialog = Rc::new(create_dialog("unset", "unset", pbgui_root.main()));
        init::vpin_dialog::init(to_thread_sender.clone(), "facility");

        // we create a slot that is triggered when OK is pressed to act only in the event
        // that the user has requested action.
        let inner_main_win = pbgui_root.main_win();
        let accepted_slot = Slot::new(
            // TODO: move implementation to slot_functions
            enclose! { (dialog, inner_main_win, to_thread_sender) move || {
                let roles = if let Some(roles) = dialog.selected_roles() {
                    roles
                } else {
                    vec!["any".to_string()]
                };

                let level = if let Some(selected_level) = dialog.selected_level() {
                     selected_level
                } else {
                    dialog.show_name()
                };

                let site = match dialog.selected_site(){
                    Some(site) => site,
                    None =>"any".to_string()
                };

                let dist = dialog.distribution();

                // TODO: pass in platform
                let platform = "any".to_string();

                // Identify whether the vpin table either (a) already has a pkgcoord that matches our
                // choices or (b) whether the query button has yet to be pressed. In either case we log and return
                let vpin_table = inner_main_win.vpin_table();
                let cnt = vpin_table.row_count();
                // this doesnt work as the table could be empty after the query. We will need to keep track of
                // whether the query button has been pressed.
                // if cnt == 0 {
                //     log::warn!("versionpin table has no items. skipping adding new versionpin");
                //     dialog.accept();
                //     return;
                // }
                let level_qs = &level.to_qstring();
                let mut roles_map = roles.into_iter().fold( RoleMap::new(), |mut acc, rol| {
                    acc.insert(rol.clone(), rol.to_qstring()); acc}
                );
                if roles_map.is_empty() {
                    log::error!("roles_map is empty but should not be");
                }
                let platform_qs = platform.to_qstring();
                let site_qs = site.to_qstring();
                let package_qs = dialog.package_qs();
                let dash = QChar::from_int(45); // 45 is ascii code for dash
                // check to see if we match the package and coords
                for row in 0..cnt {
                    let level_ = vpin_table.item(row,COL_LEVEL);
                     if level_qs.compare_q_string(level_.text().as_ref()) != 0 {
                         continue;
                        }

                    let platform_ = vpin_table.item(row, COL_PLATFORM);
                    if platform_qs.compare_q_string(platform_.text().as_ref()) != 0 {
                        continue;}

                    let site_ =  vpin_table.item(row, COL_SITE);
                    if site_qs.compare_q_string(site_.text().as_ref()) != 0 {
                        continue;}

                    let distribution = vpin_table.item(row, COL_DISTRIBUTION).text();
                    // doenst work unless the following is split in two
                    let package_ = distribution.split_q_char(dash.as_ref());
                    let package_ = package_.first();
                    if package_qs.compare_q_string(package_) != 0 {
                        continue;
                    } ;

                    //now we tackle roles. we remove any roles from the map that match, as roles is the
                    let role_ =  vpin_table.item(row, COL_ROLE).text();
                    let mut remove = Vec::new();
                    {
                        for (role_str,role_qs) in roles_map.iter() {
                            if role_.compare_q_string(role_qs) == 0 {
                                // have to clone this as it relates to the map borrow
                                remove.push(role_str.clone());
                            }
                        }
                    }
                    for role_str in remove {
                        roles_map.remove(&role_str);
                    }
                }

                if roles_map.is_empty() {
                    log::warn!("requested package and pkgcoordinates match existing items in versionpin table. skipping");
                    dialog.accept();
                    return;
                }

                // now we create roles from the remaining keys in roles_map.
                let roles = roles_map.drain().fold(Vec::new(), |mut acc, (k,_v)| {acc.push(k); acc});
                to_thread_sender
                .send(OMsg::VpinDialog(
                    OVpinDialog::SetVpin {
                        dist,
                        // and one or more roles
                        roles,
                        // at the supplied level
                        level,
                        // and site
                        site,
                        // and platform
                        platform,
                    },
                ))
                .expect("unable to get vpins");
                dialog.accept();
            }},
        );

        // Connect the accepted signal to the accepted slot
        dialog.accepted().connect(&accepted_slot);

        //let mtoolbar = pbgui_root.main_toolbar();
        let mtoolbar = pbgui_root.main_win().main_toolbar();

        let exec_dialog_slot = SlotOfQModelIndex::new(
            enclose! { (dialog, to_thread_sender) move |idx: Ref<QModelIndex>| {
                if let Some(dist) = distribution_from_idx(idx) {
                    dialog.set_distribution(dist.as_str());
                    let show = mtoolbar.show_string();
                    dialog.set_show_name(show.as_str());
                    update_vpin_dialog(&to_thread_sender, show);
                    let _result = dialog.dialog_mut().exec();
                }
            }},
        );

        pbgui_root
            .main_win()
            .tree()
            .clicked()
            .connect(&exec_dialog_slot);

        let app_update = new_event_handler(dialog.clone(), pbgui_root.main_win(), receiver);

        let my_conductor = Conductor::<Event>::new(&app_update);

        let _quit_slot = pbthread::create_quit_slot(to_thread_sender_quit, app);

        pbthread::create(
            preference.as_connectparams(),
            pbgui_root.main(),
            my_conductor,
            sender,
            to_thread_receiver,
            to_thread_sender,
            log_level,
        )
    });
}

unsafe fn create_dialog<'a, I: Into<String>>(
    name: I,
    distribution: &'a str,
    main_ptr: MutPtr<QMainWindow>,
) -> vpin_dialog::VpinDialog<'a> {
    let dialog = vpin_dialog::VpinDialog::create(name, distribution, main_ptr);
    dialog.set_default_stylesheet();
    dialog
}

/// Update the dialog to pin a distribution with data. This consists
/// of requesting that we get roles, sites, and levels
///
/// # Arguments
/// * `to_thread_sender` - A reference to a Sender instance for OMsg's.
/// * `show` - The show name as a string
///
/// # Returns
/// * None
pub fn update_vpin_dialog(to_thread_sender: &Sender<OMsg>, show: String) {
    to_thread_sender
        .send(OMsg::VpinDialog(OVpinDialog::GetRoles))
        .expect("unable to get roles");
    to_thread_sender
        .send(OMsg::VpinDialog(OVpinDialog::GetSites))
        .expect("unable to get sites");
    to_thread_sender
        .send(OMsg::VpinDialog(OVpinDialog::GetLevels(show)))
        .expect("unable to get levels");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn qchar_dash() {
        unsafe {
            let dash = QChar::from_int(45);
            let qst = QString::from_q_char(dash.as_ref());
            let qst_str = qst.to_std_string();
            assert_eq!(qst_str.as_str(), "-");
        }
    }
    #[test]
    fn can_split() {
        unsafe {
            let dash = QChar::from_int(45);
            let distribution = qs("foo-1.2.3");
            let package_qs = qs("foo");
            //let package_ = distribution.split_q_string(qs("-").as_ref());
            let package_ = distribution.split_q_char(dash.as_ref());

            assert_eq!(package_.length(), 2);
            let package_ = package_.first();
            assert_eq!(package_qs.compare_q_string(package_), 0);
        }
    }
}
