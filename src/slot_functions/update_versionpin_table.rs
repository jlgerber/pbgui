use crate::constants::*;
use packybara::packrat::{Client, NoTls, PackratDb};
use packybara::LtreeSearchMode;
use qt_core::QVariant;
use qt_widgets::{
    cpp_core::MutPtr, qt_core::QString, QComboBox, QLineEdit, QTableWidget, QTableWidgetItem,
};
use std::str::FromStr;
//-----------------------------------------------//
//            update_vpin_table                  //
//-----------------------------------------------//
// update the main versionpin table by gathering //
// the user's requested query parameters from    //
// the comboboxes up top, querying the database, //
// and updating the table                        //
//-----------------------------------------------//
pub fn update_vpin_table(
    dir_ptr: MutPtr<QComboBox>,
    line_edit_ptr: MutPtr<QLineEdit>,
    level_ptr: MutPtr<QComboBox>,
    role_ptr: MutPtr<QComboBox>,
    platform_ptr: MutPtr<QComboBox>,
    site_ptr: MutPtr<QComboBox>,
    mut vpin_tablewidget_ptr: MutPtr<QTableWidget>,
) {
    unsafe {
        let dirtxt = dir_ptr.current_text().to_std_string();
        let line_edit_txt = line_edit_ptr.text().to_std_string();
        let showtxt = level_ptr.current_text().to_std_string();
        let roletxt = role_ptr.current_text().to_std_string();
        let platformtxt = platform_ptr.current_text().to_std_string();
        let sitetxt = site_ptr.current_text().to_std_string();
        // for now
        let client = Client::connect(
            "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
            NoTls,
        )
        .unwrap();
        let mut packratdb = PackratDb::new(client);
        let mut vpin_finder = packratdb.find_all_versionpins();

        vpin_finder
            .level(showtxt.as_str())
            .role(roletxt.as_str())
            .platform(platformtxt.as_str())
            .site(sitetxt.as_str())
            .search_mode(LtreeSearchMode::from_str(dirtxt.as_str()).unwrap());
        let filter_package = if line_edit_txt != "" { true } else { false };
        let results = vpin_finder.query().unwrap();
        let mut cnt = 0;
        vpin_tablewidget_ptr.set_sorting_enabled(false);
        vpin_tablewidget_ptr.set_row_count(0);
        vpin_tablewidget_ptr.set_row_count(results.len() as i32);
        let mut filtered_cnt = 0;
        for result in results {
            if filter_package && line_edit_txt != "" {
                if !result
                    .distribution
                    .package()
                    .contains(line_edit_txt.as_str())
                {
                    filtered_cnt += 1;
                    continue;
                }
            }
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(result.versionpin_id);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            vpin_tablewidget_ptr.set_item(cnt, COL_ID, vpin_table_widget_item.into_ptr());
            // DISTRIBUTION
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&QString::from_std_str(
                result.distribution.to_string().as_str(),
            ));
            vpin_tablewidget_ptr.set_item(cnt, COL_DISTRIBUTION, vpin_table_widget_item.into_ptr());
            // LEVEL
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&QString::from_std_str(
                result.coords.level.to_string().as_str(),
            ));
            vpin_tablewidget_ptr.set_item(cnt, COL_LEVEL, vpin_table_widget_item.into_ptr());
            // ROLE
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&QString::from_std_str(
                result.coords.role.to_string().as_str(),
            ));
            vpin_tablewidget_ptr.set_item(cnt, COL_ROLE, vpin_table_widget_item.into_ptr());
            // PLATFORM
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&QString::from_std_str(
                result.coords.platform.to_string().as_str(),
            ));
            vpin_tablewidget_ptr.set_item(cnt, COL_PLATFORM, vpin_table_widget_item.into_ptr());
            // SITE
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&QString::from_std_str(
                result.coords.site.to_string().as_str(),
            ));
            vpin_tablewidget_ptr.set_item(cnt, COL_SITE, vpin_table_widget_item.into_ptr());
            // WITHS
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(result.withs.unwrap_or(vec![]).len() as i32);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            vpin_tablewidget_ptr.set_item(cnt, COL_WITHS, vpin_table_widget_item.into_ptr());
            // Coord Id
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(result.distribution_id);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            vpin_tablewidget_ptr.set_item(
                cnt,
                COL_DISTRIBUTION_ID,
                vpin_table_widget_item.into_ptr(),
            );
            vpin_tablewidget_ptr.set_column_hidden(COL_DISTRIBUTION_ID, true);
            // Coord Id
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(result.pkgcoord_id);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            vpin_tablewidget_ptr.set_item(cnt, COL_PKGCOORD_ID, vpin_table_widget_item.into_ptr());
            vpin_tablewidget_ptr.set_column_hidden(COL_PKGCOORD_ID, true);

            cnt += 1;
        }
        if filtered_cnt > 0 {
            let rc = vpin_tablewidget_ptr.row_count() - filtered_cnt;
            vpin_tablewidget_ptr.set_row_count(rc);
        }
        vpin_tablewidget_ptr.set_sorting_enabled(true);
    }
}
