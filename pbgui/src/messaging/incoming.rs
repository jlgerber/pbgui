//! incoming models the message being sent from the secondary thread
//! to the application
use pbgui_vpin::vpin_dialog::LevelMap;

/// ToIMsg trait should be implemented by the nested incoming message
/// enums. The trait is used to reduce the visual noise when dealing with IMsg
///
/// # Example
/// In thread, we send messages to the ui thread using an mpsc::channel .
/// The call would look like this without the trait:
/// ```ignore
/// sender
///   .send(IMsg::VpinDialog(IVPinDialog::Roles(roles)))
///   .expect("bla bla bla");
/// ```
/// With the trait, it can be simplified somewhat to this:
/// ```ignore
///  sender
///    .send(IVpinDialog::Roles(roles).to_imsg())
///    .expect("unable to send roles");
pub trait ToIMsg<'a> {
    fn to_imsg(self) -> IMsg<'a>;
}

pub enum IMsg<'a> {
    VpinDialog(IVpinDialog),
    PackagesTree(IPackagesTree),
    PackageWiths(IPackageWiths),
    MainToolbar(IMainToolbar),
    MainWin(IMainWin),
    UiLogger(IUiLogger<'a>),
    Error(String),
}

pub mod ivpin_dialog;
pub use ivpin_dialog::IVpinDialog;

pub mod ipackages_tree;
pub use ipackages_tree::IPackagesTree;

pub mod ipackage_withs;
pub use ipackage_withs::IPackageWiths;

pub mod imain_toolbar;
pub use imain_toolbar::IMainToolbar;

pub mod imain_win;
pub use imain_win::IMainWin;

pub mod iui_logger;
pub use iui_logger::IUiLogger;
