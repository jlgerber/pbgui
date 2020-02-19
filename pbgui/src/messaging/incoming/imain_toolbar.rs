use super::*;
/// Responses returning to the main ui thread from the secondary thread
/// for the main toolbar element.
pub enum IMainToolbar {
    /// Provides a vector of show names
    Shows(Vec<String>),
    /// Provides a vector of role names
    Roles(Vec<String>),
    /// Provides a vector of platform names
    Platforms(Vec<String>),
    /// Provides a vector of site names
    Sites(Vec<String>),
}

impl ToIMsg for IMainToolbar {
    fn to_imsg(self) -> IMsg {
        IMsg::MainToolbar(self)
    }
}
