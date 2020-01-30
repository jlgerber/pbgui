//! handle queries in a separate thread
use crate::{
    logger,
    messaging::{
        client_proxy::{ClientProxy, ConnectParams},
        event::{MainToolbar, MainWin, PackageWiths, PackagesTree},
        incoming::{IMainToolbar, IMainWin, IPackageWiths, IPackagesTree},
        outgoing::{OMainToolbar, OMainWin, OPackageWiths, OPackagesTree},
        Event, IMsg, IVpinDialog, OMsg, OVpinDialog, ToEvent, ToIMsg, VpinDialog,
    },
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
pub use vpin_dialog::*;

pub mod package_withs;
pub use package_withs::*;

pub mod packages_tree;
pub use packages_tree::*;

pub mod main_toolbar;
pub use main_toolbar::*;

pub mod main_win;
pub use main_win::*;

pub mod ui_logger;
pub use ui_logger::*;

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
// pub fn create<'a>(
//     connect_params: ConnectParams<'a>,
//     mut conductor: Conductor<Event>,
//     sender: Sender<IMsg<'a>>,
//     receiver: Receiver<OMsg<'a>>,
//     to_thread_sender: Sender<OMsg<'a>>,
//     log_level: &'a str,
//     s: &crossbeam_utils::thread::Scope<'a>,
// ) -> crossbeam_utils::thread::ScopedJoinHandle<'a, ()> {
//     //thread::scope(|s| {
//     let handle = s.spawn(|_| {
//         let client = match ClientProxy::connect(connect_params) {
//             Ok(client) => client,
//             Err(err) => {
//                 sender
//                     .send(IMsg::Error(err.to_string()))
//                     .expect("unable to send roles");
//                 conductor.signal(Event::Error);
//                 panic!("unable to connect to database");
//             }
//         };
//         let mut db = PackratDb::new(client);
//         loop {
//             let msg = receiver.recv().expect("Unable to unwrap received msg");
//             match msg {
//                 OMsg::VpinDialog(msg) => {
//                     match_vpin_dialog(msg, &mut db, &mut conductor, &sender);
//                 }
//                 OMsg::PackagesTree(msg) => {
//                     match_packages_tree(msg, &mut db, &mut conductor, &sender);
//                 }
//                 OMsg::PackageWiths(msg) => {
//                     match_package_withs(msg, &mut db, &mut conductor, &sender);
//                 }
//                 OMsg::MainToolbar(msg) => {
//                     match_main_toolbar(msg, &mut db, &mut conductor, &sender);
//                 }
//                 OMsg::MainWin(msg) => {
//                     match_main_win(msg, &mut db, &mut conductor, &sender);
//                 }
//                 OMsg::UiLogger(msg) => {
//                     match_ui_logger(msg, &mut conductor, &sender);
//                 }
//                 OMsg::Quit => {
//                     log::info!("From secondary thread. Quitting after receiving OMsg::Quit");
//                     // try break instead of return
//                     break;
//                 }
//             }
//         }
//     });
//     // the application needs to show and execute before the thread handle is joined
//     // so that the scope lives longer than the application
//     handle
// }
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
pub fn create_quit_slot<'a>(
    to_thread_sender: Sender<OMsg<'a>>,
    app: MutPtr<QApplication>,
) -> Slot<'a> {
    let quit_slot = Slot::new(move || {
        log::info!("Sending secondary thread termination request ");
        to_thread_sender.send(OMsg::Quit).expect("couldn't send");
    });
    unsafe {
        app.about_to_quit().connect(&quit_slot);
    }
    quit_slot
}
