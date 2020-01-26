use super::*;
use packybara::db::find_all::changes::FindAllChangesRow;
use packybara::db::find_all::distributions::FindAllDistributionsRow;
use packybara::db::find_all::revisions::FindAllRevisionsRow;
use packybara::db::find_all::versionpin_withs::FindAllWithsRow;
use packybara::db::find_all::versionpins::FindAllVersionPinsRow;

pub enum IMainWin {
    Vpins(Vec<FindAllVersionPinsRow>),
    WithPackages(Vec<FindAllWithsRow>),
    Changes(Vec<FindAllChangesRow>),
    HistoryRevisions(Vec<FindAllRevisionsRow>),
    SaveVpinChanges(bool), //consider changing to Result<(),>
    ChooseDistribution {
        distributions: Vec<FindAllDistributionsRow>,
        package: String,
        version: String,
        row: i32,
    },
}

impl ToIMsg for IMainWin {
    fn to_imsg(self) -> IMsg {
        IMsg::MainWin(self)
    }
}
