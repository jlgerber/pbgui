use crate::change_type::ChangeType;
use crate::constants::*;
use crate::utility::qs;
pub use crate::{RowSetterTrait, RowTrait};
use log;
use qt_core::QString;
use qt_thread_conductor::traits::FromQString;
use qt_thread_conductor::traits::ToQString;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr, Ref},
    QTableWidget, QTableWidgetItem,
};
use std::fmt;

/// A row of versionpin data
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct VersionPinChangesRow<T> {
    pub change_type: ChangeType,
    pub context: T,
    pub old_value: T,
    pub new_value: T,
}

//
// VersionPinChangesRow<String>
//
impl VersionPinChangesRow<String> {
    /// Generate a new VersionPinChangeRow<String>
    pub fn new(
        change_type: ChangeType,
        context: Ref<QString>,
        old_value: Ref<QString>,
        new_value: Ref<QString>,
    ) -> Self {
        Self {
            change_type,
            context: context.to_std_string(),
            old_value: old_value.to_std_string(),
            new_value: new_value.to_std_string(),
        }
    }
}

impl fmt::Debug for VersionPinChangesRow<String> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VersionPinChnagesRow<String>")
            .field("change_type", &self.change_type)
            .field("context", &self.context)
            .field("old_value", &self.old_value)
            .field("new_value", &self.new_value)
            .finish()
    }
}

impl RowTrait for VersionPinChangesRow<String> {
    type ReturnType = VersionPinChangesRow<String>;
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
            let change_type = source_table.item(row, COL_PC_CHANGETYPE).text();
            let context = source_table.item(row, COL_PC_CONTEXT).text();
            let old_value = source_table.item(row, COL_PC_OLD_VALUE).text();
            let new_value = source_table.item(row, COL_PC_NEW_VALUE).text();
            Some(VersionPinChangesRow::<String>::new(
                ChangeType::from_qstring(change_type.as_ref()),
                context.as_ref(),
                old_value.as_ref(),
                new_value.as_ref(),
            ))
        }
    }
}

impl RowSetterTrait for VersionPinChangesRow<String> {
    type TargetTable = MutPtr<QTableWidget>;
    fn set_table_row(&self, target_table: &mut Self::TargetTable, row: i32) {
        unsafe {
            if target_table.row_count() == row {
                target_table.set_row_count(row + 1);
            }
            // CHANGETYPE
            let mut table_widget_item = QTableWidgetItem::new();
            table_widget_item.set_text(&qs(&self.change_type));
            target_table.set_item(row, COL_PC_CHANGETYPE, table_widget_item.into_ptr());
            // CONTEXT
            let mut table_widget_item = QTableWidgetItem::new();
            table_widget_item.set_text(&qs(&self.context));
            target_table.set_item(row, COL_PC_CONTEXT, table_widget_item.into_ptr());

            // OLD_VALUE
            let mut table_widget_item = QTableWidgetItem::new();
            table_widget_item.set_text(&qs(&self.old_value));
            target_table.set_item(row, COL_PC_OLD_VALUE, table_widget_item.into_ptr());

            // new_VALUE
            let mut table_widget_item = QTableWidgetItem::new();
            table_widget_item.set_text(&qs(&self.new_value));
            target_table.set_item(row, COL_PC_NEW_VALUE, table_widget_item.into_ptr());
        }
    }
}
//
// CppBox<QString>
//
impl VersionPinChangesRow<CppBox<QString>> {
    /// Generate a new VersionPinChangeRow<String>
    pub fn new(
        change_type: ChangeType,
        context: CppBox<QString>,
        old_value: CppBox<QString>,
        new_value: CppBox<QString>,
    ) -> Self {
        Self {
            change_type,
            context: context,
            old_value: old_value,
            new_value: new_value,
        }
    }
}

impl fmt::Debug for VersionPinChangesRow<CppBox<QString>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VersionPinChnagesRow<String>")
            .field("change_type", &self.change_type)
            .field("context", &self.context.to_std_string())
            .field("old_value", &self.old_value.to_std_string())
            .field("new_value", &self.new_value.to_std_string())
            .finish()
    }
}

impl RowTrait for VersionPinChangesRow<CppBox<QString>> {
    type ReturnType = VersionPinChangesRow<CppBox<QString>>;
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
            let change_type = source_table.item(row, COL_PC_CHANGETYPE).text();
            let context = source_table.item(row, COL_PC_CONTEXT).text();
            let old_value = source_table.item(row, COL_PC_OLD_VALUE).text();
            let new_value = source_table.item(row, COL_PC_NEW_VALUE).text();
            Some(VersionPinChangesRow::<CppBox<QString>>::new(
                ChangeType::from_qstring(change_type.as_ref()),
                context,
                old_value,
                new_value,
            ))
        }
    }
}

impl RowSetterTrait for VersionPinChangesRow<CppBox<QString>> {
    type TargetTable = MutPtr<QTableWidget>;
    fn set_table_row(&self, target_table: &mut Self::TargetTable, row: i32) {
        unsafe {
            if target_table.row_count() == row {
                target_table.set_row_count(row + 1);
            }
            // DISTRIBUTION
            let mut table_widget_item = QTableWidgetItem::new();
            table_widget_item.set_text(&self.change_type.to_qstring());
            target_table.set_item(row, COL_PC_CHANGETYPE, table_widget_item.into_ptr());
            // CONTEXT
            let mut table_widget_item = QTableWidgetItem::new();
            table_widget_item.set_text(&self.context);
            target_table.set_item(row, COL_PC_CONTEXT, table_widget_item.into_ptr());

            // OLD_VALUE
            let mut table_widget_item = QTableWidgetItem::new();
            table_widget_item.set_text(&self.old_value);
            target_table.set_item(row, COL_PC_OLD_VALUE, table_widget_item.into_ptr());

            let mut table_widget_item = QTableWidgetItem::new();
            table_widget_item.set_text(&qs("->"));
            target_table.set_item(row, COL_PC_BECOMES, table_widget_item.into_ptr());

            // new_VALUE
            let mut table_widget_item = QTableWidgetItem::new();
            table_widget_item.set_text(&self.new_value);
            target_table.set_item(row, COL_PC_NEW_VALUE, table_widget_item.into_ptr());
        }
    }
}
