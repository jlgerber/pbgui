//! The event handler is responsible for updating the ui in response to events
//! emitted by the Conductor. The event handler runs in the main thread. It
//! is implementad as a qt Slot and is connected to the Conductor's signal.
//!
//!  The actual data consumed by the event handler is provided via a crossbean
//! channel, which has a message of type IMsg.
//!
//! Responsibility for handling events is delegated by the event handler to
//! a specific handler, depending upon the Event. This provides an affordance
//! for scaling the solution, making maintenance reasonably straight forward.
//!
//! The specific event handlers may be found in the event_handler subdirectory.
use crate::main_window::InnerMainWindow;
use crate::messaging::{prelude::*, Event, IMsg, IVpinDialog, VpinDialog};
use crossbeam_channel::Receiver;
use log;
use pbgui_vpin::vpin_dialog;
use qt_core::{QString, SlotOfQString};
use qt_widgets::cpp_core::Ref;
use std::rc::Rc;

pub mod vpin_dialog_eh;
use vpin_dialog_eh::match_vpin_dialog;

pub mod main_toolbar_eh;
use main_toolbar_eh::match_main_toolbar;

pub mod package_withs_eh;
use package_withs_eh::match_package_withs;

pub mod packages_tree_eh;
use packages_tree_eh::match_packages_tree;

pub mod main_win_eh;
use main_win_eh::match_main_win;

pub mod ui_logger_eh;
use ui_logger_eh::match_ui_logger;
/// Generate a new event handler, which is of type `SlotOfQString`.
/// The event handler is responsible for handling Signals of type Event
///
/// # Arguments
/// * `dialog` - reference counted pointer to VpinDialog instance
/// * `main` - reference counted pointer to the InnerMainwindow instance
/// * `receiver` - The Receiver of messages from the non-ui thread
///
/// # Returns
/// * Slot which processes messages from the non-ui thread and updates the ui in response
pub fn new_event_handler<'a>(
    dialog: Rc<vpin_dialog::VpinDialog<'a>>,
    main: Rc<InnerMainWindow<'a>>,
    receiver: Receiver<IMsg>,
) -> SlotOfQString<'a> {
    SlotOfQString::new(move |name: Ref<QString>| unsafe {
        let tree = main.tree();
        let withs = main.package_withs_list();
        let main_toolbar = main.main_toolbar();
        let logger = main.logger();
        match Event::from_qstring(name) {
            Event::VpinDialog(vpin_dialog_event) => {
                match_vpin_dialog(vpin_dialog_event, dialog.clone(), &receiver)
            }
            Event::PackagesTree(packages_tree_event) => {
                match_packages_tree(packages_tree_event, tree, &receiver)
            }
            Event::PackageWiths(package_withs_event) => {
                match_package_withs(package_withs_event, withs, &receiver)
            }
            Event::MainToolbar(main_toolbar_event) => {
                match_main_toolbar(main_toolbar_event, main_toolbar, &receiver)
            }
            Event::MainWin(main_win_event) => {
                match_main_win(main_win_event, main.clone(), &receiver)
            }
            Event::UiLogger(ui_logger_event) => {
                match_ui_logger(ui_logger_event, logger.clone(), &receiver)
            }
            Event::Noop => {
                // we do nothing. this is a quirk of qt. We need to overcome
                // a signal optimization where it wont send the same signal twice
                // in a row... see the conductor doc
            }
            Event::Error => {
                if let Ok(IMsg::Error(error)) = receiver.recv() {
                    log::error!("{}", error);
                } else {
                    log::error!("unable to transmit error");
                }
            }
        }
    })
}
