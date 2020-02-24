use crate::constants::*;
use crate::messaging::outgoing::omain_win::OMainWin;
use crate::messaging::OMsg;
use crate::messaging::Sender;
pub use crate::utility::qs;
use log;
use qt_widgets::{cpp_core::MutPtr, QTableWidget, QWidget};

// Choose an alternative distribution from a list of distributions in a popup dialog.
pub fn choose_alternative_distribution(
    row: i32,
    versionpin_table: MutPtr<QTableWidget>,
    root_widget: MutPtr<QWidget>,
    versionpin_changes_table: MutPtr<QTableWidget>,
    to_thread_sender: Sender<OMsg>,
) {
    unsafe {
        if !validate_ptrs(versionpin_table, &root_widget, versionpin_changes_table) {
            return;
        }
        // all we need is the package... perhaps we should change the vtable
        // TODO: show package & version as separate columns in versionpin table.
        let distribution = versionpin_table.item(row, COL_DISTRIBUTION);
        let orig_vpin_table_distribution = distribution.text().to_std_string();
        // split up the distribution into the package name
        // and the version
        let (package, version) = if let [package, version] =
            *orig_vpin_table_distribution.split('-').collect::<Vec<_>>()
        {
            (package, version)
        } else {
            panic!("unable to extract package and version from row");
        };
        log::debug!("signaling ChooseDistribution");
        to_thread_sender
            .send(OMsg::MainWin(OMainWin::ChooseDistribution {
                package: package.to_string(),
                version: version.to_string(),
                row,
            }))
            .expect("unable to get history revisions");
    }
}

// perform validation on the pointer inputs
fn validate_ptrs(
    versionpin_table: MutPtr<QTableWidget>,
    root_widget: &MutPtr<QWidget>,
    versionpin_changes_table: MutPtr<QTableWidget>,
) -> bool {
    if versionpin_table.is_null() {
        log::error!("versionpin_table is null");
        return false;
    }
    if root_widget.is_null() {
        log::error!("root_widget is null");
        return false;
    }
    if versionpin_changes_table.is_null() {
        log::error!("versionpin_changes_table is null. returning");
        return false;
    }
    true
}

// //
// mod distribution_version_change {
//     use super::*;

//     pub(super) fn build_changestr(
//         package: QRef<QString>,
//         original_version: QRef<QString>,
//         new_version: QRef<QString>,
//         level: QRef<QString>,
//         role: QRef<QString>,
//         platform: QRef<QString>,
//         site: QRef<QString>,
//     ) -> CppBox<QString> {
//         unsafe {
//             let changestr = qs(
//                 "%1-%2      ->      %1-%3        (level: %4,  role: %5,  platform: %6,  site: %7)",
//             )
//             .arg_7_q_string(
//                 package,
//                 original_version,
//                 new_version,
//                 level,
//                 role,
//                 platform,
//                 site,
//             );
//             changestr
//         }
//     }
// }

// Construct a qstringlist of versions, identify the index of the currently selected version,
// and provide a hasmap mapping the version to the id
// fn build_qstring_list_and_map(
//     version: &str,
//     results: Vec<FindAllDistributionsRow>,
// ) -> (CppBox<QStringList>, i32, HashMap<String, IdType>) {
//     unsafe {
//         let mut versions_list = QStringList::new();
//         let mut idx = 0;
//         let mut cnt = 0;
//         let mut dist_versions = HashMap::new();
//         for r in results {
//             if r.version == version {
//                 idx = cnt;
//             }
//             cnt += 1;
//             dist_versions.insert(r.version.clone(), r.id);
//             versions_list.append_q_string(&QString::from_std_str(r.version));
//         }
//         (versions_list, idx, dist_versions)
//     }
// }

// // Given
// fn package_and_version_from_dist<'a, T>(dist: T) -> (String, String)
// where
//     T: Into<QRef<QString>>,
// {
//     let qstr = dist.into();
//     let orig_vpin_table_distribution = qstr.to_std_string();
//     if let &[package, version] = &*orig_vpin_table_distribution.split("-").collect::<Vec<_>>() {
//         (package.to_string(), version.to_string())
//     } else {
//         panic!("unable to extract package and version from row");
//     }
// }
