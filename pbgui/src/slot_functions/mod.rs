//! The slot_functions do the heavy lifting for our qt slots, which are implemented
//! as rust closures in each component's constructor funtion, and thus should be brief,
//! for readability's sake. Thus we define the heavy lifting in terms of a
//! `slot_function`, found in `pbgui::slot_functions`.
pub(crate) mod choose_distribution;
pub(crate) mod save_packages_xml;
pub(crate) mod save_versionpin_changes;
pub(crate) mod select_history;
pub(crate) mod store_withpackage_changes;
pub(crate) mod update_changes_table;
pub(crate) mod update_versionpin_table;
pub(crate) mod update_withpackages;
