//! The main ui is broken up into major components, which
//! may be found in the `components` module, here.

/// The bottom_context_widget provides the bottom stacked widget with a per
/// stack set of controls that are hosted on the toolbar between the versionpin
/// table and the revision view below it.
pub mod bottom_context_widget;
/// The bottom stacked widget housese the change table , the hstory table
pub mod bottom_stacked_widget;

pub mod center_widget;
pub mod dist_tree;
pub mod left_toolbar;
pub mod package_withs_list;
pub mod packages_tree;
/// Part of the history widget that shows changes for a particular revision
pub mod revision_changes_table;
pub mod revisions_table;
pub mod table_headers;
pub mod versionpin_changes_row;
pub mod versionpin_changes_table;
pub mod versionpin_row;
pub mod versionpin_table;
pub mod versionpin_table_splitter;
pub mod withs_splitter;
