//! slot_functions do the heavy lifting for slots.
//! Most slot closures delegate repsonsbility to a slot function.
//! At a minimum, it makes the main_window a bit more readable
pub mod choose_distribution;
pub mod save_versionpin_changes;
pub mod select_history;
pub mod store_withpackage_changes;
pub mod update_changes_table;
pub mod update_versionpin_table;
pub mod update_withpackages;
