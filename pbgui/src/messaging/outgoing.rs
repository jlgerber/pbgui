//! models message being sent from the application to the secondary thread
pub mod ovpin_dialog;
pub use ovpin_dialog::OVpinDialog;
pub mod opackages_tree;
pub use opackages_tree::OPackagesTree;
pub mod opackage_withs;
pub use opackage_withs::OPackageWiths;
pub mod omain_toolbar;
pub use omain_toolbar::OMainToolbar;

pub mod omain_win;
pub use omain_win::OMainWin;
pub mod oui_logger;
pub use oui_logger::OUiLogger;
///
pub trait ToOMsg<'a> {
    fn to_omsg(self) -> OMsg<'a>;
}

#[derive(Debug)]
pub enum OMsg<'a> {
    VpinDialog(OVpinDialog),
    PackagesTree(OPackagesTree),
    PackageWiths(OPackageWiths),
    MainToolbar(OMainToolbar),
    MainWin(OMainWin),
    UiLogger(OUiLogger<'a>),
    Quit,
}
