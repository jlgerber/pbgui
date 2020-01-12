//! The main ui is broken up into major components, which
//! may be found in the `components` module, here.

/// The bottom_context_widget provides the bottom stacked widget with a per
/// stack set of controls that are hosted on the toolbar between the versionpin
/// table and the revision view below it.
pub mod bottom_context_widget;
/// The bottom stacked widget housese the change table , the hstory table
pub mod bottom_stacked_widget;
/// Part of the history widget that shows changes for a particular revision
pub mod revision_changes_table;
/// The top submodule
pub mod top;
pub use top::main_menu_bar;
pub use top::package_lineedit;
pub use top::query_button;
pub use top::search_comboboxes;
pub use top::toolbar as top_toolbar;
pub mod center_widget;
pub mod left_toolbar;
pub mod packages_tree;
pub mod revisions_table;
pub mod table_headers;
pub mod versionpin_changes_row;
pub mod versionpin_changes_table;
pub mod versionpin_row;
pub mod versionpin_table;
pub mod versionpin_table_splitter;
pub mod withpackage_widget;
pub mod withs_splitter;
