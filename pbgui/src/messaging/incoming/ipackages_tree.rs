use super::*;

pub enum IPackagesTree {
    Packages(Vec<String>),
    Sites(Vec<String>),
}

impl<'a> ToIMsg<'a> for IPackagesTree {
    fn to_imsg(self) -> IMsg<'a> {
        IMsg::PackagesTree(self)
    }
}
