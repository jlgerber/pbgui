//! The main ui is broken up into major components, which
//! may be found in the `components` module, here.

/// The bottom_context_widget provides the bottom stacked widget with a per
/// stack set of controls that are hosted on the toolbar between the versionpin
/// table and the revision view below it.
pub(crate) mod bottom_context_widget;
/// The bottom stacked widget houses the change table, the history table
pub(crate) mod bottom_stacked_widget;
/// Houses the create function which is responsible for creating and configuring the center widget,
pub(crate) mod center_widget;
/// Houses the DistributionTreeView component, which models the a tree
/// of packages, distributions per package, and platforms per distribution available for a
/// given site
pub(crate) mod dist_tree;
/// Creates the leftmost toolbar, and the LeftToolBarActions struct
pub(crate) mod left_toolbar;
/// Creates the WithsList struct, which provides the withs list widget
pub(crate) mod package_withs_list;
/// Creates the DistributionTreeView
pub(crate) mod packages_tree;
/// Part of the history widget that shows changes for a particular revision
pub(crate) mod revision_changes_table;
/// Creates the Revisions QTableWdiget
pub(crate) mod revisions_table;
/// Utility function to help set up table headers. Should be moved to utilities
pub(crate) mod table_headers;
/// Models a row of versionpin change data. Should eb moved up
pub(crate) mod versionpin_changes_row;
/// Creates the table tracking staged versionpin changes
pub(crate) mod versionpin_changes_table;
/// Models a row of versionpin data, parameterized by the type of data returned. Should be moved up
pub(crate) mod versionpin_row;
/// Creates the main VersionPin Table
pub(crate) mod versionpin_table;
/// Creates the VersionPin table splitter which separates the VersionPin table, from the
/// stackwidget below
pub(crate) mod versionpin_table_splitter;
/// Creates a vertical splitter which separates the main versionpin / changes splitter from the withs widget
pub(crate) mod withs_splitter;
