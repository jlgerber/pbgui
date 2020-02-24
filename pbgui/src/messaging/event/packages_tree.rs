//! Provides the PackagesTree enum, which implements the ToEvent, ToQString, and FromQString traits
use super::*;

#[derive(Debug, PartialEq)]
pub enum PackagesTree {
    GetPackages,
    GetSites,
    GetDistsForPackage,
    GetPlatformsForDist,
}

impl ToEvent for PackagesTree {
    fn to_event(self) -> Event {
        Event::PackagesTree(self)
    }
}

impl ToQString for PackagesTree {
    fn to_qstring(&self) -> CppBox<QString> {
        match &self {
            PackagesTree::GetPackages => QString::from_std_str("PackagesTree::GetPackages"),
            PackagesTree::GetSites => QString::from_std_str("PackagesTree::GetSites"),
            PackagesTree::GetDistsForPackage => {
                QString::from_std_str("PackagesTree::GetDistsForPackage")
            }
            PackagesTree::GetPlatformsForDist => {
                QString::from_std_str("PackagesTree::GetPlatformsForDist")
            }
        }
    }
}

impl FromQString for PackagesTree {
    fn from_qstring(qs: Ref<QString>) -> Self {
        match qs.to_std_string().as_str() {
            "PackagesTree::GetPackages" => PackagesTree::GetPackages,
            "PackagesTree::GetSites" => PackagesTree::GetSites,
            "PackagesTree::GetDistsForPackage" => PackagesTree::GetDistsForPackage,
            "PackagesTree::GetPlatformsForDist" => PackagesTree::GetPlatformsForDist,
            _ => panic!("Unable to convert to Event"),
        }
    }
}
