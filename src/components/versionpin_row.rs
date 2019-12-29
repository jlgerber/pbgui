use crate::constants::*;
use crate::utility::qs;
use crate::{RowSetterTrait, RowTrait};
use packybara::db::find_all::versionpins::FindAllVersionPinsRow;
use packybara::types::IdType;
use qt_core::{QString, QVariant};
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    QTableWidget, QTableWidgetItem,
};
use std::fmt;

/// A row of versionpin data
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct VersionPinRow<T> {
    pub id: IdType,
    pub dist_id: IdType,
    pub pkgcoord_id: IdType,
    pub distribution: T,
    pub level: T,
    pub role: T,
    pub platform: T,
    pub site: T,
    pub withs: i32,
}

//
// Implementation of VersionPinRowTrait<CppBox<QString>> and Debug for VersionPinRow<String>
//
impl fmt::Debug for VersionPinRow<String> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VersionPinRow<String>")
            .field("id", &self.id)
            .field("dist_id", &self.dist_id)
            .field("pkgcoord_id", &self.pkgcoord_id) //&format_args!("{}", self.addr))
            .field("distribution", &self.distribution)
            .field("level", &self.level)
            .field("role", &self.role)
            .field("platform", &self.platform)
            .field("site", &self.site)
            .field("withs", &self.withs)
            .finish()
    }
}

impl VersionPinRow<String> {
    pub fn new(
        id: IdType,
        dist_id: IdType,
        pkgcoord_id: IdType,
        distribution: CppBox<QString>,
        level: CppBox<QString>,
        role: CppBox<QString>,
        platform: CppBox<QString>,
        site: CppBox<QString>,
        withs: i32,
    ) -> Self {
        Self {
            id,
            dist_id,
            pkgcoord_id,
            distribution: distribution.to_std_string(),
            level: level.to_std_string(),
            role: role.to_std_string(),
            platform: platform.to_std_string(),
            site: site.to_std_string(),
            withs,
        }
    }
}

impl RowTrait<CppBox<QString>> for VersionPinRow<String> {
    type ReturnType = VersionPinRow<String>;
    type SourceTableType = MutPtr<QTableWidget>;

    /// Given a reference to the versionpin table, and a row number. return the row
    fn from_table_at_row(
        source_table: &Self::SourceTableType,
        row: i32,
    ) -> Option<Self::ReturnType> {
        unsafe {
            if row < 0 || source_table.row_count() <= row {
                log::warn!(
                    "row requested out of bounds: row count:{} requested:{}",
                    source_table.row_count(),
                    row
                );
                return None;
            }
            let vpin_id = source_table.item(row, COL_ID).data(2).to_int_0a();
            let dist_id = source_table
                .item(row, COL_DISTRIBUTION_ID)
                .data(2)
                .to_int_0a();
            let pkgcoord_id = source_table.item(row, COL_PKGCOORD_ID).data(2).to_int_0a();
            let distribtution = source_table.item(row, COL_DISTRIBUTION).text();
            let level = source_table.item(row, COL_LEVEL).text();
            let role = source_table.item(row, COL_ROLE).text();
            let platform = source_table.item(row, COL_PLATFORM).text();
            let site = source_table.item(row, COL_SITE).text();
            let withs = source_table.item(row, COL_WITHS).data(2).to_int_0a();
            Some(VersionPinRow::<String>::new(
                vpin_id,
                dist_id,
                pkgcoord_id,
                distribtution,
                level,
                role,
                platform,
                site,
                withs,
            ))
        }
    }
}

