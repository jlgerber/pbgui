//! Provides public traits defined by pbgui are defined by the pbgui::traits` module.

/// Define a VersionPinRowTrait in terms of input: T and out put ReturnType.
pub trait RowTrait {
    type ReturnType;
    type SourceTableType;
    /// generate a VersionPinRowTrait<T> from a reference to a versionpin table and a row number.
    /// The function is fallible, returning an Option.
    ///
    /// # Arguments
    /// * `source_table` A reference to the source of the data
    /// * `row` The row number which we want the data for.
    ///
    /// # Returns
    /// * `Some(Self::ReturnType)` if successful
    /// * `None` if unsuccessful
    fn from_table_at_row(
        source_table: &Self::SourceTableType,
        row: i32,
    ) -> Option<Self::ReturnType>;
}

/// Set a row on the target table
pub trait RowSetterTrait {
    type TargetTable;
    /// Set a row on the target table
    ///
    /// # Arguments
    /// * `target_table`: the table to mutate
    /// * `row` - The row index to update or set
    fn set_table_row(&self, target_table: &mut Self::TargetTable, row: i32);
}
