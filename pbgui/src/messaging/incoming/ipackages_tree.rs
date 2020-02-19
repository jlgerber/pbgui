use super::*;

/// Responses returning to the main ui thread from the secondary thread for the
/// package tree element.
pub enum IPackagesTree {
    /// Returns a vector of package names
    Packages(Vec<String>),
    /// Returns a vector of site names
    Sites(Vec<String>),
    /// Returns a vector of distribution names for a package
    DistsForPackage {
        /// The vector of distributions for a package
        dists: Vec<String>,
        /// at a given row in the versionpin table
        row: i32,
    },
    /// Returns a vector of platforms for a specific distribution.
    PlatformsForDist {
        /// The vector of platform names
        platforms: Vec<String>,
        /// for the package at the provided row in the tree
        package_row: i32,
        /// and the distribution at the provided row in the tree
        dist_row: i32,
    },
}

impl ToIMsg for IPackagesTree {
    fn to_imsg(self) -> IMsg {
        IMsg::PackagesTree(self)
    }
}
