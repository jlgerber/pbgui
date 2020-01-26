use super::*;

#[derive(Debug, PartialEq)]
pub enum OPackagesTree {
    GetPackages,
    GetSites,
}

impl ToOMsg for OPackagesTree {
    fn to_omsg(self) -> OMsg {
        OMsg::PackagesTree(self)
    }
}
