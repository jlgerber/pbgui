use super::*;

pub enum IPackagesTree {
    Packages(Vec<String>),
    Sites(Vec<String>),
    DistsForPackage {
        dists: Vec<String>,
        row: i32,
    },
    PlatformsForDist {
        platforms: Vec<String>,
        package_row: i32,
        dist_row: i32,
    },
}

impl ToIMsg for IPackagesTree {
    fn to_imsg(self) -> IMsg {
        IMsg::PackagesTree(self)
    }
}
