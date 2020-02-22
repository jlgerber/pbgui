//! The main ui is broken up into major components, which
//! may be found in the `components` module, here.

/// The bottom_context_widget provides the bottom stacked widget with a per
/// stack set of controls that are hosted on the toolbar between the versionpin
/// table and the revision view below it.
pub(crate) mod bottom_context_widget;
/// The bottom stacked widget housese the change table , the hstory table
pub(crate) mod bottom_stacked_widget;

pub(crate) mod center_widget;
pub(crate) mod dist_tree;
pub(crate) mod left_toolbar;
pub(crate) mod package_withs_list;
pub(crate) mod packages_tree;
/// Part of the history widget that shows changes for a particular revision
pub(crate) mod revision_changes_table;
pub(crate) mod revisions_table;
pub(crate) mod table_headers;
pub(crate) mod versionpin_changes_row;
pub(crate) mod versionpin_changes_table;
pub(crate) mod versionpin_row;
pub(crate) mod versionpin_table;
pub(crate) mod versionpin_table_splitter;
pub(crate) mod withs_splitter;
