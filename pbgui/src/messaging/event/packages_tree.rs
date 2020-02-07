use super::*;

#[derive(Debug, PartialEq)]
pub enum PackagesTree {
    GetPackages,
    GetSites,
    GetDistsForPackage,
}

impl ToEvent for PackagesTree {
    fn to_event(self) -> Event {
        Event::PackagesTree(self)
    }
}

impl ToQString for PackagesTree {
    fn to_qstring(&self) -> CppBox<QString> {
        match &self {
            &PackagesTree::GetPackages => QString::from_std_str("PackagesTree::GetPackages"),
            &PackagesTree::GetSites => QString::from_std_str("PackagesTree::GetSites"),
            &PackagesTree::GetDistsForPackage => {
                QString::from_std_str("PackagesTree::GetDistsForPackage")
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
            _ => panic!("Unable to convert to Event"),
        }
    }
}
