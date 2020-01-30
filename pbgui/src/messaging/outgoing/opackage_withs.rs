use super::*;

#[derive(Debug, PartialEq)]
pub enum OPackageWiths {
    GetPackages,
}

impl<'a> ToOMsg<'a> for OPackageWiths {
    fn to_omsg(self) -> OMsg<'a> {
        OMsg::PackageWiths(self)
    }
}
