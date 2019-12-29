use crate::constants::*;
use crate::utility::qs;
use packybara::db::find_all::versionpins::FindAllVersionPinsRow;
use packybara::types::IdType;
use qt_core::{QString, QVariant};
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    QTableWidget, QTableWidgetItem,
};
use std::fmt;

/// Define a VersionPinRowTrait in terms of input: T and out put ReturnType.
pub trait VersionPinRowTrait<T> {
    type ReturnType;
    type TableType;
    // /// New up a VersionPinRowTrait of type T
    // ///
    // /// # Arguments
    // /// * `id` the Versionpin Id that the row is describing
    // /// * `dist_id` The distribution's id
    // /// * `pkgcoord_id` - The package coordinate's id.
    // /// * `level` - The level. Generally a show name or facility
    // /// * `role` - The role
    // /// * `platform` - The platform (os)
    // /// * `site` - The site that the versionpin reps
    // /// * `withs` - The number of with packages that the versionpin has associated with it
    // ///
    // /// # Returns
    // /// * A VersionPinRowTrait<T>
    // fn new(
    //     id: IdType,
    //     dist_id: IdType,
    //     pkgcoord_id: IdType,
    //     distribution: T,
    //     level: T,
    //     role: T,
    //     platform: T,
    //     site: T,
    //     withs: i32,
    // ) -> Self;
    /// generate a VersionPinRowTrait<T> from a reference to a versionpin table and a row number.
    /// The function is fallible, returning an Option.
    ///
    /// # Arguments
    /// * `versionpin_table` A reference to the source of the data
    /// * `row` The row number which we want the data for.
    ///
    /// # Returns
    /// * `Some(Self::ReturnType)` if successful
    /// * `None` if unsuccessful
    fn from_table_at_row(
        versionpin_table: &Self::TableType, //MutPtr<QTableWidget>,
        row: i32,
    ) -> Option<Self::ReturnType>;

    //fn set_table_row(&self, versionpin_table: &mut MutPtr<QTableWidget>, row: i32);
}

/// Set a the table row
pub trait VersionPinRowSetterTrait {
    type TargetTable; //MutPtr<QTableWidget>
    fn set_table_row(&self, versionpin_table: &mut Self::TargetTable, row: i32);
}
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
//
//
//

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

impl VersionPinRowTrait<CppBox<QString>> for VersionPinRow<String> {
    type ReturnType = VersionPinRow<String>;
    type TableType = MutPtr<QTableWidget>;

    /// Given a reference to the versionpin table, and a row number. return the row
    fn from_table_at_row(
        versionpin_table: &MutPtr<QTableWidget>,
        row: i32,
    ) -> Option<Self::ReturnType> {
        unsafe {
            if row < 0 || versionpin_table.row_count() <= row {
                log::warn!(
                    "row requested out of bounds: row count:{} requested:{}",
                    versionpin_table.row_count(),
                    row
                );
                return None;
            }
            let vpin_id = versionpin_table.item(row, COL_ID).data(2).to_int_0a();
            let dist_id = versionpin_table
                .item(row, COL_DISTRIBUTION_ID)
                .data(2)
                .to_int_0a();
            let pkgcoord_id = versionpin_table
                .item(row, COL_PKGCOORD_ID)
                .data(2)
                .to_int_0a();
            let distribtution = versionpin_table.item(row, COL_DISTRIBUTION).text();
            let level = versionpin_table.item(row, COL_LEVEL).text();
            let role = versionpin_table.item(row, COL_ROLE).text();
            let platform = versionpin_table.item(row, COL_PLATFORM).text();
            let site = versionpin_table.item(row, COL_SITE).text();
            let withs = versionpin_table.item(row, COL_WITHS).data(2).to_int_0a();
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

impl VersionPinRowSetterTrait for VersionPinRow<String> {
    type TargetTable = MutPtr<QTableWidget>;
    fn set_table_row(&self, versionpin_table: &mut Self::TargetTable, row: i32) {
        unsafe {
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.id);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            versionpin_table.set_item(row, COL_ID, vpin_table_widget_item.into_ptr());
            // DISTRIBUTION
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&qs(&self.distribution));
            versionpin_table.set_item(row, COL_DISTRIBUTION, vpin_table_widget_item.into_ptr());
            // LEVEL
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&qs(&self.level));
            versionpin_table.set_item(row, COL_LEVEL, vpin_table_widget_item.into_ptr());
            // ROLE
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&qs(&self.role));
            versionpin_table.set_item(row, COL_ROLE, vpin_table_widget_item.into_ptr());
            // PLATFORM
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&qs(&self.platform));
            versionpin_table.set_item(row, COL_PLATFORM, vpin_table_widget_item.into_ptr());
            // SITE
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&qs(&self.site));
            versionpin_table.set_item(row, COL_SITE, vpin_table_widget_item.into_ptr());
            // WITHS
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.withs);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            versionpin_table.set_item(row, COL_WITHS, vpin_table_widget_item.into_ptr());
            // Coord Id
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.dist_id);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            versionpin_table.set_item(row, COL_DISTRIBUTION_ID, vpin_table_widget_item.into_ptr());
            versionpin_table.set_column_hidden(COL_DISTRIBUTION_ID, true);
            // Coord Id
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.pkgcoord_id);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            versionpin_table.set_item(row, COL_PKGCOORD_ID, vpin_table_widget_item.into_ptr());
            versionpin_table.set_column_hidden(COL_PKGCOORD_ID, true);
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
impl VersionPinRowTrait<CppBox<QString>> for VersionPinRow<CppBox<QString>> {
    type ReturnType = VersionPinRow<CppBox<QString>>;
    type TableType = MutPtr<QTableWidget>;

