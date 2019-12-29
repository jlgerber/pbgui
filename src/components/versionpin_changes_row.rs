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

/// Define a VersionPinChangesRowTrait in terms of input: T and out put ReturnType.
pub trait VersionPinChangesRowTrait<T> {
    type ReturnType;
    type TableType;
    /// New up a VersionPinChangesRowTrait of type T
    ///
    /// # Arguments
    /// * `id` the Versionpin Id that the row is describing
    /// * `context` identify the versionpin by some means
    /// * `old_value` - the older value .
    /// * `new_value` - The value we wish to change to
    ///
    /// # Returns
    /// * A VersionPinChangesRow<T>
    fn new(change_type: ChangeType, context: T, old_value: T, new_value: T) -> Self;
    /// generate a VersionPinChangesRowTrait<T> from a reference to a versionpin table and a row number.
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
        versionpin_table: &Self::TabelType, //MutPtr<QTableWidget>
        row: i32,
    ) -> Option<Self::ReturnType>;
}

/// Set a the table row
pub trait VersionPinChangesRowSetterTrait {
    fn set_table_row(&self, versionpin_changes_table: &mut MutPtr<QTableWidget>, row: i32);
}
/// A row of versionpin data
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct VersionPinChangesRow<T> {
    pub vpin_id: IdType,
    pub dist_id: IdType,
    pub pkgcoord_id: IdType,
    pub display: T,
}