impl RowSetterTrait for VersionPinRow<String> {
    type TargetTable = MutPtr<QTableWidget>;
    fn set_table_row(&self, target_table: &mut Self::TargetTable, row: i32) {
        unsafe {
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.id);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            target_table.set_item(row, COL_ID, vpin_table_widget_item.into_ptr());
            // DISTRIBUTION
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&qs(&self.distribution));
            target_table.set_item(row, COL_DISTRIBUTION, vpin_table_widget_item.into_ptr());
            // LEVEL
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&qs(&self.level));
            target_table.set_item(row, COL_LEVEL, vpin_table_widget_item.into_ptr());
            // ROLE
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&qs(&self.role));
            target_table.set_item(row, COL_ROLE, vpin_table_widget_item.into_ptr());
            // PLATFORM
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&qs(&self.platform));
            target_table.set_item(row, COL_PLATFORM, vpin_table_widget_item.into_ptr());
            // SITE
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&qs(&self.site));
            target_table.set_item(row, COL_SITE, vpin_table_widget_item.into_ptr());
            // WITHS
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.withs);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            target_table.set_item(row, COL_WITHS, vpin_table_widget_item.into_ptr());
            // Coord Id
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.dist_id);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            target_table.set_item(row, COL_DISTRIBUTION_ID, vpin_table_widget_item.into_ptr());
            target_table.set_column_hidden(COL_DISTRIBUTION_ID, true);
            // Coord Id
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.pkgcoord_id);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            target_table.set_item(row, COL_PKGCOORD_ID, vpin_table_widget_item.into_ptr());
            target_table.set_column_hidden(COL_PKGCOORD_ID, true);
        }
    }
}

//
// Implementation of VersionPinRowTrait<CppBox<QString>> and Debug for VersionPinRow<CppBox<QString>>
//
impl fmt::Debug for VersionPinRow<CppBox<QString>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VersionPinRow<String>")
            .field("id", &self.id)
            .field("dist_id", &self.dist_id)
            .field("pkgcoord_id", &self.pkgcoord_id) //&format_args!("{}", self.addr))
            .field("distribution", &self.distribution.to_std_string())
            .field("level", &self.level.to_std_string())
            .field("role", &self.role.to_std_string())
            .field("platform", &self.platform.to_std_string())
            .field("site", &self.site.to_std_string())
            .field("withs", &self.withs)
            .finish()
    }
}

impl VersionPinRow<CppBox<QString>> {
    pub fn new(
        id: IdType,
        dist_id: IdType,
        pkgcoord_id: IdType,
        distribution: CppBox<QString>,
        level: CppBox<QString>,
        role: CppBox<QString>,
        platform: CppBox<QString>,
        site: CppBox<QString>,
        withs: i32,
    ) -> Self {
        Self {
            id,
            dist_id,
            pkgcoord_id,
            distribution,
            level,
            role,
            platform,
            site,
            withs,
        }
    }
}
impl RowTrait<CppBox<QString>> for VersionPinRow<CppBox<QString>> {
    type ReturnType = VersionPinRow<CppBox<QString>>;
    type SourceTableType = MutPtr<QTableWidget>;

    /// Given a reference to the versionpin table, and a row number. return the row
    fn from_table_at_row(
        source_table: &Self::SourceTableType,
        row: i32,
    ) -> Option<Self::ReturnType> {
        unsafe {
            if row < 0 || source_table.row_count() <= row {
                log::warn!(
                    "row requested out of bounds: row count:{} requested:{}",
                    source_table.row_count(),
                    row
                );
                return None;
            }
            let vpin_id = source_table.item(row, COL_ID).data(2).to_int_0a();
            let dist_id = source_table
                .item(row, COL_DISTRIBUTION_ID)
                .data(2)
                .to_int_0a();
            let pkgcoord_id = source_table.item(row, COL_PKGCOORD_ID).data(2).to_int_0a();
            let distribtution = source_table.item(row, COL_DISTRIBUTION).text();
            let level = source_table.item(row, COL_LEVEL).text();
            let role = source_table.item(row, COL_ROLE).text();
            let platform = source_table.item(row, COL_PLATFORM).text();
            let site = source_table.item(row, COL_SITE).text();
            let withs = source_table.item(row, COL_WITHS).data(2).to_int_0a();
            Some(VersionPinRow::<CppBox<QString>>::new(
                vpin_id,
                dist_id,
                pkgcoord_id,
                distribtution,
                level,
                role,
                platform,
                site,
                withs,
            ))
        }
    }
}

