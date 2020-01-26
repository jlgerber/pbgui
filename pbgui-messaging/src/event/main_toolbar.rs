use super::*;

#[derive(Debug, PartialEq)]
pub enum MainToolbar {
    GetShows,
    GetRoles,
    GetPlatforms,
    GetSites,
}

impl ToEvent for MainToolbar {
    fn to_event(self) -> Event {
        Event::MainToolbar(self)
    }
}

impl ToQString for MainToolbar {
    fn to_qstring(&self) -> CppBox<QString> {
        match &self {
            &MainToolbar::GetShows => QString::from_std_str("MainToolbar::GetShows"),
            &MainToolbar::GetRoles => QString::from_std_str("MainToolbar::GetRoles"),
            &MainToolbar::GetPlatforms => QString::from_std_str("MainToolbar::GetPlatforms"),
            &MainToolbar::GetSites => QString::from_std_str("MainToolbar::GetSites"),
        }
    }
}

impl FromQString for MainToolbar {
    fn from_qstring(qs: Ref<QString>) -> Self {
        match qs.to_std_string().as_str() {
            "MainToolbar::GetShows" => MainToolbar::GetShows,
            "MainToolbar::GetRoles" => MainToolbar::GetRoles,
            "MainToolbar::GetPlatforms" => MainToolbar::GetPlatforms,
            "MainToolbar::GetSites" => MainToolbar::GetSites,
            _ => panic!("Unable to convert to Event"),
        }
    }
}
