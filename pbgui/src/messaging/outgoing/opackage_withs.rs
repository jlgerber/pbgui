use super::*;

#[derive(Debug, PartialEq)]
/// Requests outgoing from the man gui thread for the package withs
/// element
pub enum OPackageWiths {
    /// Retrieve all the packages in order to populate
    /// the combobox with them.
    GetPackages,
}

impl ToOMsg for OPackageWiths {
    fn to_omsg(self) -> OMsg {
        OMsg::PackageWiths(self)
    }
}