impl RowSetterTrait for VersionPinRow<CppBox<QString>> {
    type TargetTable = MutPtr<QTableWidget>;
    fn set_table_row(&self, target_table: &mut Self::TargetTable, row: i32) {
        unsafe {
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.id);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            target_table.set_item(row, COL_ID, vpin_table_widget_item.into_ptr());
            // DISTRIBUTION
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&self.distribution);
            target_table.set_item(row, COL_DISTRIBUTION, vpin_table_widget_item.into_ptr());
            // LEVEL
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&self.level);
            target_table.set_item(row, COL_LEVEL, vpin_table_widget_item.into_ptr());
            // ROLE
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&self.role);
            target_table.set_item(row, COL_ROLE, vpin_table_widget_item.into_ptr());
            // PLATFORM
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&self.platform);
            target_table.set_item(row, COL_PLATFORM, vpin_table_widget_item.into_ptr());
            // SITE
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&self.site);
            target_table.set_item(row, COL_SITE, vpin_table_widget_item.into_ptr());
            // WITHS
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.withs);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            target_table.set_item(row, COL_WITHS, vpin_table_widget_item.into_ptr());
            // Coord Id
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.dist_id);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            target_table.set_item(row, COL_DISTRIBUTION_ID, vpin_table_widget_item.into_ptr());
            target_table.set_column_hidden(COL_DISTRIBUTION_ID, true);
            // Coord Id
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.pkgcoord_id);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            target_table.set_item(row, COL_PKGCOORD_ID, vpin_table_widget_item.into_ptr());
            target_table.set_column_hidden(COL_PKGCOORD_ID, true);
        }
    }
}
//
//
//
impl RowSetterTrait for FindAllVersionPinsRow {
    type TargetTable = MutPtr<QTableWidget>;

    fn set_table_row(&self, target_table: &mut Self::TargetTable, row: i32) {
        unsafe {
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.versionpin_id);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            target_table.set_item(row, COL_ID, vpin_table_widget_item.into_ptr());
            // DISTRIBUTION
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&QString::from_std_str(
                self.distribution.to_string().as_str(),
            ));
            target_table.set_item(row, COL_DISTRIBUTION, vpin_table_widget_item.into_ptr());
            // LEVEL
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&QString::from_std_str(
                self.coords.level.to_string().as_str(),
            ));
            target_table.set_item(row, COL_LEVEL, vpin_table_widget_item.into_ptr());
            // ROLE
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&QString::from_std_str(
                self.coords.role.to_string().as_str(),
            ));
            target_table.set_item(row, COL_ROLE, vpin_table_widget_item.into_ptr());
            // PLATFORM
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&QString::from_std_str(
                self.coords.platform.to_string().as_str(),
            ));
            target_table.set_item(row, COL_PLATFORM, vpin_table_widget_item.into_ptr());
            // SITE
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&QString::from_std_str(
                self.coords.site.to_string().as_str(),
            ));
            target_table.set_item(row, COL_SITE, vpin_table_widget_item.into_ptr());
            // WITHS
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.withs.as_ref().unwrap_or(&vec![]).len() as i32);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            target_table.set_item(row, COL_WITHS, vpin_table_widget_item.into_ptr());
            // Coord Id
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.distribution_id);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            target_table.set_item(row, COL_DISTRIBUTION_ID, vpin_table_widget_item.into_ptr());
            target_table.set_column_hidden(COL_DISTRIBUTION_ID, true);
            // Coord Id
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.pkgcoord_id);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            target_table.set_item(row, COL_PKGCOORD_ID, vpin_table_widget_item.into_ptr());
            target_table.set_column_hidden(COL_PKGCOORD_ID, true);
        }
    }
}
