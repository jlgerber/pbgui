use super::*;

#[derive(Debug, PartialEq)]
pub enum OPackageWiths {
    GetPackages,
}

impl ToOMsg for OPackageWiths {
    fn to_omsg(self) -> OMsg {
        OMsg::PackageWiths(self)
    }
}
