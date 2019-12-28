use crate::change_type::ChangeType;
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
    /// New up a VersionPinRowTrait of type T
    ///
    /// # Arguments
    /// * `id` the Versionpin Id that the row is describing
    /// * `dist_id` The distribution's id
    /// * `pkgcoord_id` - The package coordinate's id.
    /// * `level` - The level. Generally a show name or facility
    /// * `role` - The role
    /// * `platform` - The platform (os)
    /// * `site` - The site that the versionpin reps
    /// * `withs` - The number of with packages that the versionpin has associated with it
    ///
    /// # Returns
    /// * A VersionPinRowTrait<T>
    fn new(
        change_type: ChangeType,
        vpin_id: IdType,
        dist_id: IdType,
        pkgcoord_id: IdType,
        distribution: T,
        level: T,
        role: T,
        platform: T,
        site: T,
        withs: i32,
    ) -> Self;
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
        versionpin_table: &MutPtr<QTableWidget>,
        row: i32,
    ) -> Option<Self::ReturnType>;

    //fn set_table_row(&self, versionpin_table: &mut MutPtr<QTableWidget>, row: i32);
}

/// Set a the table row
pub trait VersionPinRowSetterTrait {
    fn set_table_row(&self, versionpin_table: &mut MutPtr<QTableWidget>, row: i32);
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
