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
pub trait ToIMsg {
    fn to_imsg(self) -> IMsg;
}

pub enum IMsg {
    VpinDialog(IVpinDialog),
    PackagesTree(IPackagesTree),
    PackageWiths(IPackageWiths),
    MainToolbar(IMainToolbar),
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
