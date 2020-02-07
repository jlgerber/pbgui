use super::*;

#[derive(Debug, PartialEq)]
pub enum OPackagesTree {
    GetPackages,
    GetSites,
    GetPackageDists {
        site: String,
        package: String,
        package_row: i32,
    },
    GetDistPlatforms {
        site: String,
        package: String,
        version: String,
        package_row: i32,
        dist_row: i32,
    },
}

impl ToOMsg for OPackagesTree {
    fn to_omsg(self) -> OMsg {
        OMsg::PackagesTree(self)
    }
}
