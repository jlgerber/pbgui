use super::*;

#[derive(Debug, PartialEq)]
/// Requests originating with the main gui thread for the
/// packages tree ui element.
pub enum OPackagesTree {
    /// Request a list of packages
    GetPackages,
    /// Request a list of sites
    GetSites,
    /// Request a list of package distributions
    GetPackageDists {
        /// given a site
        site: String,
        /// a package
        package: String,
        /// and the package's row in the tree
        package_row: i32,
    },
    /// Request a list of platforms for a particular
    /// distribution in the tree,
    GetDistPlatforms {
        /// for a spcified site,
        site: String,
        /// a specified package,
        package: String,
        /// a specified version
        version: String,
        /// for the package at a given row
        package_row: i32,
        /// and a distribution at a given row
        dist_row: i32,
    },
}

impl ToOMsg for OPackagesTree {
    fn to_omsg(self) -> OMsg {
        OMsg::PackagesTree(self)
    }
}
