use super::*;

pub enum IPackagesTree {
    Packages(Vec<String>),
    Sites(Vec<String>),
}

impl ToIMsg for IPackagesTree {
    fn to_imsg(self) -> IMsg {
        IMsg::PackagesTree(self)
    }
}
