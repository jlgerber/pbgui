use qt_widgets::{cpp_core::MutPtr, QListWidget, QTableWidget};

/// add the withpackage change to the list
pub fn store_withpackage_changes(
    withpackage_list: MutPtr<QListWidget>,
    versionpin_table: MutPtr<QTableWidget>,
    changes_table: MutPtr<QTableWidget>,
) {
    unsafe {
        // build up a list of packages in order as a vec of String
        let mut items = Vec::with_capacity(withpackage_list.count() as usize);
        for cnt in 0..withpackage_list.count() {
            let item = withpackage_list.item(cnt);
            items.push(item.text().to_std_string());
        }
        // get current versionpin distribution_id
        println!("{:?}", items);
    }
}