    /// Given a reference to the versionpin table, and a row number. return the row
    fn from_table_at_row(
        versionpin_table: &MutPtr<QTableWidget>,
        row: i32,
    ) -> Option<Self::ReturnType> {
        unsafe {
            if row < 0 || versionpin_table.row_count() <= row {
                log::warn!(
                    "row requested out of bounds: row count:{} requested:{}",
                    versionpin_table.row_count(),
                    row
                );
                return None;
            }
            let vpin_id = versionpin_table.item(row, COL_ID).data(2).to_int_0a();
            let dist_id = versionpin_table
                .item(row, COL_DISTRIBUTION_ID)
                .data(2)
                .to_int_0a();
            let pkgcoord_id = versionpin_table
                .item(row, COL_PKGCOORD_ID)
                .data(2)
                .to_int_0a();
            let distribtution = versionpin_table.item(row, COL_DISTRIBUTION).text();
            let level = versionpin_table.item(row, COL_LEVEL).text();
            let role = versionpin_table.item(row, COL_ROLE).text();
            let platform = versionpin_table.item(row, COL_PLATFORM).text();
            let site = versionpin_table.item(row, COL_SITE).text();
            let withs = versionpin_table.item(row, COL_WITHS).data(2).to_int_0a();
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
impl VersionPinRowSetterTrait for VersionPinRow<CppBox<QString>> {
    type TargetTable = MutPtr<QTableWidget>;
    fn set_table_row(&self, versionpin_table: &mut Self::TargetTable, row: i32) {
        unsafe {
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.id);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            versionpin_table.set_item(row, COL_ID, vpin_table_widget_item.into_ptr());
            // DISTRIBUTION
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&self.distribution);
            versionpin_table.set_item(row, COL_DISTRIBUTION, vpin_table_widget_item.into_ptr());
            // LEVEL
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&self.level);
            versionpin_table.set_item(row, COL_LEVEL, vpin_table_widget_item.into_ptr());
            // ROLE
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&self.role);
            versionpin_table.set_item(row, COL_ROLE, vpin_table_widget_item.into_ptr());
            // PLATFORM
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&self.platform);
            versionpin_table.set_item(row, COL_PLATFORM, vpin_table_widget_item.into_ptr());
            // SITE
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&self.site);
            versionpin_table.set_item(row, COL_SITE, vpin_table_widget_item.into_ptr());
            // WITHS
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.withs);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            versionpin_table.set_item(row, COL_WITHS, vpin_table_widget_item.into_ptr());
            // Coord Id
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.dist_id);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            versionpin_table.set_item(row, COL_DISTRIBUTION_ID, vpin_table_widget_item.into_ptr());
            versionpin_table.set_column_hidden(COL_DISTRIBUTION_ID, true);
            // Coord Id
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.pkgcoord_id);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            versionpin_table.set_item(row, COL_PKGCOORD_ID, vpin_table_widget_item.into_ptr());
            versionpin_table.set_column_hidden(COL_PKGCOORD_ID, true);
        }
    }
}
//
//
//
impl VersionPinRowSetterTrait for FindAllVersionPinsRow {
    type TargetTable = MutPtr<QTableWidget>;

    fn set_table_row(&self, versionpin_table: &mut Self::TargetTable, row: i32) {
        unsafe {
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.versionpin_id);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            versionpin_table.set_item(row, COL_ID, vpin_table_widget_item.into_ptr());
            // DISTRIBUTION
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&QString::from_std_str(
                self.distribution.to_string().as_str(),
            ));
            versionpin_table.set_item(row, COL_DISTRIBUTION, vpin_table_widget_item.into_ptr());
            // LEVEL
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&QString::from_std_str(
                self.coords.level.to_string().as_str(),
            ));
            versionpin_table.set_item(row, COL_LEVEL, vpin_table_widget_item.into_ptr());
            // ROLE
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&QString::from_std_str(
                self.coords.role.to_string().as_str(),
            ));
            versionpin_table.set_item(row, COL_ROLE, vpin_table_widget_item.into_ptr());
            // PLATFORM
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&QString::from_std_str(
                self.coords.platform.to_string().as_str(),
            ));
            versionpin_table.set_item(row, COL_PLATFORM, vpin_table_widget_item.into_ptr());
            // SITE
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            vpin_table_widget_item.set_text(&QString::from_std_str(
                self.coords.site.to_string().as_str(),
            ));
            versionpin_table.set_item(row, COL_SITE, vpin_table_widget_item.into_ptr());
            // WITHS
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.withs.as_ref().unwrap_or(&vec![]).len() as i32);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            versionpin_table.set_item(row, COL_WITHS, vpin_table_widget_item.into_ptr());
            // Coord Id
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.distribution_id);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            versionpin_table.set_item(row, COL_DISTRIBUTION_ID, vpin_table_widget_item.into_ptr());
            versionpin_table.set_column_hidden(COL_DISTRIBUTION_ID, true);
            // Coord Id
            let mut vpin_table_widget_item = QTableWidgetItem::new();
            let variant = QVariant::from_int(self.pkgcoord_id);
            vpin_table_widget_item.set_data(
                2, // EditRole
                variant.as_ref(),
            );
            versionpin_table.set_item(row, COL_PKGCOORD_ID, vpin_table_widget_item.into_ptr());
            versionpin_table.set_column_hidden(COL_PKGCOORD_ID, true);
        }
    }
}
