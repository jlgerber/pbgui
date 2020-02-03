use super::*;
use crate::change_type::Change;

#[derive(Debug, PartialEq)]
pub enum OMainWin {
    GetVpins {
        level: String,
        role: String,
        platform: String,
        site: String,
        dir: String,
        package: Option<String>,
    },
    GetWithsForVpin {
        vpin_id: i32,
    },
    GetTransactionChanges {
        tx_id: i32,
    },
    GetHistoryRevisions,
    SaveVpinChanges {
        changes: Vec<Change>,
        user: String,
        comments: String,
    },
    ChooseDistribution {
        package: String,
        version: String,
        row: i32,
    },
    SavePackagesXml {
        show: String,
        output: String,
    },
}

impl ToOMsg for OMainWin {
    fn to_omsg(self) -> OMsg {
        OMsg::MainWin(self)
    }
}
