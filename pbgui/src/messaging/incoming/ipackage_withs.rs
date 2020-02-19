use super::*;

/// Responses returning to the main ui thread from the secondary thread
/// for the package withs element.
pub enum IPackageWiths {
    /// Returns a list of packages
    Packages(Vec<String>),
}

impl ToIMsg for IPackageWiths {
    fn to_imsg(self) -> IMsg {
        IMsg::PackageWiths(self)
    }
}
