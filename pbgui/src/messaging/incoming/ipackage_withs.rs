use super::*;

pub enum IPackageWiths {
    Packages(Vec<String>),
}

impl ToIMsg for IPackageWiths {
    fn to_imsg(self) -> IMsg {
        IMsg::PackageWiths(self)
    }
}
