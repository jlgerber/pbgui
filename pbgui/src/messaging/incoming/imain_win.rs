use super::*;
use packybara::db::find_all::changes::FindAllChangesRow;
use packybara::db::find_all::distributions::FindAllDistributionsRow;
use packybara::db::find_all::revisions::FindAllRevisionsRow;
use packybara::db::find_all::versionpin_withs::FindAllWithsRow;
use packybara::db::find_all::versionpins::FindAllVersionPinsRow;

/// Responsies returning to the main gui thread from the secondary thread
/// for the main window.
pub enum IMainWin {
    /// Returns a vector of versionpin row data
    Vpins(Vec<FindAllVersionPinsRow>),
    /// Returns the results of querying with packages maching a
    /// query rpovided to OMainWin
    WithPackages(Vec<FindAllWithsRow>),
    /// Returns a vector of Changes
    Changes(Vec<FindAllChangesRow>),
    /// Returns a vector of revision rows
    HistoryRevisions(Vec<FindAllRevisionsRow>),
    /// Returns success/faliure after updating the database with versionpin
    /// changes previously stashed in the vpin changes table.
    SaveVpinChanges(bool), //consider changing to Result<(),>
    /// Returns the results of searching for alternative distributions
    /// and provides the original package, version and row from the
    /// versionpin table.
    ChooseDistribution {
        /// list of distributions found
        distributions: Vec<FindAllDistributionsRow>,
        /// for package:
        package: String,
        /// with original version
        version: String,
        /// found in versionpin table at row
        row: i32,
    },
    /// Returns a bool indicating success or faliure after attempting to
    /// save state as packages.xml to a provided location
    SavePackagesXml(bool),
}

impl ToIMsg for IMainWin {
    fn to_imsg(self) -> IMsg {
        IMsg::MainWin(self)
    }
}
