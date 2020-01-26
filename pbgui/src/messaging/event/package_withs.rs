use super::*;

#[derive(Debug, PartialEq)]
pub enum PackageWiths {
    GetPackages,
}

impl ToEvent for PackageWiths {
    fn to_event(self) -> Event {
        Event::PackageWiths(self)
    }
}

impl ToQString for PackageWiths {
    fn to_qstring(&self) -> CppBox<QString> {
        match &self {
            &PackageWiths::GetPackages => QString::from_std_str("PackageWiths::GetPackages"),
        }
    }
}

impl FromQString for PackageWiths {
    fn from_qstring(qs: Ref<QString>) -> Self {
        match qs.to_std_string().as_str() {
            "PackageWiths::GetPackages" => PackageWiths::GetPackages,
            _ => panic!("Unable to convert to Event"),
        }
    }
}
