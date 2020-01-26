#![windows_subsystem = "windows"]
use env_logger;
use env_logger::Env;
//use log;
use crossbeam_channel::{unbounded as channel, Receiver, Sender};
use pbgui::main_window;
use pbgui::messaging::init;
use pbgui::messaging::{
    client_proxy::ConnectParams, event::Event, new_event_handler, thread as pbthread, IMsg, OMsg,
    OVpinDialog,
};
use pbgui::utility::{distribution_from_idx, qs};
use pbgui_vpin::vpin_dialog;
use qt_core::{QModelIndex, QResource, Slot, SlotOfQModelIndex};
use qt_thread_conductor::conductor::Conductor;
use qt_widgets::{
    cpp_core::{MutPtr, Ref},
    QApplication, QMainWindow,
};
use rustqt_utils::enclose;
use std::env;
use std::rc::Rc;
use structopt::StructOpt;

#[derive(StructOpt, Debug, PartialEq)]
pub struct PbGui {
    /// Set the log level. This may target one or more
    /// specific modules or be general.
    /// (levels: trace, debug, info, warn, error)
    #[structopt(long)]
    pub loglevel: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = PbGui::from_args();
    if let PbGui {
        loglevel: Some(ref level),
        ..
    } = opt
    {
        env::set_var("RUST_LOG", level);
    }
    env_logger::from_env(Env::default().default_filter_or("warn")).init();
    // sender, receiver for communicating from secondary thread to primary ui thread
    let (sender, receiver): (Sender<IMsg>, Receiver<IMsg>) = channel();
    // sender and receiver for communicating from ui thread to secondary thread
    let (to_thread_sender, to_thread_receiver): (Sender<OMsg>, Receiver<OMsg>) = channel();
    // sender to handle quitting
    let to_thread_sender_quit = to_thread_sender.clone();

    QApplication::init(|app| unsafe {
        let _result = QResource::register_resource_q_string(&qs(
            "/Users/jgerber/bin/pbgui-resources/pbgui.rcc",
        ));
        let _result = QResource::register_resource_q_string(&qs(
            "/Users/jgerber/bin/pbgui-resources/pbgui_tree.rcc",
        ));
        let _result = QResource::register_resource_q_string(&qs(
            "/Users/jgerber/bin/pbgui-resources/pbgui_withlist.rcc",
        ));
        let pbgui_root = main_window::MainWindow::new(to_thread_sender.clone());
        init::packages_tree::init(to_thread_sender.clone());
        init::package_withs::init(to_thread_sender.clone());
        init::main_toolbar::init(to_thread_sender.clone());

        let dialog = Rc::new(create_dialog("unset", "unset", pbgui_root.main()));
        init::vpin_dialog::init(to_thread_sender.clone(), "facility");

        // we create a slot that is triggered when OK is pressed to act only in the event
        // that the user has requested action.
        let accepted_slot = Slot::new(enclose! { (dialog) move || {
            if let Some(roles) = dialog.selected_roles() {
                println!("roles: {:?}", roles);
            } else {
                println!("roles: any");
            }
            if let Some(selected_level) = dialog.selected_level() {
                println!("level: {:?}", selected_level);
            } else {
                println!("level: {}", dialog.show_name());
            }
            match dialog.selected_site(){
                Some(site) => println!(
                    "site:  {}", site
                ),
                None => println!("site:  Any"),
            }
            dialog.accept();
        }});

        // Connect the accepted signal to the accepted slot
        dialog.accepted().connect(&accepted_slot);

        //let mtoolbar = pbgui_root.main_toolbar();
        let mtoolbar = pbgui_root.main_win().main_toolbar();

        let exec_dialog_slot = SlotOfQModelIndex::new(
            enclose! { (dialog, to_thread_sender) move |idx: Ref<QModelIndex>| {
                    println!("clicked slot called");
                if let Some(dist) = distribution_from_idx(idx) {
                    dialog.set_distribution(dist.as_str());
                    let show = mtoolbar.show_string();
                    dialog.set_show_name(show.as_str());
                    update_vpin_dialog(&to_thread_sender, show);
                    let result = dialog.dialog_mut().exec();
                    println!("exec_dialog_slot triggered by button result -> {}", result);
                }
            }},
        );

        pbgui_root
            .main_win()
            .packages_tree()
            .clicked()
            .connect(&exec_dialog_slot);

        let app_update = new_event_handler(
            dialog.clone(),
            pbgui_root.main_win(),
            //pbgui_root.main_win().tree(),
            //pbgui_root.main_win().package_withs_list(),
            //pbgui_root.main_win().main_toolbar(),
            receiver,
        );

        let my_conductor = Conductor::<Event>::new(&app_update);

        let _quit_slot = pbthread::create_quit_slot(to_thread_sender_quit, app.clone());

        pbthread::create(
            ConnectParams::default(),
            pbgui_root.main(),
            my_conductor,
            sender,
            to_thread_receiver,
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

///  updagte the  dialog to  pin a distribution with data. This consists
/// Of requesting that we get roles, sites, and levels
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
