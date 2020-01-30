use super::*;

pub enum IPackageWiths {
    Packages(Vec<String>),
}

impl<'a> ToIMsg<'a> for IPackageWiths {
    fn to_imsg(self) -> IMsg<'a> {
        IMsg::PackageWiths(self)
    }
}
