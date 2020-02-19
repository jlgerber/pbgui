use super::*;
use crate::change_type::Change;
use crate::SearchMode;

#[derive(Debug, PartialEq)]
pub enum OMainWin {
    /// Request a list of versionpins
    GetVpins {
        /// given a SearchMode (ancestor, descendant, exact)
        mode: SearchMode,
        /// and a list of constraints to shape the results.
        package: Option<String>,
        level: String,
        role: String,
        platform: String,
        site: String,
        dir: String,
    },
    /// Request a list of withs
    GetWithsForVpin {
        /// Given a versionpin database id.
        vpin_id: i32,
    },
    /// Request a list of changes in the database
    GetTransactionChanges {
        /// for a given transaction id.
        tx_id: i32,
    },
    /// Request the list of historical revisions from the db.
    GetHistoryRevisions,
    /// save all proposed versionpin changes,
    SaveVpinChanges {
        /// given a vector of Change instances,
        changes: Vec<Change>,
        /// the user making the changes
        user: String,
        /// And the user's comments regarding the changes.
        comments: String,
    },
    /// choose an alternate distribution for an existing
    /// one from the versionpin table.
    ChooseDistribution {
        /// The package
        package: String,
        /// and version
        version: String,
        /// found at versionpin table row in he versionpin table.
        row: i32,
    },
    /// Save a packages.xml
    SavePackagesXml {
        /// For a given show
        show: String,
        /// to a specified location
        output: String,
    },
}

impl ToOMsg for OMainWin {
    fn to_omsg(self) -> OMsg {
        OMsg::MainWin(self)
    }
}
