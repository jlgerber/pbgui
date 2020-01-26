//! handle queries in a separate thread
use crate::{
    client_proxy::{ClientProxy, ConnectParams},
    event::{MainToolbar, PackageWiths, PackagesTree},
    incoming::{IMainToolbar, IPackageWiths, IPackagesTree},
    outgoing::{OMainToolbar, OPackageWiths, OPackagesTree},
    Event, IMsg, IVpinDialog, OMsg, OVpinDialog, ToEvent, ToIMsg, VpinDialog,
};
use crossbeam_channel::{Receiver, Sender};
use crossbeam_utils::thread;
use log;
use packybara::packrat::PackratDb;
use packybara::traits::*;
use pbgui_vpin::vpin_dialog::LevelMap;
use qt_core::Slot;
use qt_thread_conductor::conductor::Conductor;
use qt_widgets::{cpp_core::MutPtr, QApplication, QMainWindow};

pub mod vpin_dialog;
use vpin_dialog::match_vpin_dialog;

pub mod package_withs;
use package_withs::match_package_withs;

pub mod packages_tree;
use packages_tree::match_packages_tree;

pub mod main_toolbar;
use main_toolbar::match_main_toolbar;

/// Create the thread that handles requests for data from the ui. The thread
/// receives messages via the `receiver`, matches against them, and sends data
/// back to the UI via the `sender`. Finally, triggering an appropriate update
/// via the `conductor`. The `conductor` and `sender` work as a team. The `sender`
/// handles complex data, and the `conductor` notifies QT.
///
/// # Arguments
/// * `main_window` - Mutable MutPtr wrapped QMainWindow instance
/// * `conductor` - Mutable instance of the Conductor<Event>, responsible for signaling
///                 to QT
/// * sender - Sends IMsg's to the UI thread
/// * receiver - Receives OMsg's from the UI thread
///
/// # Returns
/// * i32 - The status
pub fn create(
    connect_params: ConnectParams,
    mut main_window: MutPtr<QMainWindow>,
    mut conductor: Conductor<Event>,
    sender: Sender<IMsg>,
    receiver: Receiver<OMsg>,
) -> i32 {
    let mut result = 0;
    thread::scope(|s| {
        let handle = s.spawn(|_| {
            let client = match ClientProxy::connect(connect_params) {
                Ok(client) => client,
                Err(err) => {
                    sender
                        .send(IMsg::Error(err.to_string()))
                        .expect("unable to send roles");
                    conductor.signal(Event::Error);
                    panic!("unable to connect to database");
                }
            };
            let mut db = PackratDb::new(client);
            //let mut show: Option<String> = None;
            loop {
                let msg = receiver.recv().expect("Unable to unwrap received msg");
                match msg {
                    OMsg::VpinDialog(msg) => {
                        match_vpin_dialog(msg, &mut db, &mut conductor, &sender);
                    }
                    OMsg::PackagesTree(msg) => {
                        match_packages_tree(msg, &mut db, &mut conductor, &sender);
                    }
                    OMsg::PackageWiths(msg) => {
                        match_package_withs(msg, &mut db, &mut conductor, &sender);
                    }
                    OMsg::MainToolbar(msg) => {
                        match_main_toolbar(msg, &mut db, &mut conductor, &sender);
                    }
                    OMsg::Quit => {
                        log::info!("From secondary thread. Quitting after receiving OMsg::Quit");
                        // try break instead of return
                        break;
                    }
                }
            }
        });
        // the application needs to show and execute before the thread handle is joined
        // so that the scope lives longer than the application
        unsafe {
            main_window.show();
            result = QApplication::exec();
        }
        let _res = handle.join().expect("problem joining scoped thread handle");
    })
    .expect("problem with scoped channel");
    result
}
/// Create the slot that handles terminating the secondary thread when
/// the application is about to quit. This function will also wire up
/// the appropriate signal & slot to handle this.
///
/// # Arguments
/// * `to_thread_sender` - the sender responsible for signaling the secondary thread.
/// * `app` - A MutPtr to the QApplication instance.
///
/// # Returns
/// * the slot designed to terminate the secondary thread
pub fn create_quit_slot<'a>(to_thread_sender: Sender<OMsg>, app: MutPtr<QApplication>) -> Slot<'a> {
    let quit_slot = Slot::new(move || {
        log::info!("Sending secondary thread termination request ");
        to_thread_sender.send(OMsg::Quit).expect("couldn't send");
    });
    unsafe {
        app.about_to_quit().connect(&quit_slot);
    }
    quit_slot
}
