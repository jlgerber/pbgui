use super::*;
use crate::change_type::Change;
/// Responses returning to the main ui thread from the secondary thread for the
/// versionpin dialog element.
pub enum IVpinDialog {
    /// Returns a vector of all role names
    Roles(Vec<String>),
    /// Returns a vector of all site names
    Sites(Vec<String>),
    /// Returns a vector of levels in a LevelMap, which is a type alias for
    /// a HashMap<String, Vec<String>>;
    Levels(LevelMap),
    // this should probably be a SetVpinOk(Vec<AddVpinRow>) or a SetVpinFailed{err: String}
    SetVpin(Vec<Change>),
}

impl ToIMsg for IVpinDialog {
    fn to_imsg(self) -> IMsg {
        IMsg::VpinDialog(self)
    }
}
