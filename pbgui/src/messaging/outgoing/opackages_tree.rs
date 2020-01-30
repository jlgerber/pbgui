use super::*;

#[derive(Debug, PartialEq)]
pub enum OPackagesTree {
    GetPackages,
    GetSites,
}

impl<'a> ToOMsg<'a> for OPackagesTree {
    fn to_omsg(self) -> OMsg<'a> {
        OMsg::PackagesTree(self)
    }
}
